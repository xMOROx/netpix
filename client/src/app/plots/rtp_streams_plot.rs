//! RTP Streams Plot Component
//!
//! Provides an interactive visualization of RTP streams over time using SVG.
//! Features: zoom, pan, responsive sizing, tooltips, stream visibility toggles.

use dioxus::prelude::*;
use crate::app::AppState;
use netpix_common::RtpStreamKey;
use std::collections::HashMap;

/// X-axis display mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum XAxisMode {
    RtpTimestamp,
    SecondsFromStart,
    SequenceNumber,
}

impl XAxisMode {
    fn label(&self) -> &'static str {
        match self {
            XAxisMode::RtpTimestamp => "RTP Timestamp",
            XAxisMode::SecondsFromStart => "Seconds from Start",
            XAxisMode::SequenceNumber => "Sequence Number",
        }
    }
}

/// Point data for rendering - with precomputed SVG coordinates
#[derive(Clone)]
struct PlotPoint {
    px: f64,
    py_low: f64,
    py_top: f64,
    color: String,
    hover: String,
}

/// Stream separator line with precomputed Y
#[derive(Clone)]
struct StreamLine {
    py: f64,
    alias: String,
    in_view: bool,
}

/// Grid line data
#[derive(Clone)]
struct GridLine {
    pos: f64,
    label: String,
}

