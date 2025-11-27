use dioxus::prelude::*;
use crate::app::AppState;
use crate::app::components::{FilterInput, FilterHelpData};
use crate::app::tables::filters::{RtpPacketFilterContext, parse_rtp_packet_filter};
use crate::filter_system::FilterExpression;
use netpix_common::packet::SessionPacket;

fn rtp_packets_help_data() -> FilterHelpData {
    FilterHelpData::new(
        "RTP Packet Filters",
        &[
            ("source:<ip>", "Filter by source IP address"),
            ("dest:<ip>", "Filter by destination IP address"),
            ("alias:<name>", "Filter by stream alias"),
            ("padding:+/-", "Filter by padding flag (+ = set, - = not set)"),
            ("extension:+/-", "Filter by extension header"),
            ("marker:+/-", "Filter by marker bit"),
            ("seq:<num>", "Filter by sequence number"),
            ("timestamp:<op><num>", "Filter by RTP timestamp (>, <, =)"),
            ("payload:<op><size>", "Filter by payload size in bytes"),
        ],
        &[
            "source:192.168 AND marker:+",
            "payload:>500 OR extension:+",
            "alias:stream1 AND NOT padding:+",
        ],
    )
}

#[component]
pub fn RtpPacketsTable(state: Signal<AppState>) -> Element {
    let filter_text = use_signal(String::new);
    let mut filter_error = use_signal(|| None::<String>);
    
    // Read update counter to trigger re-renders when data changes
    let _update = state.read().update_counter;
    let streams = state.read().streams.clone();
    let streams_ref = streams.borrow();
    
    // Build SSRC to alias lookup first
    let ssrc_aliases: std::collections::HashMap<_, _> = streams_ref
        .rtp_streams
        .iter()
        .map(|(key, stream)| (*key, stream.alias.to_string()))
        .collect();
    
    // Parse filter
    let parsed_filter = if filter_text.read().is_empty() {
        None
    } else {
        match parse_rtp_packet_filter(&filter_text.read()) {
            Ok(f) => { filter_error.set(None); Some(f) }
            Err(e) => { filter_error.set(Some(e.to_string())); None }
        }
    };
    
    // Filter for RTP packets only
    let rtp_packets: Vec<_> = streams_ref
        .packets
        .values()
        .filter(|packet| matches!(packet.contents, SessionPacket::Rtp(_)))
        .filter(|packet| {
            if let Some(ref filter) = parsed_filter {
                if let SessionPacket::Rtp(ref rtp) = packet.contents {
                    let key = (packet.source_addr, packet.destination_addr, packet.transport_protocol, rtp.ssrc);
                    let alias = ssrc_aliases.get(&key).map(|s| s.as_str()).unwrap_or("");
                    let ctx = RtpPacketFilterContext {
                        source_addr: &packet.source_addr.to_string(),
                        destination_addr: &packet.destination_addr.to_string(),
                        alias,
                        packet: rtp,
                    };
                    filter.matches(&ctx)
                } else { true }
            } else { true }
        })
        .collect();
    
    let first_ts = rtp_packets
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
                placeholder: "Filter: source:ip, dest:ip, alias:name, marker:+/-, seq:num, payload:>size...".to_string(),
                help_content: String::new(),
                help_data: Some(rtp_packets_help_data()),
            }
            
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
