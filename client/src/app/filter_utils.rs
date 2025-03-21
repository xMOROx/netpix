use egui::{Align2, Color32, RichText, TextEdit, Vec2};

use crate::filter_system::ParseError;

use super::{
    BASIC_FILTERS_TITLE, CLOSE_BUTTON_TEXT, EXAMPLES_TITLE, FILTER_ERROR_TITLE,
    FILTER_ERROR_WINDOW_OFFSET, FILTER_HELP_TITLE, FILTER_HINT_TEXT, HEADER_BOTTOM_MARGIN,
    HEADER_TEXT_SIZE, HELP_BUTTON_TEXT, HELP_BUTTON_TOOLTIP, HELP_WINDOW_MIN_WIDTH,
    LOGICAL_OPERATORS, LOGICAL_OPERATORS_TITLE, SECTION_BOTTOM_MARGIN, SECTION_HEADER_COLOR,
    SECTION_HEADER_TEXT_SIZE, VERTICAL_SPACING,
};

#[derive(Debug, Clone)]
pub struct FilterInput {
    pub filter_buffer: String,
    pub filter_error: Option<ParseError>,
    pub show_filter_help: bool,
    help_content: FilterHelpContent,
}

#[derive(Debug, Clone)]
pub struct FilterHelpContent {
    title: String,
    basic_filters: Vec<(String, String)>,
    examples: Vec<String>,
}

pub struct FilterHelpBuilder {
    content: FilterHelpContent,
}

impl FilterInput {
    pub fn new(help_content: FilterHelpContent) -> Self {
        Self {
            filter_buffer: String::new(),
            filter_error: None,
            show_filter_help: false,
            help_content,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) -> bool {
        let mut changed = false;

        egui::TopBottomPanel::top("filter_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let text_edit = TextEdit::singleline(&mut self.filter_buffer)
                    .font(egui::style::TextStyle::Monospace)
                    .desired_width(f32::INFINITY)
                    .hint_text(FILTER_HINT_TEXT);

                let response = ui
                    .small_button(HELP_BUTTON_TEXT)
                    .on_hover_text(HELP_BUTTON_TOOLTIP);

                changed = ui.add(text_edit).changed();
                self.show_filter_help ^= response.clicked();
            });
        });

        if let Some(error) = &self.filter_error {
            let modal = egui::Window::new(FILTER_ERROR_TITLE)
                .anchor(Align2::RIGHT_BOTTOM, FILTER_ERROR_WINDOW_OFFSET)
                .collapsible(true)
                .resizable(false);

            modal.show(ctx, |ui| {
                ui.colored_label(Color32::RED, format!("{}", error));
            });
        }

        if self.show_filter_help {
            let modal = egui::Window::new(FILTER_HELP_TITLE)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .resizable(false)
                .collapsible(false)
                .min_width(HELP_WINDOW_MIN_WIDTH);

            modal.show(ctx, |ui| {
                // Main header
                ui.heading(
                    RichText::new(&self.help_content.title)
                        .size(HEADER_TEXT_SIZE)
                        .strong(),
                );
                ui.add_space(HEADER_BOTTOM_MARGIN);

                // Basic Filters section
                ui.heading(
                    RichText::new(BASIC_FILTERS_TITLE)
                        .color(SECTION_HEADER_COLOR)
                        .size(SECTION_HEADER_TEXT_SIZE),
                );
                ui.add_space(SECTION_BOTTOM_MARGIN);
                for (filter, desc) in &self.help_content.basic_filters {
                    ui.horizontal(|ui| {
                        ui.label("•");
                        ui.label(RichText::new(filter).strong().monospace());
                        ui.label(RichText::new(format!("- {}", desc)));
                    });
                }

                ui.add_space(VERTICAL_SPACING);
                // Logical Operators section
                ui.heading(
                    RichText::new(LOGICAL_OPERATORS_TITLE)
                        .color(SECTION_HEADER_COLOR)
                        .size(SECTION_HEADER_TEXT_SIZE),
                );
                ui.add_space(SECTION_BOTTOM_MARGIN);
                for (op, desc) in LOGICAL_OPERATORS {
                    ui.horizontal(|ui| {
                        ui.label("•");
                        ui.label(RichText::new(op).strong().monospace());
                        ui.label(RichText::new(format!("- {}", desc)));
                    });
                }

                ui.add_space(VERTICAL_SPACING);
                // Examples section
                ui.heading(
                    RichText::new(EXAMPLES_TITLE)
                        .color(SECTION_HEADER_COLOR)
                        .size(SECTION_HEADER_TEXT_SIZE),
                );
                ui.add_space(SECTION_BOTTOM_MARGIN);
                for example in &self.help_content.examples {
                    ui.horizontal(|ui| {
                        ui.label("•");
                        ui.label(RichText::new(example).strong().monospace());
                    });
                }

                ui.add_space(VERTICAL_SPACING);
                if ui.button(CLOSE_BUTTON_TEXT).clicked() {
                    self.show_filter_help = false;
                }
            });
        }

        changed
    }

    pub fn get_filter(&self) -> &str {
        self.filter_buffer.trim()
    }

    pub fn set_error(&mut self, error: Option<ParseError>) {
        self.filter_error = error;
    }

    pub fn clear(&mut self) {
        self.filter_buffer.clear();
        self.filter_error = None;
    }

    pub fn get_error(&self) -> Option<&ParseError> {
        self.filter_error.as_ref()
    }
}

impl FilterHelpContent {
    pub fn builder(title: impl Into<String>) -> FilterHelpBuilder {
        FilterHelpBuilder {
            content: FilterHelpContent {
                title: title.into(),
                basic_filters: Vec::new(),
                examples: Vec::new(),
            },
        }
    }
}

impl FilterHelpBuilder {
    pub fn filter(mut self, filter: impl Into<String>, desc: impl Into<String>) -> Self {
        self.content
            .basic_filters
            .push((filter.into(), desc.into()));
        self
    }

    pub fn example(mut self, example: impl Into<String>) -> Self {
        self.content.examples.push(example.into());
        self
    }

    pub fn build(self) -> FilterHelpContent {
        self.content
    }
}
