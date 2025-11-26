use dioxus::prelude::*;

/// A filter input component with help tooltip
#[component]
pub fn FilterInput(
    filter_text: Signal<String>,
    filter_error: Signal<Option<String>>,
    placeholder: String,
    help_content: String,
) -> Element {
    let error = filter_error.read().clone();
    let has_error = error.is_some();
    let border_color = if has_error { "#f44336" } else { "#555" };
    
    rsx! {
        div {
            class: "filter-input-container",
            style: "display: flex; align-items: center; gap: 8px; padding: 8px 16px; background: #252525; border-bottom: 1px solid #444;",
            
            // Help icon with tooltip
            div {
                class: "help-icon",
                style: "position: relative; cursor: help;",
                title: "{help_content}",
                
                span {
                    style: "color: #888; font-size: 16px;",
                    "❓"
                }
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
    }
}

/// Build help content for a filter
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
