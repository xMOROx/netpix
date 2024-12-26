use egui::{ComboBox, Label, TextWrapMode, Ui, Widget};

use crate::app::{side_button, tab::Tab, App, SOURCE_KEY, TAB_KEY};

pub struct AppSidePanel {}
pub struct AppTopBar {}
pub struct AppBottomBar {}

impl AppSidePanel {
    pub fn build(app: &mut App, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = (0.0, 8.0).into();
        for (_text_style, font_id) in style.text_styles.iter_mut() {
            font_id.size = 20.0;
        }

        egui::SidePanel::left("side_panel")
            .resizable(false)
            .default_width(32.0)
            .show(ctx, |ui| {
                ui.set_style(style);
                ui.vertical_centered(|ui| {
                    ui.add_space(6.0);

                    let button = side_button("▶");
                    let resp = ui
                        .add_enabled(!app.is_capturing, button)
                        .on_hover_text("Resume packet capturing");
                    if resp.clicked() {
                        app.is_capturing = true
                    }

                    let button = side_button("⏸");
                    let resp = ui
                        .add_enabled(app.is_capturing, button)
                        .on_hover_text("Stop packet capturing");
                    if resp.clicked() {
                        app.is_capturing = false
                    }

                    let button = side_button("🗑");
                    let resp = ui
                        .add(button)
                        .on_hover_text("Discard previously captured packets");
                    if resp.clicked() {
                        app.streams.borrow_mut().clear();
                    }

                    //TODO: implement more optimal way to do that - with lots of packages it is too much for wasm to handle this
                    let button = side_button("↻");
                    let resp = ui
                        .add(button)
                        .on_hover_text("Refetch all previously captured packets");
                    if resp.clicked() {
                        app.streams.borrow_mut().clear();
                        app.refetch_packets()
                    }
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(8.0);

                    egui::widgets::global_theme_preference_switch(ui);
                });
            });
    }
}

impl AppTopBar {
    pub fn build(app: &mut App, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let selected = app.tab.display_name();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                Self::build_dropdown_source(app, ui, frame);
                ui.separator();
                Self::build_menu_button(app, ui, frame);
                Label::new(selected).ui(ui);
            });
        });
    }

    fn build_menu_button(app: &mut App, ui: &mut Ui, frame: &mut eframe::Frame) {
        ui.menu_button("📑 Open tabs", |ui| {
            ui.heading("Tabs");

            let menu_sections = Tab::sections();

            for (label, sections) in menu_sections {
                ui.menu_button(label, |ui| {
                    for tab in sections {
                        let resp = ui.selectable_value(&mut app.tab, tab, tab.display_name());
                        if resp.clicked() {
                            if let Some(storage) = frame.storage_mut() {
                                storage.set_string(TAB_KEY, tab.to_string());
                            }
                        }
                    }
                });
            }
        });
    }

    fn build_dropdown_source(app: &mut App, ui: &mut Ui, frame: &mut eframe::Frame) {
        let selected = match app.selected_source {
            Some(ref source) => source.to_string(),
            None => "Select packets source...".to_string(),
        };

        ComboBox::from_id_salt("source_picker")
            .width(300.0)
            .wrap_mode(TextWrapMode::Extend)
            .selected_text(selected)
            .show_ui(ui, |ui| {
                let mut was_changed = false;

                for source in app.sources.iter() {
                    let resp = ui.selectable_value(
                        &mut app.selected_source,
                        Some(source.clone()),
                        source.to_string(),
                    );
                    if resp.clicked() {
                        was_changed = true;
                    }
                }

                if was_changed {
                    app.streams.borrow_mut().clear();
                    app.change_source_request();
                    if let Some(storage) = frame.storage_mut() {
                        let source = app.selected_source.as_ref().unwrap();
                        storage.set_string(SOURCE_KEY, source.to_string());
                    }
                }
            });
    }
}

impl AppBottomBar {
    pub fn build(app: &App, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                let streams = app.streams.borrow();
                let count = streams.packets.id_count();
                let count_label = format!("Packets: {}", count);

                let captured_count = streams.packets.len();
                let captured_label = format!("Captured: {}", captured_count);

                let discharged_label = format!("Discharged: {}", app.discharged_count);
                let overwritten_label = format!("Overwritten: {}", app.overwritten_count);
                let label = format!(
                    "{} • {} • {} • {}",
                    count_label, captured_label, discharged_label, overwritten_label
                );
                ui.label(label);
            });
        });
    }
}
