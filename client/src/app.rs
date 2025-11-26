use dioxus::prelude::*;
use log::{error, info, warn};
use netpix_common::{Request, Response, Source};
use std::rc::Rc;
use web_sys::{BinaryType, MessageEvent, WebSocket};
use wasm_bindgen::{closure::Closure, JsCast};

use crate::streams::RefStreams;

mod tab;
mod websocket;
mod tables;
mod components;
mod plots;

use tab::Tab;
use websocket::WebSocketManager;
use tables::{PacketsTable, RtpPacketsTable, RtpStreamsTable, RtcpPacketsTable, StunPacketsTable,
             MpegtsPacketsTable, MpegtsStreamsTable, MpegtsInfoTable};
use plots::RtpStreamsPlot;
pub use components::{FilterInput, build_filter_help};

// App state
#[derive(Clone)]
pub(crate) struct AppState {
    pub is_capturing: bool,
    pub streams: RefStreams,
    pub sources: Vec<Source>,
    pub selected_source: Option<Source>,
    pub current_tab: Tab,
    pub discharged_count: usize,
    pub overwritten_count: usize,
    pub update_counter: usize, // Force re-renders when data changes
    pub pending_source_change: Option<Source>, // Track source changes
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            is_capturing: true,
            streams: RefStreams::default(),
            sources: Vec::new(),
            selected_source: None,
            current_tab: Tab::Packets,
            discharged_count: 0,
            overwritten_count: 0,
            update_counter: 0,
            pending_source_change: None,
        }
    }
}

#[component]
pub fn App() -> Element {
    let mut state = use_signal(AppState::default);
    let ws_manager = use_signal(WebSocketManager::new);

    // Initialize WebSocket connection
    use_effect(move || {
        let host = web_sys::window()
            .and_then(|w| w.location().host().ok())
            .unwrap_or_else(|| "localhost:5566".to_string());
        
        let ws_url = format!("ws://{}/ws", host);
        info!("Connecting to WebSocket at {}", ws_url);
        
        if let Err(e) = ws_manager.read().connect(&ws_url) {
            error!("Failed to connect to WebSocket: {:?}", e);
        }
    });

    // Handle incoming WebSocket messages with polling
    use_effect(move || {
        spawn(async move {
            loop {
                // Check for pending source change
                let pending_source = {
                    let mut s = state.write();
                    s.pending_source_change.take()
                };
                
                if let Some(source) = pending_source {
                    info!("Sending ChangeSource request for: {}", source);
                    let request = Request::ChangeSource(source.clone());
                    if let Err(e) = ws_manager.read().send_request(&request) {
                        error!("Failed to send ChangeSource request: {:?}", e);
                    }
                }
                
                // Drain all pending messages
                let messages = ws_manager.read().drain_messages();
                for msg in messages {
                    handle_message(msg, &mut state, &ws_manager);
                }
                // Poll every 50ms
                gloo_timers::future::TimeoutFuture::new(50).await;
            }
        });
    });

    rsx! {
        div {
            class: "app-container",
            style: "width: 100vw; height: 100vh; display: flex; flex-direction: column;",
            
            // Top bar
            TopBar { state: state }
            
            // Main content area
            div {
                class: "main-content",
                style: "flex: 1; display: flex; overflow: hidden;",
                
                // Side panel
                SidePanel { state: state }
                
                // Content panel
                ContentPanel { state: state }
            }
            
            // Bottom bar
            BottomBar { state: state }
        }
    }
}

