use dioxus::prelude::*;
use crate::app::AppState;
use crate::app::components::FilterInput;
use crate::app::tables::filters::{build_mpegts_packets_filter_help, parse_mpegts_packet_filter, MpegtsPacketFilterContext, MpegtsPacketFilterType};
use crate::filter_system::FilterExpression;
use netpix_common::packet::SessionPacket;

#[component]
pub fn MpegtsPacketsTable(state: Signal<AppState>) -> Element {
    // Read update counter to trigger re-renders when data changes
    let _update = state.read().update_counter;
    let filter_text = use_signal(String::new);
    let filter_error = use_signal(|| None::<String>);
    
    let streams = state.read().streams.clone();
    let streams_ref = streams.borrow();
    
    // Parse filter
    let filter: Option<MpegtsPacketFilterType> = if filter_text.read().is_empty() {
        None
    } else {
        parse_mpegts_packet_filter(&filter_text.read()).ok()
    };
    
    // Filter for MPEG-TS packets only
    let mpegts_packets: Vec<_> = streams_ref
        .packets
        .values()
        .filter(|packet| {
            if let SessionPacket::Mpegts(ref mpegts) = packet.contents {
                if let Some(ref f) = filter {
                    let ctx = MpegtsPacketFilterContext { packet, mpegts };
                    f.matches(&ctx)
                } else {
                    true
                }
            } else {
                false
            }
        })
        .collect();
    
    let first_ts = streams_ref
        .packets
        .values()
        .filter(|p| matches!(p.contents, SessionPacket::Mpegts(_)))
        .next()
        .map(|p| p.timestamp)
        .unwrap_or_default();
    
    rsx! {
        div {
            style: "width: 100%; height: 100%; display: flex; flex-direction: column;",
            
            // Filter bar
            div {
                style: "padding: 10px 20px; background: #252525; border-bottom: 1px solid #333;",
                FilterInput {
                    filter_text: filter_text,
                    filter_error: filter_error,
                    placeholder: "Filter MPEG-TS packets (e.g., source:192.168 AND pid:100)".to_string(),
                    help_content: build_mpegts_packets_filter_help().to_string(),
                }
            }
            
            // Table container
            div {
                style: "flex: 1; overflow: auto; background: #1e1e1e;",
                
                table {
                    style: "width: 100%; border-collapse: collapse; color: #ddd; font-family: monospace; font-size: 12px;",
                    
                    thead {
                        style: "position: sticky; top: 0; background: #2c2c2c; z-index: 10;",
                        tr {
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "No." }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Time" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Source" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Destination" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Fragments" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "PIDs" }
                        }
                    }
                    
                    tbody {
                        for (idx, packet) in mpegts_packets.iter().enumerate() {
                            {
                                if let SessionPacket::Mpegts(ref mpegts) = packet.contents {
                                    let timestamp = packet.timestamp - first_ts;
                                    let time_str = format!("{:.4}", timestamp.as_secs_f64());
                                    let source_str = format!("{}", packet.source_addr);
                                    let dest_str = format!("{}", packet.destination_addr);
                                    let fragments = mpegts.number_of_fragments;
                                    let pids: Vec<String> = mpegts.fragments.iter()
                                        .map(|f| f.header.pid.to_string())
                                        .collect();
                                    let pids_str = pids.join(", ");
                                    
                                    rsx! {
                                        tr {
                                            key: "{packet.id}",
                                            style: if idx % 2 == 0 { "background: #1e1e1e;" } else { "background: #252525;" },
                                            td { style: "padding: 8px; border-bottom: 1px solid #333;", "{packet.id}" }
                                            td { style: "padding: 8px; border-bottom: 1px solid #333;", "{time_str}" }
                                            td { style: "padding: 8px; border-bottom: 1px solid #333;", "{source_str}" }
                                            td { style: "padding: 8px; border-bottom: 1px solid #333;", "{dest_str}" }
                                            td { style: "padding: 8px; border-bottom: 1px solid #333; color: #2196F3;", "{fragments}" }
                                            td { style: "padding: 8px; border-bottom: 1px solid #333; font-size: 11px;", "{pids_str}" }
                                        }
                                    }
                                } else {
                                    rsx! { tr {} }
                                }
                            }
                        }
                    }
                }
                
                if mpegts_packets.is_empty() {
                    div {
                        style: "padding: 40px; text-align: center; color: #888;",
                        "No MPEG-TS packets captured yet"
                    }
                }
            }
            
            // Footer with packet count
            div {
                style: "padding: 10px; background: #2c2c2c; border-top: 1px solid #444; font-size: 12px; color: #888;",
                "MPEG-TS packets: {mpegts_packets.len()}"
            }
        }
    }
}
