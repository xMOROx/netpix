use dioxus::prelude::*;
use crate::app::AppState;
use crate::app::components::FilterInput;
use crate::app::tables::filters::{RtpStreamFilterContext, parse_rtp_stream_filter};
use crate::filter_system::FilterExpression;

#[component]
pub fn RtpStreamsTable(state: Signal<AppState>) -> Element {
    let mut filter_text = use_signal(String::new);
    let mut filter_error = use_signal(|| None::<String>);
    
    // Read update counter to trigger re-renders when data changes
    let _update = state.read().update_counter;
    let streams = state.read().streams.clone();
    let streams_ref = streams.borrow();
    
    // Parse filter
    let parsed_filter = if filter_text.read().is_empty() {
        None
    } else {
        match parse_rtp_stream_filter(&filter_text.read()) {
            Ok(f) => { filter_error.set(None); Some(f) }
            Err(e) => { filter_error.set(Some(e.to_string())); None }
        }
    };
    
    let mut stream_list: Vec<_> = streams_ref.rtp_streams.values()
        .filter(|stream| {
            if let Some(ref filter) = parsed_filter {
                let ctx = RtpStreamFilterContext {
                    source_addr: &stream.source_addr.to_string(),
                    destination_addr: &stream.destination_addr.to_string(),
                    alias: &stream.alias,
                    stream,
                };
                filter.matches(&ctx)
            } else { true }
        })
        .collect();
    stream_list.sort_by_key(|s| s.alias.as_str());
    
    rsx! {
        div {
            style: "width: 100%; height: 100%; display: flex; flex-direction: column;",
            
            // Filter input
            FilterInput {
                filter_text: filter_text,
                filter_error: filter_error,
                placeholder: "Filter: source:ip, dest:ip, alias:name, cname:name, bitrate:>1000, packets:>100...".to_string(),
                help_content: "source:192.168 - Source IP\ndest:10.0 - Destination IP\nalias:stream - Stream alias\ncname:name - CNAME\nbitrate:>1000 - Bitrate in kbps\npackets:>100 - Packet count".to_string(),
            }
            
            // Table container
            div {
                style: "flex: 1; overflow: auto; background: #1e1e1e;",
                
                table {
                    style: "width: 100%; border-collapse: collapse; color: #ddd; font-family: monospace; font-size: 12px;",
                    
                    thead {
                        style: "position: sticky; top: 0; background: #2c2c2c; z-index: 10;",
                        tr {
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Alias" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Source" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Destination" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "SSRC" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Packets" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Expected" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Lost" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Loss %" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Jitter" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Bitrate" }
                        }
                    }
                    
                    tbody {
                        for (idx, stream) in stream_list.iter().enumerate() {
                            {
                                let source_str = format!("{}", stream.source_addr);
                                let dest_str = format!("{}", stream.destination_addr);
                                let packets_count = stream.rtp_packets.len();
                                let expected = stream.get_expected_count();
                                let lost = if expected > packets_count { expected - packets_count } else { 0 };
                                let loss_pct = if expected > 0 {
                                    format!("{:.2}%", (lost as f64 / expected as f64) * 100.0)
                                } else {
                                    "0.00%".to_string()
                                };
                                let jitter = stream.get_mean_jitter()
                                    .map(|j| format!("{:.2}", j))
                                    .unwrap_or_else(|| "N/A".to_string());
                                let bitrate = format!("{:.0} kbps", stream.get_mean_bitrate());
                                
                                rsx! {
                                    tr {
                                        key: "{stream.alias}",
                                        style: if idx % 2 == 0 { "background: #1e1e1e;" } else { "background: #252525;" },
                                        td { style: "padding: 8px; border-bottom: 1px solid #333; color: #9C27B0; font-weight: bold;", "{stream.alias}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{source_str}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{dest_str}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{stream.ssrc}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{packets_count}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{expected}" }
                                        td { 
                                            style: if lost > 0 { "padding: 8px; border-bottom: 1px solid #333; color: #f44336;" } else { "padding: 8px; border-bottom: 1px solid #333;" },
                                            "{lost}" 
                                        }
                                        td { 
                                            style: if lost > 0 { "padding: 8px; border-bottom: 1px solid #333; color: #f44336;" } else { "padding: 8px; border-bottom: 1px solid #333;" },
                                            "{loss_pct}" 
                                        }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{jitter}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{bitrate}" }
                                    }
                                }
                            }
                        }
                    }
                }
                
                if stream_list.is_empty() {
                    div {
                        style: "padding: 40px; text-align: center; color: #888;",
                        "No RTP streams detected yet"
                    }
                }
            }
            
            // Footer with stream count
            div {
                style: "padding: 10px; background: #2c2c2c; border-top: 1px solid #444; font-size: 12px; color: #888;",
                "RTP streams: {stream_list.len()}"
            }
        }
    }
}
