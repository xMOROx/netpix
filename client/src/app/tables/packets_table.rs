use dioxus::prelude::*;
use crate::app::AppState;
use crate::app::components::{FilterInput, build_filter_help};
use crate::filter_system::FilterExpression;
use super::filters::{PacketFilterContext, parse_packet_filter};

// Packets table filter help
fn packets_filter_help() -> String {
    build_filter_help(
        "Network Packet Filters",
        &[
            ("source:<ip>", "Filter by source IP address"),
            ("dest:<ip>", "Filter by destination IP address"),
            ("protocol:<proto>", "Filter by protocol (TCP, UDP, RTP, RTCP, MPEG-TS)"),
            ("type:<type>", "Filter by session type"),
            ("length:<op><size>", "Filter by packet size (>, <, >=, <=, or exact)"),
        ],
        &[
            "source:192.168 AND protocol:udp",
            "length:>100 AND type:rtp",
            "NOT dest:10.0.0.1",
            "(protocol:tcp AND length:>500) OR source:192.168",
        ],
    )
}

#[component]
pub fn PacketsTable(state: Signal<AppState>) -> Element {
    // Filter state
    let filter_text = use_signal(String::new);
    let mut filter_error = use_signal(|| None::<String>);
    
    // Read update counter to trigger re-renders when data changes
    let _update = state.read().update_counter;
    let streams = state.read().streams.clone();
    let packets = streams.borrow();
    let packet_list: Vec<_> = packets.packets.values().collect();
    let mut sorted_packets = packet_list.clone();
    sorted_packets.sort_by_key(|p| p.timestamp);
    
    let first_ts = sorted_packets
        .first()
        .map(|p| p.timestamp)
        .unwrap_or_default();
    
    // Apply filter
    let filter_str = filter_text.read().clone();
    let filtered_packets: Vec<_> = if filter_str.trim().is_empty() {
        filter_error.set(None);
        sorted_packets
    } else {
        match parse_packet_filter(&filter_str) {
            Ok(filter) => {
                filter_error.set(None);
                sorted_packets
                    .into_iter()
                    .filter(|packet| {
                        let ctx = PacketFilterContext { packet };
                        filter.matches(&ctx)
                    })
                    .collect()
            }
            Err(e) => {
                filter_error.set(Some(format!("{}", e)));
                Vec::new()
            }
        }
    };
    
    rsx! {
        div {
            style: "width: 100%; height: 100%; display: flex; flex-direction: column;",
            
            // Filter input
            FilterInput {
                filter_text: filter_text,
                filter_error: filter_error,
                placeholder: "Filter packets (e.g., source:192.168 AND protocol:udp)".to_string(),
                help_content: packets_filter_help(),
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
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Protocol" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Length" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Treated as" }
                        }
                    }
                    
                    tbody {
                        for (idx, packet) in filtered_packets.iter().enumerate() {
                            {
                                let timestamp = packet.timestamp - first_ts;
                                let time_str = format!("{:.4}", timestamp.as_secs_f64());
                                let transport_str = format!("{:?}", packet.transport_protocol);
                                let session_str = format!("{:?}", packet.session_protocol);
                                let source_str = format!("{}", packet.source_addr);
                                let dest_str = format!("{}", packet.destination_addr);
                                
                                rsx! {
                                    tr {
                                        key: "{packet.id}",
                                        style: if idx % 2 == 0 { "background: #1e1e1e;" } else { "background: #252525;" },
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{packet.id}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{time_str}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{source_str}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{dest_str}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{transport_str}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{packet.length}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{session_str}" }
                                    }
                                }
                            }
                        }
                    }
                }
                
                if filtered_packets.is_empty() {
                    div {
                        style: "padding: 40px; text-align: center; color: #888;",
                        if filter_str.trim().is_empty() {
                            "No packets captured yet"
                        } else {
                            "No packets match the filter"
                        }
                    }
                }
            }
            
            // Footer with packet count
            div {
                style: "padding: 10px; background: #2c2c2c; border-top: 1px solid #444; font-size: 12px; color: #888;",
                "Showing {filtered_packets.len()} packets"
            }
        }
    }
}
