use dioxus::prelude::*;
use crate::app::AppState;
use netpix_common::packet::SessionPacket;

#[component]
pub fn RtpPacketsTable(state: Signal<AppState>) -> Element {
    let streams = state.read().streams.clone();
    let streams_ref = streams.borrow();
    
    // Filter for RTP packets only
    let rtp_packets: Vec<_> = streams_ref
        .packets
        .values()
        .filter(|packet| matches!(packet.contents, SessionPacket::Rtp(_)))
        .collect();
    
    let first_ts = rtp_packets
        .first()
        .map(|p| p.timestamp)
        .unwrap_or_default();
    
    // Build SSRC to alias lookup
    let ssrc_aliases: std::collections::HashMap<_, _> = streams_ref
        .rtp_streams
        .iter()
        .map(|(key, stream)| (*key, stream.alias.to_string()))
        .collect();
    
    rsx! {
        div {
            style: "width: 100%; height: 100%; display: flex; flex-direction: column;",
            
            // Table container
            div {
                style: "flex: 1; overflow: auto; background: #1e1e1e;",
                
                table {
                    style: "width: 100%; border-collapse: collapse; color: #ddd; font-family: monospace; font-size: 11px;",
                    
                    thead {
                        style: "position: sticky; top: 0; background: #2c2c2c; z-index: 10;",
                        tr {
                            th { style: "padding: 8px 4px; text-align: left; border-bottom: 2px solid #444; font-weight: bold; font-size: 10px;", "No." }
                            th { style: "padding: 8px 4px; text-align: left; border-bottom: 2px solid #444; font-weight: bold; font-size: 10px;", "Time" }
                            th { style: "padding: 8px 4px; text-align: left; border-bottom: 2px solid #444; font-weight: bold; font-size: 10px;", "Source" }
                            th { style: "padding: 8px 4px; text-align: left; border-bottom: 2px solid #444; font-weight: bold; font-size: 10px;", "Dest" }
                            th { style: "padding: 8px 4px; text-align: center; border-bottom: 2px solid #444; font-weight: bold; font-size: 10px;", "P" }
                            th { style: "padding: 8px 4px; text-align: center; border-bottom: 2px solid #444; font-weight: bold; font-size: 10px;", "X" }
                            th { style: "padding: 8px 4px; text-align: center; border-bottom: 2px solid #444; font-weight: bold; font-size: 10px;", "M" }
                            th { style: "padding: 8px 4px; text-align: left; border-bottom: 2px solid #444; font-weight: bold; font-size: 10px;", "PT" }
                            th { style: "padding: 8px 4px; text-align: left; border-bottom: 2px solid #444; font-weight: bold; font-size: 10px;", "Seq" }
                            th { style: "padding: 8px 4px; text-align: left; border-bottom: 2px solid #444; font-weight: bold; font-size: 10px;", "Timestamp" }
                            th { style: "padding: 8px 4px; text-align: left; border-bottom: 2px solid #444; font-weight: bold; font-size: 10px;", "SSRC" }
                            th { style: "padding: 8px 4px; text-align: left; border-bottom: 2px solid #444; font-weight: bold; font-size: 10px;", "Alias" }
                            th { style: "padding: 8px 4px; text-align: left; border-bottom: 2px solid #444; font-weight: bold; font-size: 10px;", "Payload" }
                        }
                    }
                    
                    tbody {
                        for (idx, packet) in rtp_packets.iter().enumerate() {
                            {
                                if let SessionPacket::Rtp(ref rtp) = packet.contents {
                                    let timestamp = packet.timestamp - first_ts;
                                    let time_str = format!("{:.4}", timestamp.as_secs_f64());
                                    let source_str = format!("{}", packet.source_addr);
                                    let dest_str = format!("{}", packet.destination_addr);
                                    
                                    let key = (
                                        packet.source_addr,
                                        packet.destination_addr,
                                        packet.transport_protocol,
                                        rtp.ssrc,
                                    );
                                    let alias = ssrc_aliases.get(&key).map(|s| s.as_str()).unwrap_or("");
                                    
                                    let padding = if rtp.padding { "✓" } else { "" };
                                    let extension = if rtp.extension { "✓" } else { "" };
                                    let marker = if rtp.marker { "✓" } else { "" };
                                    
                                    rsx! {
                                        tr {
                                            key: "{packet.id}",
                                            style: if idx % 2 == 0 { "background: #1e1e1e;" } else { "background: #252525;" },
                                            td { style: "padding: 6px 4px; border-bottom: 1px solid #333;", "{packet.id}" }
                                            td { style: "padding: 6px 4px; border-bottom: 1px solid #333;", "{time_str}" }
                                            td { style: "padding: 6px 4px; border-bottom: 1px solid #333; font-size: 10px;", "{source_str}" }
                                            td { style: "padding: 6px 4px; border-bottom: 1px solid #333; font-size: 10px;", "{dest_str}" }
                                            td { style: "padding: 6px 4px; border-bottom: 1px solid #333; text-align: center; color: #4CAF50;", "{padding}" }
                                            td { style: "padding: 6px 4px; border-bottom: 1px solid #333; text-align: center; color: #2196F3;", "{extension}" }
                                            td { style: "padding: 6px 4px; border-bottom: 1px solid #333; text-align: center; color: #FF9800;", "{marker}" }
                                            td { style: "padding: 6px 4px; border-bottom: 1px solid #333;", "{rtp.payload_type}" }
                                            td { style: "padding: 6px 4px; border-bottom: 1px solid #333;", "{rtp.sequence_number}" }
                                            td { style: "padding: 6px 4px; border-bottom: 1px solid #333;", "{rtp.timestamp}" }
                                            td { style: "padding: 6px 4px; border-bottom: 1px solid #333;", "{rtp.ssrc}" }
                                            td { style: "padding: 6px 4px; border-bottom: 1px solid #333; color: #9C27B0; font-weight: bold;", "{alias}" }
                                            td { style: "padding: 6px 4px; border-bottom: 1px solid #333;", "{rtp.payload_length}" }
                                        }
                                    }
                                } else {
                                    rsx! { tr {} }
                                }
                            }
                        }
                    }
                }
                
                if rtp_packets.is_empty() {
                    div {
                        style: "padding: 40px; text-align: center; color: #888;",
                        "No RTP packets captured yet"
                    }
                }
            }
            
            // Footer with packet count
            div {
                style: "padding: 10px; background: #2c2c2c; border-top: 1px solid #444; font-size: 12px; color: #888;",
                "RTP packets: {rtp_packets.len()}"
            }
        }
    }
}
