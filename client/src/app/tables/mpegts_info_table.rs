use dioxus::prelude::*;
use crate::app::AppState;

#[component]
pub fn MpegtsInfoTable(state: Signal<AppState>) -> Element {
    // Read update counter to trigger re-renders when data changes
    let _update = state.read().update_counter;
    let streams = state.read().streams.clone();
    let streams_ref = streams.borrow();
    
    let mut stream_list: Vec<_> = streams_ref.mpeg_ts_streams.values().collect();
    stream_list.sort_by_key(|s| s.alias.as_str());
    
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
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Alias" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Stream" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "PAT" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "PMT" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Substreams" }
                            th { style: "padding: 10px; text-align: left; border-bottom: 2px solid #444; font-weight: bold;", "Info" }
                        }
                    }
                    
                    tbody {
                        for (idx, stream) in stream_list.iter().enumerate() {
                            {
                                let stream_addr = format!("{} → {}", 
                                    stream.stream_info.packet_association_table.source_addr, 
                                    stream.stream_info.packet_association_table.destination_addr);
                                let has_pat = stream.stream_info.pat.is_some();
                                let pmt_count = stream.stream_info.pmt.len();
                                let substreams = stream.substreams.len();
                                let info = if has_pat {
                                    if pmt_count > 0 {
                                        format!("{} PMT(s)", pmt_count)
                                    } else {
                                        "PAT only".to_string()
                                    }
                                } else {
                                    "No PSI data".to_string()
                                };
                                
                                rsx! {
                                    tr {
                                        key: "{stream.alias}",
                                        style: if idx % 2 == 0 { "background: #1e1e1e;" } else { "background: #252525;" },
                                        td { style: "padding: 8px; border-bottom: 1px solid #333; color: #9C27B0; font-weight: bold;", "{stream.alias}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333; font-size: 11px;", "{stream_addr}" }
                                        td { 
                                            style: if has_pat { "padding: 8px; border-bottom: 1px solid #333; color: #4CAF50;" } else { "padding: 8px; border-bottom: 1px solid #333; color: #888;" },
                                            if has_pat { "✓" } else { "✗" }
                                        }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333;", "{pmt_count}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333; color: #2196F3;", "{substreams}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #333; font-size: 11px;", "{info}" }
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
