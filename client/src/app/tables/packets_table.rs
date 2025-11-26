use dioxus::prelude::*;
use crate::app::AppState;

#[component]
pub fn PacketsTable(state: Signal<AppState>) -> Element {
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
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Protocol" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Length" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Treated as" }
                        }
                    }
                    
                    tbody {
                        for (idx, packet) in sorted_packets.iter().enumerate() {
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
                
                if sorted_packets.is_empty() {
                    div {
                        style: "padding: 40px; text-align: center; color: #888;",
                        "No packets captured yet"
                    }
                }
            }
            
            // Footer with packet count
            div {
                style: "padding: 10px; background: #2c2c2c; border-top: 1px solid #444; font-size: 12px; color: #888;",
                "Total packets: {sorted_packets.len()}"
            }
        }
    }
}
