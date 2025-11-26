use dioxus::prelude::*;
use crate::app::AppState;
use netpix_common::packet::SessionPacket;

#[component]
pub fn StunPacketsTable(state: Signal<AppState>) -> Element {
    // Read update counter to trigger re-renders when data changes
    let _update = state.read().update_counter;
    let streams = state.read().streams.clone();
    let streams_ref = streams.borrow();
    
    // Filter for STUN packets only
    let stun_packets: Vec<_> = streams_ref
        .packets
        .values()
        .filter(|packet| matches!(packet.contents, SessionPacket::Stun(_)))
        .collect();
    
    let first_ts = stun_packets
        .first()
        .map(|p| p.timestamp)
        .unwrap_or_default();
    
    rsx! {
        div {
            style: "width: 100%; height: 100%; display: flex; flex-direction: column;",
            
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
