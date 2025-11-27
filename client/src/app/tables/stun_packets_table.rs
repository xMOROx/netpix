use dioxus::prelude::*;
use crate::app::AppState;
use crate::app::components::{FilterInput, FilterHelpData};
use crate::app::tables::filters::{StunPacketFilterContext, parse_stun_packet_filter};
use crate::filter_system::FilterExpression;
use netpix_common::packet::SessionPacket;

fn stun_packets_help_data() -> FilterHelpData {
    FilterHelpData::new(
        "STUN Packet Filters",
        &[
            ("source:<ip>", "Filter by source IP address"),
            ("dest:<ip>", "Filter by destination IP address"),
            ("type:<type>", "Filter by STUN message type (binding, etc.)"),
            ("transaction:<id>", "Filter by transaction ID (hex)"),
            ("length:<op><size>", "Filter by message length"),
        ],
        &[
            "type:binding AND source:192.168",
            "length:>100",
            "transaction:abc123",
        ],
    )
}

#[component]
pub fn StunPacketsTable(state: Signal<AppState>) -> Element {
    let filter_text = use_signal(String::new);
    let mut filter_error = use_signal(|| None::<String>);
    
    // Read update counter to trigger re-renders when data changes
    let _update = state.read().update_counter;
    let streams = state.read().streams.clone();
    let streams_ref = streams.borrow();
    
    // Parse filter
    let parsed_filter = if filter_text.read().is_empty() {
        None
    } else {
        match parse_stun_packet_filter(&filter_text.read()) {
            Ok(f) => { filter_error.set(None); Some(f) }
            Err(e) => { filter_error.set(Some(e.to_string())); None }
        }
    };
    
    // Filter for STUN packets only
    let stun_packets: Vec<_> = streams_ref
        .packets
        .values()
        .filter(|packet| matches!(packet.contents, SessionPacket::Stun(_)))
        .filter(|packet| {
            if let Some(ref filter) = parsed_filter {
                if let SessionPacket::Stun(ref stun) = packet.contents {
                    let ctx = StunPacketFilterContext {
                        source_addr: &packet.source_addr.to_string(),
                        destination_addr: &packet.destination_addr.to_string(),
                        packet: stun,
                    };
                    filter.matches(&ctx)
                } else { true }
            } else { true }
        })
        .collect();
    
    let first_ts = stun_packets
        .first()
        .map(|p| p.timestamp)
        .unwrap_or_default();
    
    rsx! {
        div {
            style: "width: 100%; height: 100%; display: flex; flex-direction: column;",
            
            // Filter input
            FilterInput {
                filter_text: filter_text,
                filter_error: filter_error,
                placeholder: "Filter: source:ip, dest:ip, type:binding, transaction:id, length:>size...".to_string(),
                help_content: String::new(),
                help_data: Some(stun_packets_help_data()),
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
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Type" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Transaction ID" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Length" }
                        }
                    }
                    
                    tbody {
                        for (idx, packet) in stun_packets.iter().enumerate() {
                            {
                                if let SessionPacket::Stun(ref stun) = packet.contents {
                                    let timestamp = packet.timestamp - first_ts;
                                    let time_str = format!("{:.4}", timestamp.as_secs_f64());
                                    let source_str = format!("{}", packet.source_addr);
                                    let dest_str = format!("{}", packet.destination_addr);
                                    let msg_type = format!("{:?}", stun.message_type);
                                    let transaction_id = format!("{:02x}{:02x}{:02x}...{:02x}", 
                                        stun.transaction_id[0],
                                        stun.transaction_id[1],
                                        stun.transaction_id[2],
                                        stun.transaction_id[11]);
                                    
                                    rsx! {
                                        tr {
                                            key: "{packet.id}",
                                            style: if idx % 2 == 0 { "background: #1e1e1e;" } else { "background: #252525;" },
                                            td { style: "padding: 8px; border-bottom: 1px solid #333;", "{packet.id}" }
                                            td { style: "padding: 8px; border-bottom: 1px solid #333;", "{time_str}" }
                                            td { style: "padding: 8px; border-bottom: 1px solid #333;", "{source_str}" }
                                            td { style: "padding: 8px; border-bottom: 1px solid #333;", "{dest_str}" }
                                            td { style: "padding: 8px; border-bottom: 1px solid #333; color: #FF9800;", "{msg_type}" }
                                            td { style: "padding: 8px; border-bottom: 1px solid #333; font-size: 11px;", "{transaction_id}" }
                                            td { style: "padding: 8px; border-bottom: 1px solid #333;", "{stun.message_length}" }
                                        }
                                    }
                                } else {
                                    rsx! { tr {} }
                                }
                            }
                        }
                    }
                }
                
                if stun_packets.is_empty() {
                    div {
                        style: "padding: 40px; text-align: center; color: #888;",
                        "No STUN packets captured yet"
                    }
                }
            }
            
            // Footer with packet count
            div {
                style: "padding: 10px; background: #2c2c2c; border-top: 1px solid #444; font-size: 12px; color: #888;",
                "STUN packets: {stun_packets.len()}"
            }
        }
    }
}
