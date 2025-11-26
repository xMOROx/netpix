use dioxus::prelude::*;
use crate::app::AppState;
use crate::app::components::FilterInput;
use crate::app::tables::filters::{build_mpegts_streams_filter_help, parse_mpegts_stream_filter, MpegtsStreamFilterContext, MpegtsStreamFilterType};
use crate::filter_system::FilterExpression;

#[component]
pub fn MpegtsStreamsTable(state: Signal<AppState>) -> Element {
    // Read update counter to trigger re-renders when data changes
    let _update = state.read().update_counter;
    let filter_text = use_signal(String::new);
    let filter_error = use_signal(|| None::<String>);
    
    let streams = state.read().streams.clone();
    let streams_ref = streams.borrow();
    
    // Parse filter
    let filter: Option<MpegtsStreamFilterType> = if filter_text.read().is_empty() {
        None
    } else {
        parse_mpegts_stream_filter(&filter_text.read()).ok()
    };
    
    let mut stream_list: Vec<_> = streams_ref.mpeg_ts_streams.values()
        .filter(|stream| {
            if let Some(ref f) = filter {
                let ctx = MpegtsStreamFilterContext { stream };
                f.matches(&ctx)
            } else {
                true
            }
        })
        .collect();
    stream_list.sort_by_key(|s| s.alias.as_str());
    
    rsx! {
        div {
            style: "width: 100%; height: 100%; display: flex; flex-direction: column;",
            
            // Filter bar
            div {
                style: "padding: 10px 20px; background: #252525; border-bottom: 1px solid #333;",
                FilterInput {
                    filter_text: filter_text,
                    filter_error: filter_error,
                    placeholder: "Filter MPEG-TS streams (e.g., alias:stream1 AND substreams:>3)".to_string(),
                    help_content: build_mpegts_streams_filter_help().to_string(),
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
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Alias" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Source" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Destination" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Packets" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Substreams" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Bitrate" }
                        }
                    }
                    
                    tbody {
                        for (idx, stream) in stream_list.iter().enumerate() {
                            {
                                let source_str = format!("{}", stream.stream_info.packet_association_table.source_addr);
                                let dest_str = format!("{}", stream.stream_info.packet_association_table.destination_addr);
                                let packets_count = stream.stream_info.packets.len();
                                let substreams_count = stream.substreams.len();
                                let bitrate = format!("{:.0} kbps", stream.stream_info.statistics.get_bitrate().get_protocol_bitrate());
                                
                                rsx! {
                                    tr {
                                        key: "{stream.alias}",
                                        style: if idx % 2 == 0 { "background: #1e1e1e;" } else { "background: #252525;" },
                                        td { style: "padding: 8px; border-bottom: 1px solid #333; color: #9C27B0; font-weight: bold;", "{stream.alias}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{source_str}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{dest_str}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{packets_count}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333; color: #2196F3;", "{substreams_count}" }
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
                        "No MPEG-TS streams detected yet"
                    }
                }
            }
            
            // Footer with stream count
            div {
                style: "padding: 10px; background: #2c2c2c; border-top: 1px solid #444; font-size: 12px; color: #888;",
                "MPEG-TS streams: {stream_list.len()}"
            }
        }
    }
}