#[component]
fn TopBar(mut state: Signal<AppState>) -> Element {
    let current_tab = state.read().current_tab;
    let tab_value = current_tab.value();
    let selected_source = state.read().selected_source.clone();
    let source_value = selected_source.as_ref().map(|s| s.to_string()).unwrap_or_default();
    
    rsx! {
        div {
            class: "top-bar",
            style: "background: #2c2c2c; padding: 10px; display: flex; align-items: center; gap: 10px; border-bottom: 1px solid #444;",
            
            // Source selector
            select {
                style: "padding: 5px; background: #1e1e1e; color: #ddd; border: 1px solid #555; border-radius: 4px; min-width: 200px;",
                value: "{source_value}",
                onchange: move |evt| {
                    let value = evt.value();
                    info!("Source selection changed to: {}", value);
                    
                    // Find the source that matches this string
                    let sources = state.read().sources.clone();
                    if let Some(source) = sources.into_iter().find(|s| s.to_string() == value) {
                        info!("Found matching source: {:?}", source);
                        let mut s = state.write();
                        // Clear existing data when source changes
                        s.streams.borrow_mut().clear();
                        s.selected_source = Some(source.clone());
                        s.pending_source_change = Some(source);
                        s.update_counter += 1;
                    }
                },
                option { 
                    value: "",
                    disabled: true,
                    selected: selected_source.is_none(),
                    "Select Source..." 
                }
                for source in state.read().sources.iter() {
                    option { 
                        value: "{source}",
                        selected: selected_source.as_ref() == Some(source),
                        "{source}"
                    }
                }
            }
            
            div { style: "width: 1px; height: 24px; background: #555;" }
            
            // Tab selector
            span { 
                style: "color: #ddd; font-size: 14px;",
                "📑 "
            }
            
            select {
                style: "padding: 5px; background: #1e1e1e; color: #ddd; border: 1px solid #555; border-radius: 4px;",
                value: "{tab_value}",
                onchange: move |evt| {
                    let value = evt.value();
                    let new_tab = match value.as_str() {
                        "Packets" => Tab::Packets,
                        "RtpPackets" => Tab::RtpSection(tab::RtpSection::Packets),
                        "RtcpPackets" => Tab::RtpSection(tab::RtpSection::RtcpPackets),
                        "RtpStreams" => Tab::RtpSection(tab::RtpSection::Streams),
                        "RtpPlot" => Tab::RtpSection(tab::RtpSection::Plot),
                        "MpegTsPackets" => Tab::MpegTsSection(tab::MpegTsSection::Packets),
                        "MpegTsStreams" => Tab::MpegTsSection(tab::MpegTsSection::Streams),
                        "MpegTsInfo" => Tab::MpegTsSection(tab::MpegTsSection::Information),
                        "StunPackets" => Tab::IceSection(tab::IceSection::StunPackets),
                        _ => Tab::Packets,
                    };
                    state.write().current_tab = new_tab;
                },
                
                optgroup { label: "📋 General",
                    option { value: "Packets", "📦 Packets" }
                }
                
                optgroup { label: "🔈 RTP",
                    option { value: "RtpPackets", "RTP Packets" }
                    option { value: "RtcpPackets", "RTCP Packets" }
                    option { value: "RtpStreams", "RTP Streams" }
                    option { value: "RtpPlot", "RTP Plot" }
                }
                
                optgroup { label: "📺 MPEG-TS",
                    option { value: "MpegTsPackets", "MPEG-TS Packets" }
                    option { value: "MpegTsStreams", "MPEG-TS Streams" }
                    option { value: "MpegTsInfo", "MPEG-TS Information" }
                }
                
                optgroup { label: "🗼 ICE",
                    option { value: "StunPackets", "STUN Packets" }
                }
            }
            
            span {
                style: "color: #ddd; margin-left: auto; font-size: 14px;",
                "{current_tab.display_name()}"
            }
        }
    }
}

#[component]
fn SidePanel(state: Signal<AppState>) -> Element {
    let is_capturing = state.read().is_capturing;
    
    rsx! {
        div {
            class: "side-panel",
            style: "width: 50px; background: #2c2c2c; display: flex; flex-direction: column; align-items: center; padding: 10px 0; gap: 10px; border-right: 1px solid #444;",
            
            // Play button
            button {
                style: "width: 40px; height: 40px; border: none; background: #1e1e1e; color: #4CAF50; font-size: 20px; border-radius: 4px; cursor: pointer;",
                disabled: is_capturing,
                onclick: move |_| {
                    state.write().is_capturing = true;
                },
                title: "Resume packet capturing",
                "▶"
            }
            
            // Pause button
            button {
                style: "width: 40px; height: 40px; border: none; background: #1e1e1e; color: #ff9800; font-size: 20px; border-radius: 4px; cursor: pointer;",
                disabled: !is_capturing,
                onclick: move |_| {
                    state.write().is_capturing = false;
                },
                title: "Stop packet capturing",
                "⏸"
            }
            
            // Clear button
            button {
                style: "width: 40px; height: 40px; border: none; background: #1e1e1e; color: #f44336; font-size: 20px; border-radius: 4px; cursor: pointer;",
                onclick: move |_| {
                    state.write().streams.borrow_mut().clear();
                },
                title: "Discard previously captured packets",
                "🗑"
            }
            
            // Refresh button
            button {
                style: "width: 40px; height: 40px; border: none; background: #1e1e1e; color: #2196F3; font-size: 20px; border-radius: 4px; cursor: pointer;",
                onclick: move |_| {
                    state.write().streams.borrow_mut().clear();
                    // TODO: Implement refetch_packets
                },
                title: "Refetch all previously captured packets",
                "↻"
            }
            
            // Spacer
            div { style: "flex: 1;" }
            
            // Theme toggle would go here
            div {
                style: "color: #888; font-size: 12px;",
                "🌙"
            }
        }
    }
}

