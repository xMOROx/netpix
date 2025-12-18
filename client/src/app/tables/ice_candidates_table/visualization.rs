use super::types::*;
use egui::*;

pub struct IceCandidatesVisualization {
    filter_state: Option<CandidatePairState>,
    selected_pair: Option<super::types::CandidatePairKey>,
    split_ratio: f32,
}

impl Default for IceCandidatesVisualization {
    fn default() -> Self {
        Self {
            filter_state: None,
            selected_pair: None,
            split_ratio: 0.5,
        }
    }
}

impl IceCandidatesVisualization {
    pub fn show(&mut self, ui: &mut Ui, data: &IceCandidatesData) {
        ui.horizontal(|ui| {
            ui.heading("🌐 ICE Connectivity Process");
            ui.separator();

            ui.label(format!("📊 Total Pairs: {}", data.candidate_pairs.len()));
            ui.separator();

            let gathering_count = data
                .candidate_pairs
                .values()
                .filter(|p| p.state == CandidatePairState::Gathering)
                .count();
            let gathered_count = data
                .candidate_pairs
                .values()
                .filter(|p| p.state == CandidatePairState::Gathered)
                .count();
            let checking_count = data
                .candidate_pairs
                .values()
                .filter(|p| p.state == CandidatePairState::InProgress)
                .count();
            let nominated_count = data
                .candidate_pairs
                .values()
                .filter(|p| p.state == CandidatePairState::Nominated)
                .count();

            ui.colored_label(
                Color32::LIGHT_BLUE,
                format!("> Gathering: {}", gathering_count),
            );
            ui.separator();
            ui.colored_label(Color32::GREEN, format!("* Gathered: {}", gathered_count));
            ui.separator();
            ui.colored_label(Color32::YELLOW, format!("~ Checking: {}", checking_count));
            ui.separator();
            ui.colored_label(Color32::GREEN, format!("+ Nominated: {}", nominated_count));
        });

        ui.horizontal(|ui| {
            ui.label("Filter by state:");

            if ui
                .selectable_value(&mut self.filter_state, None, "All")
                .clicked()
            {
                self.filter_state = None;
            }
            for state in &[
                CandidatePairState::Gathering,
                CandidatePairState::Gathered,
                CandidatePairState::InProgress,
                CandidatePairState::Nominated,
                CandidatePairState::Failed,
            ] {
                if ui
                    .selectable_value(&mut self.filter_state, Some(*state), state.label())
                    .clicked()
                {
                    self.filter_state = Some(*state);
                }
            }
        });

        ui.separator();

        if data.candidate_pairs.is_empty() {
            ui.vertical_centered(|ui| {
                ui.heading("No ICE candidate pairs found");
                ui.label("Capture some STUN/ICE traffic to see connectivity visualization");
                ui.add_space(10.0);
                ui.label("Expected ICE phases:");
                ui.label("1 Gathering: STUN requests to STUN server (no ICE attributes)");
                ui.label("2 Signaling: SDP exchange (not visible in STUN packets)");
                ui.label("3 Connectivity Checks: STUN requests with ICE attributes");
            });
            return;
        }

        let mut filtered_pairs: Vec<_> = data
            .candidate_pairs
            .values()
            .filter(|pair| self.filter_state.is_none() || self.filter_state == Some(pair.state))
            .collect();

        filtered_pairs.sort_by_key(|pair| pair.first_seen);

        if filtered_pairs.is_empty() {
            ui.vertical_centered(|ui| {
                ui.heading("No pairs match selected filters");
            });
            return;
        }

        let available_height = ui.available_height();
        let diagram_height = available_height * self.split_ratio;
        let list_height = available_height * (1.0 - self.split_ratio);

        ui.allocate_ui_with_layout(
            Vec2::new(ui.available_width(), diagram_height),
            Layout::top_down(Align::Min),
            |ui| {
                ScrollArea::vertical()
                    .id_salt("connectivity_diagram")
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        if let Some(k) = self.show_connectivity_diagram(ui, &filtered_pairs) {
                            self.selected_pair = Some(k);
                        }
                    });
            },
        );

        // Draggable separator
        let separator_response =
            ui.allocate_response(Vec2::new(ui.available_width(), 8.0), Sense::drag());

        if separator_response.dragged() {
            let delta_y = separator_response.drag_delta().y;
            self.split_ratio = (self.split_ratio + delta_y / available_height).clamp(0.2, 0.8);
        }

        // Draw separator with hover effect
        let separator_color = if separator_response.hovered() {
            Color32::LIGHT_BLUE
        } else {
            Color32::GRAY
        };

        ui.painter().rect_filled(
            separator_response.rect,
            Rounding::ZERO,
            separator_color.linear_multiply(0.3),
        );

        ui.painter().line_segment(
            [
                separator_response.rect.left_center(),
                separator_response.rect.right_center(),
            ],
            Stroke::new(2.0, separator_color),
        );

        ui.allocate_ui_with_layout(
            Vec2::new(ui.available_width(), list_height - 8.0),
            Layout::top_down(Align::Min),
            |ui| {
                ScrollArea::vertical()
                    .id_salt("candidate_pairs_list")
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        if let Some(k) = self.show_candidate_pairs_list(ui, &filtered_pairs) {
                            self.selected_pair = Some(k);
                        }
                    });
            },
        );
    }

    fn show_connectivity_diagram(
        &self,
        ui: &mut Ui,
        pairs: &[&CandidatePairStats],
    ) -> Option<CandidatePairKey> {
        let mut clicked: Option<CandidatePairKey> = None;
        ui.group(|ui| {
            ui.label(RichText::new("Connectivity Overview").strong().size(16.0));

            let available_width = ui.available_width();
            let pair_height = 50.0;
            let total_height = pairs.len() as f32 * pair_height;

            let (response, painter) =
                ui.allocate_painter(Vec2::new(available_width, total_height), Sense::click());
            let rect = response.rect;

            painter.rect_filled(rect, Rounding::same(4.0), Color32::from_rgb(30, 30, 30));

            for (idx, pair) in pairs.iter().enumerate() {
                let y = rect.min.y + (idx as f32 * pair_height) + pair_height / 2.0;

                let local_x = rect.min.x + 100.0;
                painter.circle_filled(Pos2::new(local_x, y), 8.0, pair.state.color());

                let local_text_pos = Pos2::new(local_x + 15.0, y);
                let is_selected = match &self.selected_pair {
                    Some(k) => *k == pair.key,
                    None => false,
                };

                if is_selected {
                    painter.circle_stroke(
                        Pos2::new(local_x, y),
                        12.0,
                        Stroke::new(3.0, Color32::from_rgb(200, 200, 255)),
                    );
                }

                painter.text(
                    local_text_pos,
                    Align2::LEFT_CENTER,
                    &pair.key.local_candidate,
                    FontId::monospace(10.0),
                    Color32::WHITE,
                );

                // Calculate text width for local candidate
                let local_text_width = ui.fonts(|f| {
                    f.layout_no_wrap(
                        pair.key.local_candidate.clone(),
                        FontId::monospace(10.0),
                        Color32::WHITE,
                    )
                    .size()
                    .x
                });

                let remote_x = rect.max.x - 100.0;
                painter.circle_filled(Pos2::new(remote_x, y), 8.0, pair.state.color());

                let remote_text_pos = Pos2::new(remote_x - 15.0, y);
                if is_selected {
                    painter.circle_stroke(
                        Pos2::new(remote_x, y),
                        12.0,
                        Stroke::new(3.0, Color32::from_rgb(200, 200, 255)),
                    );
                }

                painter.text(
                    remote_text_pos,
                    Align2::RIGHT_CENTER,
                    &pair.key.remote_candidate,
                    FontId::monospace(10.0),
                    Color32::WHITE,
                );

                // Calculate text width for remote candidate
                let remote_text_width = ui.fonts(|f| {
                    f.layout_no_wrap(
                        pair.key.remote_candidate.clone(),
                        FontId::monospace(10.0),
                        Color32::WHITE,
                    )
                    .size()
                    .x
                });

                let line_color = pair.state.color();
                let line_width = if pair.state == CandidatePairState::Nominated {
                    3.0
                } else {
                    2.0
                };

                let line_start_x = local_text_pos.x + local_text_width + 10.0;
                let line_end_x = remote_text_pos.x - remote_text_width - 10.0;

                painter.line_segment(
                    [Pos2::new(line_start_x, y), Pos2::new(line_end_x, y)],
                    Stroke::new(line_width, line_color),
                );

                painter.text(
                    Pos2::new((line_start_x + line_end_x) / 2.0, y - 8.0),
                    Align2::CENTER_BOTTOM,
                    pair.state.label(),
                    FontId::monospace(9.0),
                    pair.state.color(),
                );
            }
            if response.clicked() {
                if let Some(pos) = response.hover_pos() {
                    let rel_y = pos.y - rect.min.y;
                    if rel_y >= 0.0 {
                        let idx = (rel_y / pair_height).floor() as usize;
                        if idx < pairs.len() {
                            clicked = Some(pairs[idx].key.clone());
                        }
                    }
                }
            }
        });

        clicked
    }

    fn show_candidate_pairs_list(
        &self,
        ui: &mut Ui,
        pairs: &[&CandidatePairStats],
    ) -> Option<CandidatePairKey> {
        let mut clicked: Option<CandidatePairKey> = None;
        ui.group(|ui| {
            ui.label(RichText::new("Candidate Pairs Details").strong().size(16.0));

            for (idx, pair) in pairs.iter().enumerate() {
                let label = format!(
                    "{} ↔ {}  {}",
                    pair.key.local_candidate,
                    pair.key.remote_candidate,
                    pair.state.label()
                );

                let is_selected = match &self.selected_pair {
                    Some(k) => *k == pair.key,
                    None => false,
                };

                if ui.selectable_label(is_selected, label).clicked() {
                    clicked = Some(pair.key.clone());
                }

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.colored_label(pair.state.color(), pair.state.label());
                        ui.label("-");
                        ui.label(pair.state.description());
                    });

                    ui.horizontal(|ui| {
                        ui.label("Endpoint A:");
                        ui.monospace(
                            RichText::new(&pair.key.local_candidate).color(Color32::LIGHT_BLUE),
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.label("Endpoint B:");
                        ui.monospace(
                            RichText::new(&pair.key.remote_candidate)
                                .color(Color32::from_rgb(100, 200, 255)),
                        );
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new("Packets").strong());
                            ui.label(format!("Checks Sent: {}", pair.checks_sent));
                            ui.label(format!("Checks Received: {}", pair.checks_received));
                            ui.label(format!("Responses Received: {}", pair.responses_received));
                            ui.label(format!("Responses Sent: {}", pair.responses_sent));
                        });

                        ui.separator();

                        ui.vertical(|ui| {
                            ui.label(RichText::new("Status").strong());
                            let color = if pair.failures > 0 {
                                Color32::RED
                            } else {
                                Color32::GREEN
                            };
                            ui.colored_label(color, format!("Failures: {}", pair.failures));
                            ui.label(format!(
                                "Duration: {:.2}s",
                                (pair.last_seen.as_secs_f64() - pair.first_seen.as_secs_f64())
                            ));
                            if pair.avg_rtt_ms > 0.0 {
                                ui.label(format!("Avg RTT: {:.2}ms", pair.avg_rtt_ms));
                            }
                        });
                    });
                });

                if idx < pairs.len() - 1 {
                    ui.separator();
                }
            }
        });
        clicked
    }
}
