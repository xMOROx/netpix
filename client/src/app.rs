use dioxus::prelude::*;
use log::{error, warn};
use netpix_common::{Request, Response, Source};
use std::rc::Rc;
use web_sys::{BinaryType, MessageEvent, WebSocket};
use wasm_bindgen::{closure::Closure, JsCast};

use crate::streams::RefStreams;

mod tab;
mod websocket;
use tab::Tab;
use websocket::WebSocketManager;

// App state
#[derive(Clone)]
struct AppState {
    is_capturing: bool,
    streams: RefStreams,
    sources: Vec<Source>,
    selected_source: Option<Source>,
    current_tab: Tab,
    discharged_count: usize,
    overwritten_count: usize,
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
        }
    }
}

#[component]
pub fn App() -> Element {
    let mut state = use_signal(AppState::default);
    let mut ws_manager = use_signal(|| WebSocketManager::new());

    // Initialize WebSocket connection
    use_effect(move || {
        let host = web_sys::window()
            .and_then(|w| w.location().host().ok())
            .unwrap_or_else(|| "localhost:3550".to_string());
        
        let ws_url = format!("ws://{}/ws", host);
        
        spawn(async move {
            if let Err(e) = ws_manager.write().connect(&ws_url).await {
                error!("Failed to connect to WebSocket: {:?}", e);
            }
        });
    });

    // Handle incoming WebSocket messages
    use_effect(move || {
        spawn(async move {
            loop {
                if let Some(msg) = ws_manager.read().receive_message().await {
                    handle_message(msg, &mut state);
                }
                gloo_timers::future::TimeoutFuture::new(10).await;
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
fn TopBar(state: Signal<AppState>) -> Element {
    let current_tab = state.read().current_tab;
    
    rsx! {
        div {
            class: "top-bar",
            style: "background: #2c2c2c; padding: 10px; display: flex; align-items: center; gap: 10px; border-bottom: 1px solid #444;",
            
            // Source selector
            select {
                style: "padding: 5px; background: #1e1e1e; color: #ddd; border: 1px solid #555; border-radius: 4px;",
                onchange: move |evt| {
                    warn!("Source changed: {}", evt.value());
                },
                option { "Select Source..." }
                for source in state.read().sources.iter() {
                    option { 
                        value: "{source:?}",
                        "{source:?}"
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
                value: "{current_tab:?}",
                onchange: move |evt| {
                    warn!("Tab changed: {}", evt.value());
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
            style: "flex: 1; background: #1e1e1e; color: #ddd; padding: 20px; overflow: auto;",
            
            h2 { 
                style: "margin-top: 0; color: #fff;",
                "{current_tab.display_name()}" 
            }
            
            p { 
                style: "color: #888;",
                "Content for {current_tab.display_name()} will be displayed here." 
            }
            
            // TODO: Render actual table/plot content based on current_tab
            div {
                style: "margin-top: 20px; padding: 20px; background: #2c2c2c; border-radius: 4px;",
                "Table/Plot content placeholder"
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

fn handle_message(msg: Vec<u8>, state: &mut Signal<AppState>) {
    let Ok(response) = Response::decode(&msg) else {
        error!("Failed to decode response message");
        return;
    };

    match response {
        (Response::Packet(packet), _) => {
            let state_ref = state.read();
            let mut streams = state_ref.streams.borrow_mut();
            streams.add_packet(packet);
        }
        (Response::Sources(sources), _) => {
            let mut s = state.write();
            if let Some(ref source) = s.selected_source {
                if !sources.contains(source) {
                    s.selected_source = None;
                }
            }
            s.sources = sources;
        }
        (Response::Sdp(stream_key, sdp), _) => {
            let state_ref = state.read();
            let mut streams = state_ref.streams.borrow_mut();
            if let Some(stream) = streams.rtp_streams.get_mut(&stream_key) {
                stream.add_sdp(sdp);
            }
        }
        (Response::PacketsStats(stats), _) => {
            let mut s = state.write();
            s.discharged_count = stats.discharged;
            s.overwritten_count = stats.overwritten;
        }
    }
}
