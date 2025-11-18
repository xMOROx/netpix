use dioxus::prelude::*;
use crate::app::AppState;
use netpix_common::packet::SessionPacket;
use netpix_common::RtcpPacket;

#[component]
pub fn RtcpPacketsTable(state: Signal<AppState>) -> Element {
    let streams = state.read().streams.clone();
    let streams_ref = streams.borrow();
    
    // Collect RTCP packets (which can be compound - multiple RTCP packets in one UDP packet)
    let mut rtcp_packets = Vec::new();
    for packet in streams_ref.packets.values() {
        if let SessionPacket::Rtcp(ref rtcp_list) = packet.contents {
            for (idx, rtcp_packet) in rtcp_list.iter().enumerate() {
                rtcp_packets.push((packet, rtcp_packet, idx + 1));
            }
        }
    }
    
    let first_ts = streams_ref.packets.first()
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
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Details" }
                        }
                    }
                    
                    tbody {
                        for (table_idx, (packet, rtcp, compound_idx)) in rtcp_packets.iter().enumerate() {
                            {
                                let timestamp = packet.timestamp - first_ts;
                                let time_str = format!("{:.4}", timestamp.as_secs_f64());
                                let source_str = format!("{}", packet.source_addr);
                                let dest_str = format!("{}", packet.destination_addr);
                                let packet_num = format!("{} ({})", packet.id, compound_idx);
                                let rtcp_type = rtcp.get_type_name();
                                let details = get_rtcp_details(rtcp);
                                
                                rsx! {
                                    tr {
                                        key: "{packet.id}-{compound_idx}",
                                        style: if table_idx % 2 == 0 { "background: #1e1e1e;" } else { "background: #252525;" },
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{packet_num}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{time_str}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{source_str}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{dest_str}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333; color: #4CAF50;", "{rtcp_type}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333; font-size: 11px;", "{details}" }
                                    }
                                }
                            }
                        }
                    }
                }
                
                if rtcp_packets.is_empty() {
                    div {
                        style: "padding: 40px; text-align: center; color: #888;",
                        "No RTCP packets captured yet"
                    }
                }
            }
            
            // Footer with packet count
            div {
                style: "padding: 10px; background: #2c2c2c; border-top: 1px solid #444; font-size: 12px; color: #888;",
                "RTCP packets: {rtcp_packets.len()}"
            }
        }
    }
}

fn get_rtcp_details(rtcp: &RtcpPacket) -> String {
    match rtcp {
        RtcpPacket::SenderReport(sr) => {
            format!("SSRC: {}, Packets: {}, Bytes: {}", 
                sr.ssrc, sr.packet_count, sr.octet_count)
        },
        RtcpPacket::ReceiverReport(rr) => {
            format!("SSRC: {}, Reports: {}", rr.ssrc, rr.reports.len())
        },
        RtcpPacket::SourceDescription(sdes) => {
            let items: Vec<_> = sdes.chunks.iter()
                .flat_map(|c| &c.items)
                .map(|item| format!("{:?}", item.sdes_type))
                .collect();
            format!("Items: {}", items.join(", "))
        },
        RtcpPacket::Goodbye(bye) => {
            format!("SSRCs: {}", bye.sources.len())
        },
        _ => rtcp.get_type_name().to_string(),
    }
}