#[component]
fn ContentPanel(state: Signal<AppState>) -> Element {
    let current_tab = state.read().current_tab;
    
    rsx! {
        div {
            class: "content-panel",
            style: "flex: 1; background: #1e1e1e; color: #ddd; overflow: hidden; display: flex; flex-direction: column;",
            
            // Header
            div {
                style: "padding: 20px; padding-bottom: 10px;",
                h2 { 
                    style: "margin: 0; color: #fff;",
                    "{current_tab.display_name()}" 
                }
            }
            
            // Content area - render appropriate component based on tab
            div {
                style: "flex: 1; overflow: hidden;",
                match current_tab {
                    Tab::Packets => rsx! {
                        PacketsTable { state: state }
                    },
                    Tab::RtpSection(section) => {
                        match section {
                            tab::RtpSection::Packets => rsx! {
                                RtpPacketsTable { state: state }
                            },
                            tab::RtpSection::RtcpPackets => rsx! {
                                RtcpPacketsTable { state: state }
                            },
                            tab::RtpSection::Streams => rsx! {
                                RtpStreamsTable { state: state }
                            },
                            tab::RtpSection::Plot => rsx! {
                                RtpStreamsPlot { state: state }
                            },
                        }
                    },
                    Tab::MpegTsSection(section) => {
                        match section {
                            tab::MpegTsSection::Packets => rsx! {
                                MpegtsPacketsTable { state: state }
                            },
                            tab::MpegTsSection::Streams => rsx! {
                                MpegtsStreamsTable { state: state }
                            },
                            tab::MpegTsSection::Information => rsx! {
                                MpegtsInfoTable { state: state }
                            },
                        }
                    },
                    Tab::IceSection(section) => {
                        match section {
                            tab::IceSection::StunPackets => rsx! {
                                StunPacketsTable { state: state }
                            },
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn BottomBar(state: Signal<AppState>) -> Element {
    let discharged = state.read().discharged_count;
    let overwritten = state.read().overwritten_count;
    
    rsx! {
        div {
            class: "bottom-bar",
            style: "background: #2c2c2c; padding: 8px 20px; display: flex; gap: 20px; border-top: 1px solid #444; font-size: 12px; color: #888;",
            
            span { 
                style: "color: #ddd;",
                "Discharged: {discharged}" 
            }
            span { 
                style: "color: #ddd;",
                "Overwritten: {overwritten}" 
            }
        }
    }
}

fn handle_message(msg: Vec<u8>, state: &mut Signal<AppState>, _ws_manager: &Signal<WebSocketManager>) {
    let Ok(response) = Response::decode(&msg) else {
        error!("Failed to decode response message");
        return;
    };

    info!("Received response: {:?}", std::mem::discriminant(&response.0));

    match response {
        (Response::Packet(packet), _) => {
            // Check if capturing is enabled
            if !state.read().is_capturing {
                return;
            }
            info!("Adding packet to streams");
            {
                let state_ref = state.read();
                let mut streams = state_ref.streams.borrow_mut();
                streams.add_packet(packet);
            }
            // Increment counter to trigger re-render
            state.write().update_counter += 1;
        }
        (Response::Sources(sources), _) => {
            info!("Received {} sources", sources.len());
            let mut s = state.write();
            
            // Check if current selected source is still valid
            if let Some(ref source) = s.selected_source {
                if !sources.contains(source) {
                    s.selected_source = None;
                }
            }
            
            // Auto-select the first source if none is selected and sources are available
            if s.selected_source.is_none() && !sources.is_empty() {
                let first_source = sources[0].clone();
                info!("Auto-selecting first source: {}", first_source);
                s.selected_source = Some(first_source.clone());
                s.pending_source_change = Some(first_source);
            }
            
            s.sources = sources;
        }
        (Response::Sdp(stream_key, sdp), _) => {
            info!("Received SDP for stream: {:?}", stream_key);
            {
                let state_ref = state.read();
                let mut streams = state_ref.streams.borrow_mut();
                if let Some(stream) = streams.rtp_streams.get_mut(&stream_key) {
                    stream.add_sdp(sdp);
                }
            }
            // Increment counter to trigger re-render
            state.write().update_counter += 1;
        }
        (Response::PacketsStats(stats), _) => {
            let mut s = state.write();
            s.discharged_count = stats.discharged;
            s.overwritten_count = stats.overwritten;
        }
    }
}
