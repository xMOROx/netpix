//! RTP Streams Plot Component
//!
//! Provides a visualization of RTP streams over time using SVG.

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

#[component]
pub fn RtpStreamsPlot(state: Signal<AppState>) -> Element {
    // Read update counter to trigger re-renders
    let _update = state.read().update_counter;
    let mut x_axis_mode = use_signal(|| XAxisMode::SecondsFromStart);
    let mut streams_visibility: Signal<HashMap<RtpStreamKey, bool>> = use_signal(HashMap::new);
    
    let streams_data = state.read().streams.clone();
    let streams = streams_data.borrow();
    let rtp_streams = &streams.rtp_streams;
    
    // Get list of streams
    let stream_list: Vec<_> = rtp_streams.iter().collect();
    
    // Calculate plot dimensions and data
    let plot_width = 800.0_f64;
    let plot_height = 400.0_f64;
    let margin = 60.0_f64;
    
    // Build plot data
    let mut all_points: Vec<(f64, f64, String, String)> = Vec::new(); // (x, y, color, hover)
    let mut y_offset = 0.0_f64;
    let y_stream_gap = 50.0_f64;
    
    for (key, stream) in stream_list.iter() {
        // Check visibility
        let is_visible = streams_visibility.read().get(key).copied().unwrap_or(true);
        if !is_visible {
            continue;
        }
        
        let rtp_packets = &stream.rtp_packets;
        if rtp_packets.is_empty() {
            continue;
        }
        
        let first_packet = rtp_packets.first();
        let first_time = first_packet.map(|p| p.time).unwrap_or_default();
        let first_seq = first_packet.map(|p| p.packet.sequence_number as f64).unwrap_or(0.0);
        let first_ts = first_packet.map(|p| p.packet.timestamp as f64).unwrap_or(0.0);
        
        for packet in rtp_packets.iter() {
            let x = match *x_axis_mode.read() {
                XAxisMode::RtpTimestamp => packet.packet.timestamp as f64 - first_ts,
                XAxisMode::SecondsFromStart => (packet.time - first_time).as_secs_f64(),
                XAxisMode::SequenceNumber => packet.packet.sequence_number as f64 - first_seq,
            };
            
            // Y is based on payload size, stacked per stream
            let payload_height = (packet.packet.payload_length as f64 / 10.0).min(40.0);
            let y = y_offset + payload_height;
            
            // Determine color based on packet state
            let color = if packet.prev_lost {
                "#FFD700" // Gold - previous packet lost
            } else if packet.packet.marker {
                "#00FF00" // Green - marker bit set
            } else {
                "#FF4444" // Red - normal packet
            };
            
            let hover = format!(
                "Stream: {} (SSRC: {:x})\nSeq: {}\nTimestamp: {}\nPayload: {} bytes{}{}",
                stream.alias,
                stream.ssrc,
                packet.packet.sequence_number,
                packet.packet.timestamp,
                packet.packet.payload_length,
                if packet.packet.marker { "\n[MARKER]" } else { "" },
                if packet.prev_lost { "\n[PREV LOST]" } else { "" }
            );
            
            all_points.push((x, y, color.to_string(), hover));
        }
        
        y_offset += y_stream_gap;
    }
    
    // Normalize coordinates for SVG
    let x_min = all_points.iter().map(|(x, _, _, _)| *x).fold(f64::INFINITY, f64::min);
    let x_max = all_points.iter().map(|(x, _, _, _)| *x).fold(f64::NEG_INFINITY, f64::max);
    let y_max = all_points.iter().map(|(_, y, _, _)| *y).fold(0.0_f64, f64::max);
    
    let x_range = if (x_max - x_min).abs() < 0.001 { 1.0 } else { x_max - x_min };
    let y_range = if y_max < 0.001 { 1.0 } else { y_max };
    
    // Precompute SVG dimensions
    let svg_width = plot_width + margin * 2.0;
    let svg_height = plot_height + margin * 2.0;
    let x_label_x = margin + plot_width / 2.0;
    let x_label_y = margin + plot_height + 40.0;
    let y_label_y = margin + plot_height / 2.0;
    let y_axis_end = margin + plot_height;
    let x_axis_end = margin + plot_width;
    
    rsx! {
        div {
            style: "width: 100%; height: 100%; display: flex; flex-direction: column; padding: 16px; background: #1e1e1e; color: #ddd;",
            
            // Header with controls
            div {
                style: "margin-bottom: 16px;",
                
                // X-axis mode selector
                div {
                    style: "display: flex; align-items: center; gap: 16px; margin-bottom: 12px;",
                    span { style: "font-weight: bold;", "X Axis:" }
                    
                    label {
                        style: "display: flex; align-items: center; gap: 4px; cursor: pointer;",
                        input {
                            r#type: "radio",
                            name: "x_axis",
                            checked: *x_axis_mode.read() == XAxisMode::SecondsFromStart,
                            onchange: move |_| x_axis_mode.set(XAxisMode::SecondsFromStart),
                        }
                        "Seconds from Start"
                    }
                    
                    label {
                        style: "display: flex; align-items: center; gap: 4px; cursor: pointer;",
                        input {
                            r#type: "radio",
                            name: "x_axis",
                            checked: *x_axis_mode.read() == XAxisMode::RtpTimestamp,
                            onchange: move |_| x_axis_mode.set(XAxisMode::RtpTimestamp),
                        }
                        "RTP Timestamp"
                    }
                    
                    label {
                        style: "display: flex; align-items: center; gap: 4px; cursor: pointer;",
                        input {
                            r#type: "radio",
                            name: "x_axis",
                            checked: *x_axis_mode.read() == XAxisMode::SequenceNumber,
                            onchange: move |_| x_axis_mode.set(XAxisMode::SequenceNumber),
                        }
                        "Sequence Number"
                    }
                }
                
                // Stream toggles
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 12px; align-items: center;",
                    span { style: "font-weight: bold;", "Streams:" }
                    
                    for (key, stream) in stream_list.iter() {
                        {
                            let key = **key;
                            let alias = stream.alias.clone();
                            let is_visible = streams_visibility.read().get(&key).copied().unwrap_or(true);
                            
                            rsx! {
                                label {
                                    style: "display: flex; align-items: center; gap: 4px; cursor: pointer; padding: 4px 8px; background: #333; border-radius: 4px;",
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
                style: "display: flex; gap: 20px; margin-bottom: 12px; padding: 8px; background: #252525; border-radius: 4px;",
                
                div {
                    style: "display: flex; align-items: center; gap: 6px;",
                    div { style: "width: 12px; height: 12px; background: #FF4444; border-radius: 50%;" }
                    span { "Normal Packet" }
                }
                div {
                    style: "display: flex; align-items: center; gap: 6px;",
                    div { style: "width: 12px; height: 12px; background: #00FF00; border-radius: 50%;" }
                    span { "Marker Bit Set" }
                }
                div {
                    style: "display: flex; align-items: center; gap: 6px;",
                    div { style: "width: 12px; height: 12px; background: #FFD700; border-radius: 50%;" }
                    span { "Previous Packet Lost" }
                }
            }
            
            // Plot area
            div {
                style: "flex: 1; overflow: auto; background: #252525; border-radius: 8px; padding: 16px;",
                
                if all_points.is_empty() {
                    div {
                        style: "display: flex; align-items: center; justify-content: center; height: 100%; color: #888;",
                        "No RTP packets to display"
                    }
                } else {
                    svg {
                        width: "{svg_width}",
                        height: "{svg_height}",
                        style: "background: #1a1a1a; border-radius: 4px;",
                        
                        // Grid lines - horizontal
                        {
                            let grid_y: Vec<f64> = (0..=4).map(|i| margin + (i as f64 * plot_height / 4.0)).collect();
                            rsx! {
                                for y in grid_y.iter() {
                                    line {
                                        x1: "{margin}",
                                        y1: "{y}",
                                        x2: "{x_axis_end}",
                                        y2: "{y}",
                                        stroke: "#333",
                                        stroke_width: "1",
                                    }
                                }
                            }
                        }
                        
                        // Grid lines - vertical
                        {
                            let grid_x: Vec<f64> = (0..=4).map(|i| margin + (i as f64 * plot_width / 4.0)).collect();
                            rsx! {
                                for x in grid_x.iter() {
                                    line {
                                        x1: "{x}",
                                        y1: "{margin}",
                                        x2: "{x}",
                                        y2: "{y_axis_end}",
                                        stroke: "#333",
                                        stroke_width: "1",
                                    }
                                }
                            }
                        }
                        
                        // Axes
                        line {
                            x1: "{margin}",
                            y1: "{y_axis_end}",
                            x2: "{x_axis_end}",
                            y2: "{y_axis_end}",
                            stroke: "#666",
                            stroke_width: "2",
                        }
                        line {
                            x1: "{margin}",
                            y1: "{margin}",
                            x2: "{margin}",
                            y2: "{y_axis_end}",
                            stroke: "#666",
                            stroke_width: "2",
                        }
                        
                        // Data points
                        for (x, y, color, hover) in all_points.iter() {
                            {
                                let px = margin + ((*x - x_min) / x_range) * plot_width;
                                let py = margin + plot_height - ((*y) / y_range) * plot_height;
                                rsx! {
                                    circle {
                                        cx: "{px}",
                                        cy: "{py}",
                                        r: "3",
                                        fill: "{color}",
                                        
                                        title {
                                            "{hover}"
                                        }
                                    }
                                }
                            }
                        }
                        
                        // X-axis label
                        text {
                            x: "{x_label_x}",
                            y: "{x_label_y}",
                            fill: "#888",
                            text_anchor: "middle",
                            font_size: "12",
                            "{x_axis_mode.read().label()}"
                        }
                        
                        // Y-axis label
                        text {
                            x: "20",
                            y: "{y_label_y}",
                            fill: "#888",
                            text_anchor: "middle",
                            font_size: "12",
                            transform: "rotate(-90, 20, {y_label_y})",
                            "Payload Size"
                        }
                    }
                }
            }
            
            // Stats footer
            div {
                style: "margin-top: 12px; padding: 8px; background: #252525; border-radius: 4px; font-size: 12px; color: #888;",
                "Total RTP streams: {rtp_streams.len()} | Data points: {all_points.len()}"
            }
        }
    }
}