#[component]
pub fn RtpStreamsPlot(state: Signal<AppState>) -> Element {
    // Read update counter to trigger re-renders
    let _update = state.read().update_counter;
    
    // Plot state
    let mut x_axis_mode = use_signal(|| XAxisMode::SecondsFromStart);
    let mut streams_visibility: Signal<HashMap<RtpStreamKey, bool>> = use_signal(HashMap::new);
    
    // Zoom state
    let mut zoom_level = use_signal(|| 1.0_f64);
    
    // View bounds controls
    let mut x_start = use_signal(|| 0.0_f64);
    let mut x_length = use_signal(|| 100.0_f64);
    let mut use_custom_bounds = use_signal(|| false);
    
    let streams_data = state.read().streams.clone();
    let streams = streams_data.borrow();
    let rtp_streams = &streams.rtp_streams;
    
    // Get list of streams sorted by alias
    let mut stream_list: Vec<_> = rtp_streams.iter().collect();
    stream_list.sort_by(|a, b| a.1.alias.cmp(&b.1.alias));
    
    // Build raw data first
    let mut raw_points: Vec<(f64, f64, f64, String, String)> = Vec::new(); // x, y_low, y_top, color, hover
    let mut raw_lines: Vec<(f64, String)> = Vec::new(); // y, alias
    let mut y_offset = 0.0_f64;
    let y_stream_gap = 100.0_f64;
    let min_height = 2.0_f64;
    
    let mut data_x_min = f64::INFINITY;
    let mut data_x_max = f64::NEG_INFINITY;
    let mut data_y_max = 0.0_f64;
    
    for (key, stream) in stream_list.iter() {
        let is_visible = streams_visibility.read().get(key).copied().unwrap_or(true);
        if !is_visible {
            continue;
        }
        
        let rtp_packets = &stream.rtp_packets;
        if rtp_packets.is_empty() {
            continue;
        }
        
        raw_lines.push((y_offset, format!("{} (SSRC: {:x})", stream.alias, stream.ssrc)));
        
        let first_packet = rtp_packets.first();
        let first_time = first_packet.map(|p| p.time).unwrap_or_default();
        let first_seq = first_packet.map(|p| p.packet.sequence_number as f64).unwrap_or(0.0);
        let first_ts = first_packet.map(|p| p.packet.timestamp as f64).unwrap_or(0.0);
        
        let mut stream_max_y = y_offset;
        
        for packet in rtp_packets.iter() {
            let x = match *x_axis_mode.read() {
                XAxisMode::RtpTimestamp => packet.packet.timestamp as f64 - first_ts,
                XAxisMode::SecondsFromStart => (packet.time - first_time).as_secs_f64(),
                XAxisMode::SequenceNumber => packet.packet.sequence_number as f64 - first_seq,
            };
            
            if x < data_x_min { data_x_min = x; }
            if x > data_x_max { data_x_max = x; }
            
            let payload_height = (packet.packet.payload_length as f64 * 0.02).max(min_height);
            let y_low = y_offset;
            let y_top = y_offset + payload_height;
            
            if y_top > stream_max_y { stream_max_y = y_top; }
            if y_top > data_y_max { data_y_max = y_top; }
            
            let color = if packet.prev_lost {
                "#FFD700".to_string()
            } else if packet.packet.marker {
                "#00FF00".to_string()
            } else {
                "#FF4444".to_string()
            };
            
            let hover = format!(
                "Stream: {} | Seq: {} | TS: {} | {} bytes{}{}",
                stream.alias,
                packet.packet.sequence_number,
                packet.packet.timestamp,
                packet.packet.payload_length,
                if packet.packet.marker { " [M]" } else { "" },
                if packet.prev_lost { " [LOST]" } else { "" }
            );
            
            raw_points.push((x, y_low, y_top, color, hover));
        }
        
        y_offset = stream_max_y + y_stream_gap;
    }
    
    // Handle empty state
    if data_x_min == f64::INFINITY { data_x_min = 0.0; }
    if data_x_max == f64::NEG_INFINITY { data_x_max = 1.0; }
    if data_y_max < 0.001 { data_y_max = 100.0; }
    
    // SVG dimensions
    let zoom = *zoom_level.read();
    let svg_width = (1200.0 * zoom).max(800.0);
    let svg_height = (600.0_f64.max(data_y_max * 2.0) * zoom).max(400.0);
    let margin = 80.0_f64;
    let plot_width = svg_width - margin * 2.0;
    let plot_height = svg_height - margin * 2.0;
    
    // View bounds
    let view_x_min = if *use_custom_bounds.read() {
        *x_start.read()
    } else {
        data_x_min - (data_x_max - data_x_min) * 0.05
    };
    let view_x_max = if *use_custom_bounds.read() {
        *x_start.read() + *x_length.read()
    } else {
        data_x_max + (data_x_max - data_x_min) * 0.05
    };
    
    let view_y_min = 0.0;
    let view_y_max = data_y_max * 1.1;
    
    let x_range = (view_x_max - view_x_min).max(0.001);
    let y_range = (view_y_max - view_y_min).max(0.001);
    
    // Precompute SVG coordinates for all points
    let plot_points: Vec<PlotPoint> = raw_points
        .iter()
        .filter(|(x, _, _, _, _)| *x >= view_x_min && *x <= view_x_max)
        .map(|(x, y_low, y_top, color, hover)| {
            let px = margin + ((x - view_x_min) / x_range) * plot_width;
            let py_low = margin + plot_height - ((y_low - view_y_min) / y_range) * plot_height;
            let py_top = margin + plot_height - ((y_top - view_y_min) / y_range) * plot_height;
            PlotPoint {
                px,
                py_low: py_low.clamp(margin, margin + plot_height),
                py_top: py_top.clamp(margin, margin + plot_height),
                color: color.clone(),
                hover: hover.clone(),
            }
        })
        .collect();
    
    // Precompute stream lines
    let stream_lines: Vec<StreamLine> = raw_lines
        .iter()
        .map(|(y, alias)| {
            let py = margin + plot_height - ((y - view_y_min) / y_range) * plot_height;
            StreamLine {
                py,
                alias: alias.clone(),
                in_view: py >= margin && py <= margin + plot_height,
            }
        })
        .collect();
    
    // Precompute grid lines
    let h_grid: Vec<GridLine> = (0..=5)
        .map(|i| {
            let pos = margin + (i as f64 * plot_height / 5.0);
            let value = view_y_max - (i as f64 / 5.0) * y_range;
            GridLine { pos, label: format!("{:.0}", value) }
        })
        .collect();
    
    let format_x_val = |v: f64| -> String {
        match *x_axis_mode.read() {
            XAxisMode::SecondsFromStart => format!("{:.2}s", v),
            _ => format!("{:.0}", v),
        }
    };
    
    let v_grid: Vec<GridLine> = (0..=5)
        .map(|i| {
            let pos = margin + (i as f64 * plot_width / 5.0);
            let value = view_x_min + (i as f64 / 5.0) * x_range;
            GridLine { pos, label: format_x_val(value) }
        })
        .collect();
    
    let slider_max = (data_x_max * 1.2).max(100.0) as i64;
    let x_label_text = x_axis_mode.read().label().to_string();
    let data_x_min_str = format_x_val(data_x_min);
    let data_x_max_str = format_x_val(data_x_max);
    let x_start_str = format_x_val(*x_start.read());
    let x_length_str = format_x_val(*x_length.read());
    let stream_count = rtp_streams.len();
    let point_count = plot_points.len();
    let zoom_str = format!("{:.1}x", zoom);
    
    rsx! {
        div {
            style: "width: 100%; height: 100%; display: flex; flex-direction: column; background: #1e1e1e; color: #ddd; overflow: hidden;",
            
            // Controls
            div {
                style: "padding: 12px 16px; background: #252525; border-bottom: 1px solid #333;",
                
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 16px; align-items: center;",
                    
                    // X-axis selector
                    span { style: "font-weight: bold;", "X:" }
                    select {
                        style: "padding: 4px 8px; background: #333; color: white; border: 1px solid #555; border-radius: 4px;",
                        value: "{x_axis_mode.read().label()}",
                        onchange: move |evt: Event<FormData>| {
                            let mode = match evt.value().as_str() {
                                "Seconds from Start" => XAxisMode::SecondsFromStart,
                                "RTP Timestamp" => XAxisMode::RtpTimestamp,
                                _ => XAxisMode::SequenceNumber,
                            };
                            x_axis_mode.set(mode);
                            use_custom_bounds.set(false);
                        },
                        option { value: "Seconds from Start", "Seconds from Start" }
                        option { value: "RTP Timestamp", "RTP Timestamp" }
                        option { value: "Sequence Number", "Sequence Number" }
                    }
                    
                    // Zoom
                    span { style: "font-weight: bold;", "Zoom:" }
                    button {
                        style: "padding: 4px 8px; background: #444; border: none; color: white; border-radius: 4px; cursor: pointer;",
                        onclick: move |_| zoom_level.set((zoom * 1.5).min(10.0)),
                        "+"
                    }
                    button {
                        style: "padding: 4px 8px; background: #444; border: none; color: white; border-radius: 4px; cursor: pointer;",
                        onclick: move |_| zoom_level.set((zoom / 1.5).max(0.5)),
                        "-"
                    }
                    button {
                        style: "padding: 4px 8px; background: #555; border: none; color: white; border-radius: 4px; cursor: pointer;",
                        onclick: move |_| {
                            zoom_level.set(1.0);
                            use_custom_bounds.set(false);
                        },
                        "Reset"
                    }
                    span { style: "color: #888; font-size: 12px;", "{zoom_str}" }
                    
                    // Bounds
                    label {
                        style: "display: flex; align-items: center; gap: 4px;",
                        input {
                            r#type: "checkbox",
                            checked: *use_custom_bounds.read(),
                            onchange: move |evt: Event<FormData>| use_custom_bounds.set(evt.checked()),
                        }
                        "Custom X"
                    }
                    
                    if *use_custom_bounds.read() {
                        span { "Start:" }
                        input {
                            r#type: "range",
                            min: "0",
                            max: "{slider_max}",
                            value: "{*x_start.read() as i64}",
                            style: "width: 100px;",
                            oninput: move |evt: Event<FormData>| {
                                if let Ok(v) = evt.value().parse::<f64>() {
                                    x_start.set(v);
                                }
                            },
                        }
                        span { style: "font-size: 11px; min-width: 50px;", "{x_start_str}" }
                        
                        span { "Len:" }
                        input {
                            r#type: "range",
                            min: "1",
                            max: "{slider_max}",
                            value: "{*x_length.read() as i64}",
                            style: "width: 100px;",
                            oninput: move |evt: Event<FormData>| {
                                if let Ok(v) = evt.value().parse::<f64>() {
                                    x_length.set(v.max(1.0));
                                }
                            },
                        }
                        span { style: "font-size: 11px; min-width: 50px;", "{x_length_str}" }
                    }
                }
                
                // Stream toggles
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 8px; margin-top: 8px; align-items: center;",
                    span { style: "font-weight: bold;", "Streams:" }
                    
                    for (key, stream) in stream_list.iter() {
                        {
                            let key = **key;
                            let alias = stream.alias.clone();
                            let is_visible = streams_visibility.read().get(&key).copied().unwrap_or(true);
                            let bg = if is_visible { "#3a5a3a" } else { "#444" };
                            
                            rsx! {
                                label {
                                    style: "display: flex; align-items: center; gap: 4px; cursor: pointer; padding: 2px 6px; background: {bg}; border-radius: 4px; font-size: 11px;",
                                    input {
                                        r#type: "checkbox",
                                        checked: is_visible,
                                        onchange: move |_| {
                                            let mut vis = streams_visibility.write();
                                            let current = vis.get(&key).copied().unwrap_or(true);
                                            vis.insert(key, !current);
                                        },
                                    }
                                    "{alias}"
                                }
                            }
                        }
                    }
                }
            }
            
            // Legend
            div {
                style: "display: flex; gap: 16px; padding: 6px 16px; background: #2a2a2a; border-bottom: 1px solid #333; font-size: 11px;",
                
                div {
                    style: "display: flex; align-items: center; gap: 4px;",
                    div { style: "width: 8px; height: 8px; background: #FF4444; border-radius: 50%;" }
                    "Normal"
                }
                div {
                    style: "display: flex; align-items: center; gap: 4px;",
                    div { style: "width: 8px; height: 8px; background: #00FF00; border-radius: 50%;" }
                    "Marker"
                }
                div {
                    style: "display: flex; align-items: center; gap: 4px;",
                    div { style: "width: 8px; height: 8px; background: #FFD700; border-radius: 50%;" }
                    "Lost"
                }
            }
            
            // Plot area with mouse wheel zoom
            div {
                style: "flex: 1; overflow: auto; background: #1a1a1a;",
                onwheel: move |evt: WheelEvent| {
                    let delta = evt.delta().strip_units().y;
                    if delta < 0.0 {
                        // Scroll up = zoom in
                        zoom_level.set((zoom * 1.2).min(10.0));
                    } else if delta > 0.0 {
                        // Scroll down = zoom out
                        zoom_level.set((zoom / 1.2).max(0.5));
                    }
                },
                
                if plot_points.is_empty() {
                    div {
                        style: "display: flex; align-items: center; justify-content: center; height: 100%; color: #888;",
                        "No RTP packets to display"
                    }
                } else {
                    svg {
                        width: "{svg_width}",
                        height: "{svg_height}",
                        style: "min-width: 100%; min-height: 100%;",
                        
                        // Background
                        rect { x: "0", y: "0", width: "{svg_width}", height: "{svg_height}", fill: "#1a1a1a" }
                        rect { x: "{margin}", y: "{margin}", width: "{plot_width}", height: "{plot_height}", fill: "#222", stroke: "#444" }
                        
                        // Horizontal grid
                        for gl in h_grid.iter() {
                            line { x1: "{margin}", y1: "{gl.pos}", x2: "{margin + plot_width}", y2: "{gl.pos}", stroke: "#333" }
                            text { x: "{margin - 5.0}", y: "{gl.pos + 4.0}", fill: "#666", text_anchor: "end", font_size: "10", "{gl.label}" }
                        }
                        
                        // Vertical grid
                        for gl in v_grid.iter() {
                            line { x1: "{gl.pos}", y1: "{margin}", x2: "{gl.pos}", y2: "{margin + plot_height}", stroke: "#333" }
                            text { x: "{gl.pos}", y: "{margin + plot_height + 15.0}", fill: "#666", text_anchor: "middle", font_size: "10", "{gl.label}" }
                        }
                        
                        // Stream separators
                        for sl in stream_lines.iter().filter(|sl| sl.in_view) {
                            line { x1: "{margin}", y1: "{sl.py}", x2: "{margin + plot_width}", y2: "{sl.py}", stroke: "#555", stroke_dasharray: "5,5" }
                            text { x: "{margin + 5.0}", y: "{sl.py - 5.0}", fill: "#aaa", font_size: "11", font_weight: "bold", "{sl.alias}" }
                        }
                        
                        // Data points
                        for pt in plot_points.iter() {
                            line { x1: "{pt.px}", y1: "{pt.py_low}", x2: "{pt.px}", y2: "{pt.py_top}", stroke: "{pt.color}" }
                            circle {
                                cx: "{pt.px}",
                                cy: "{pt.py_top}",
                                r: "3",
                                fill: "{pt.color}",
                                style: "cursor: pointer;",
                                title { "{pt.hover}" }
                            }
                        }
                        
                        // Axis labels
                        text { x: "{margin + plot_width / 2.0}", y: "{margin + plot_height + 35.0}", fill: "#999", text_anchor: "middle", font_size: "12", "{x_label_text}" }
                        text { x: "20", y: "{margin + plot_height / 2.0}", fill: "#999", text_anchor: "middle", font_size: "12", transform: "rotate(-90, 20, {margin + plot_height / 2.0})", "Payload Size" }
                    }
                }
            }
            
            // Footer
            div {
                style: "padding: 6px 16px; background: #252525; border-top: 1px solid #333; font-size: 11px; color: #888; display: flex; gap: 16px;",
                span { "Streams: {stream_count}" }
                span { "Points: {point_count}" }
                span { "X: {data_x_min_str} → {data_x_max_str}" }
            }
        }
    }
}
