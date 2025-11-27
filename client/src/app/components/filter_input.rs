use dioxus::prelude::*;

/// Filter help data structure
#[derive(Clone, PartialEq)]
pub struct FilterHelpData {
    pub title: String,
    pub filters: Vec<(String, String)>,
    pub examples: Vec<String>,
}

impl FilterHelpData {
    pub fn new(title: &str, filters: &[(&str, &str)], examples: &[&str]) -> Self {
        Self {
            title: title.to_string(),
            filters: filters.iter().map(|(f, d)| (f.to_string(), d.to_string())).collect(),
            examples: examples.iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// A filter input component with help modal
#[component]
pub fn FilterInput(
    filter_text: Signal<String>,
    filter_error: Signal<Option<String>>,
    placeholder: String,
    help_content: String,
    #[props(default)] help_data: Option<FilterHelpData>,
) -> Element {
    let mut show_help = use_signal(|| false);
    let error = filter_error.read().clone();
    let has_error = error.is_some();
    let border_color = if has_error { "#f44336" } else { "#555" };
    
    rsx! {
        div {
            class: "filter-input-container",
            style: "display: flex; align-items: center; gap: 8px; padding: 8px 16px; background: #252525; border-bottom: 1px solid #444;",
            
            // Help button
            button {
                style: "padding: 4px 10px; background: #3a3a5a; color: #ddd; border: 1px solid #555; border-radius: 4px; cursor: pointer; font-size: 13px; font-weight: bold;",
                title: "Show filter help",
                onclick: move |_| show_help.set(true),
                "?"
            }
            
            // Filter input
            input {
                r#type: "text",
                placeholder: "{placeholder}",
                value: "{filter_text}",
                style: "flex: 1; padding: 8px 12px; background: #1e1e1e; color: #ddd; border: 1px solid {border_color}; border-radius: 4px; font-family: monospace; font-size: 13px;",
                oninput: move |evt| {
                    filter_text.set(evt.value().clone());
                }
            }
            
            // Error indicator
            if let Some(err) = error {
                div {
                    class: "filter-error",
                    style: "color: #f44336; font-size: 12px; max-width: 300px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                    title: "{err}",
                    "⚠ {err}"
                }
            }
            
            // Clear button
            if !filter_text.read().is_empty() {
                button {
                    style: "padding: 4px 8px; background: #333; color: #888; border: 1px solid #555; border-radius: 4px; cursor: pointer; font-size: 12px;",
                    onclick: move |_| {
                        filter_text.set(String::new());
                        filter_error.set(None);
                    },
                    "Clear"
                }
            }
        }
        
        // Help modal
        if *show_help.read() {
            {
                let help_data_clone = help_data.clone();
                rsx! {
                    FilterHelpModal {
                        show_help: show_help,
                        help_content: help_content.clone(),
                        help_data: help_data_clone,
                    }
                }
            }
        }
    }
}

/// Filter help modal component
#[component]
fn FilterHelpModal(
    show_help: Signal<bool>,
    help_content: String,
    help_data: Option<FilterHelpData>,
) -> Element {
    // Use structured help data if available, otherwise fall back to text
    let (title, filters, examples) = if let Some(data) = &help_data {
        (data.title.clone(), data.filters.clone(), data.examples.clone())
    } else {
        // Parse from help_content string (fallback)
        ("Filter Help".to_string(), Vec::new(), Vec::new())
    };
    
    rsx! {
        // Modal overlay
        div {
            style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.7); display: flex; align-items: center; justify-content: center; z-index: 1000;",
            onclick: move |_| show_help.set(false),
            
            // Modal content
            div {
                style: "background: #2a2a2a; border: 1px solid #555; border-radius: 8px; padding: 24px; min-width: 500px; max-width: 700px; max-height: 80vh; overflow-y: auto; box-shadow: 0 8px 32px rgba(0,0,0,0.5);",
                onclick: move |e| e.stop_propagation(),
                
                // Header
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;",
                    h2 {
                        style: "margin: 0; color: #fff; font-size: 20px;",
                        "{title}"
                    }
                    button {
                        style: "background: #444; border: none; color: #aaa; font-size: 20px; cursor: pointer; padding: 4px 10px; border-radius: 4px;",
                        onclick: move |_| show_help.set(false),
                        "×"
                    }
                }
                
                // Basic Filters section
                if !filters.is_empty() {
                    div {
                        style: "margin-bottom: 20px;",
                        h3 {
                            style: "color: #6a9fb5; font-size: 16px; margin-bottom: 12px;",
                            "📋 Basic Filters"
                        }
                        for (filter, desc) in filters.iter() {
                            div {
                                style: "display: flex; gap: 8px; margin-bottom: 8px; padding-left: 12px;",
                                span { style: "color: #888;", "•" }
                                code {
                                    style: "background: #1e1e1e; color: #e6db74; padding: 2px 6px; border-radius: 3px; font-family: monospace;",
                                    "{filter}"
                                }
                                span { style: "color: #bbb;", "— {desc}" }
                            }
                        }
                    }
                }
                
                // Logical Operators section
                div {
                    style: "margin-bottom: 20px;",
                    h3 {
                        style: "color: #6a9fb5; font-size: 16px; margin-bottom: 12px;",
                        "🔗 Logical Operators"
                    }
                    div {
                        style: "display: flex; gap: 8px; margin-bottom: 8px; padding-left: 12px;",
                        span { style: "color: #888;", "•" }
                        code {
                            style: "background: #1e1e1e; color: #a6e22e; padding: 2px 6px; border-radius: 3px; font-family: monospace;",
                            "AND"
                        }
                        span { style: "color: #bbb;", "— Both conditions must match" }
                    }
                    div {
                        style: "display: flex; gap: 8px; margin-bottom: 8px; padding-left: 12px;",
                        span { style: "color: #888;", "•" }
                        code {
                            style: "background: #1e1e1e; color: #a6e22e; padding: 2px 6px; border-radius: 3px; font-family: monospace;",
                            "OR"
                        }
                        span { style: "color: #bbb;", "— Either condition matches" }
                    }
                    div {
                        style: "display: flex; gap: 8px; margin-bottom: 8px; padding-left: 12px;",
                        span { style: "color: #888;", "•" }
                        code {
                            style: "background: #1e1e1e; color: #f92672; padding: 2px 6px; border-radius: 3px; font-family: monospace;",
                            "NOT"
                        }
                        span { style: "color: #bbb;", "— Negates the condition" }
                    }
                    div {
                        style: "display: flex; gap: 8px; margin-bottom: 8px; padding-left: 12px;",
                        span { style: "color: #888;", "•" }
                        code {
                            style: "background: #1e1e1e; color: #66d9ef; padding: 2px 6px; border-radius: 3px; font-family: monospace;",
                            "( )"
                        }
                        span { style: "color: #bbb;", "— Grouping for precedence" }
                    }
                }
                
                // Examples section
                if !examples.is_empty() {
                    div {
                        style: "margin-bottom: 16px;",
                        h3 {
                            style: "color: #6a9fb5; font-size: 16px; margin-bottom: 12px;",
                            "💡 Examples"
                        }
                        for example in examples.iter() {
                            div {
                                style: "display: flex; gap: 8px; margin-bottom: 8px; padding-left: 12px;",
                                span { style: "color: #888;", "•" }
                                code {
                                    style: "background: #1e1e1e; color: #fd971f; padding: 4px 8px; border-radius: 3px; font-family: monospace; font-size: 12px;",
                                    "{example}"
                                }
                            }
                        }
                    }
                }
                
                // Close button at bottom
                div {
                    style: "text-align: center; margin-top: 20px;",
                    button {
                        style: "padding: 8px 24px; background: #3a5a8a; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                        onclick: move |_| show_help.set(false),
                        "Close"
                    }
                }
            }
        }
    }
}

/// Build help content for a filter (legacy string format)
pub fn build_filter_help(title: &str, filters: &[(&str, &str)], examples: &[&str]) -> String {
    let mut help = format!("=== {} ===\n\n", title);
    
    help.push_str("FILTERS:\n");
    for (filter, description) in filters {
        help.push_str(&format!("  {} - {}\n", filter, description));
    }
    
    if !examples.is_empty() {
        help.push_str("\nEXAMPLES:\n");
        for example in examples {
            help.push_str(&format!("  {}\n", example));
        }
    }
    
    help.push_str("\nCOMBINATORS:\n");
    help.push_str("  AND - both conditions must match\n");
    help.push_str("  OR  - either condition must match\n");
    help.push_str("  NOT - negates the condition\n");
    help.push_str("  ()  - grouping\n");
    
    help
}
