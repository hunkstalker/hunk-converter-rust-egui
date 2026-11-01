use eframe::egui::{
    self, Color32, CornerRadius, CursorIcon, FontId, RichText, Sense, Stroke, StrokeKind, vec2,
};

use crate::domain::{HISTORY_LIMIT, color_codes, hex_rgba};
use crate::i18n::{self, K};
use crate::theme as T;

use super::super::HunkApp;
use super::super::actions::AppAction;
use super::history::{HistorySwatch, paint_history_swatch};
use super::widgets::{HISTORY_COLUMNS, SWATCH_GAP, SWATCH_SIZE, hline, paint_checker};

impl HunkApp {
    pub(crate) fn show_color_panel(&mut self, ui: &mut egui::Ui) -> Vec<AppAction> {
        let mut actions = Vec::new();
        let sample = self.sampled_color;
        let history = self.history.clone();
        let entry_count = history.len();
        let focus_index = self.history_focus.min(entry_count.saturating_sub(1));
        self.history_focus = focus_index;

        T::floating_frame().show(ui, |ui| {
            ui.set_width(ui.available_width());

            ui.label(T::section_label(self.t(K::History)));
            ui.add_space(6.0);

            let content_width = ui.available_width();
            let rows = HISTORY_LIMIT.div_ceil(HISTORY_COLUMNS);
            ui.spacing_mut().item_spacing = vec2(SWATCH_GAP, SWATCH_GAP);
            for row in 0..rows {
                let count = (HISTORY_LIMIT - row * HISTORY_COLUMNS).min(HISTORY_COLUMNS);
                let row_width =
                    count as f32 * SWATCH_SIZE + count.saturating_sub(1) as f32 * SWATCH_GAP;
                let offset = ((content_width - row_width) * 0.5).max(0.0);
                ui.horizontal(|ui| {
                    ui.add_space(offset);
                    for column in 0..count {
                        let index = row * HISTORY_COLUMNS + column;
                        let entry = history.get(index);
                        let (rect, _) =
                            ui.allocate_exact_size(vec2(SWATCH_SIZE, SWATCH_SIZE), Sense::hover());
                        let sense = match entry {
                            None => Sense::hover(),
                            Some(_) if index == focus_index => Sense::click(),
                            Some(_) => Sense::CLICK,
                        };
                        let id = egui::Id::unique(("history_swatch", index));
                        let response = ui.interact(rect, id, sense);
                        if index == focus_index && self.history_focus_pending {
                            response.request_focus();
                            self.history_focus_pending = false;
                        }
                        paint_history_swatch(
                            ui.painter(),
                            rect,
                            HistorySwatch {
                                entry,
                                selected: sample,
                            },
                        );
                        if let Some(entry) = entry {
                            let tooltip = i18n::history_tooltip(
                                self.language,
                                &hex_rgba(entry.rgba),
                                entry.locked,
                            );
                            let selected = sample == Some(entry.rgba);
                            response.widget_info(|| {
                                egui::WidgetInfo::selected(
                                    egui::Role::RadioButton,
                                    true,
                                    selected,
                                    tooltip.clone(),
                                )
                            });
                            let response = response
                                .on_hover_cursor(CursorIcon::PointingHand)
                                .on_hover_text(tooltip);
                            if response.has_focus() {
                                ui.memory_mut(|memory| {
                                    memory.set_focus_lock_filter(
                                        response.id,
                                        egui::EventFilter {
                                            horizontal_arrows: true,
                                            vertical_arrows: true,
                                            ..Default::default()
                                        },
                                    );
                                });
                                super::widgets::paint_focus_ring(ui.painter(), rect, 7);
                            }
                            if response.clicked() {
                                self.history_focus = index;
                                self.history_focus_pending = true;
                                self.sampled_color = Some(entry.rgba);
                                self.sampled_position = None;
                                self.eyedropper_active = false;
                                self.sample_pending_history = false;
                                ui.ctx().memory_mut(|memory| memory.request_focus(id));
                                actions.push(AppAction::RequestRepaint);
                            } else if response.clicked_by(egui::PointerButton::Secondary) {
                                actions.push(AppAction::ToggleHistoryLock(index));
                            }
                            if response.has_focus() {
                                let (left, right, up, down, home, end, lock) = ui.input(|input| {
                                    (
                                        input.key_pressed(egui::Key::ArrowLeft),
                                        input.key_pressed(egui::Key::ArrowRight),
                                        input.key_pressed(egui::Key::ArrowUp),
                                        input.key_pressed(egui::Key::ArrowDown),
                                        input.key_pressed(egui::Key::Home),
                                        input.key_pressed(egui::Key::End),
                                        input.key_pressed(egui::Key::L),
                                    )
                                });
                                let target = if home {
                                    Some(0)
                                } else if end {
                                    Some(entry_count - 1)
                                } else if left {
                                    Some(index.saturating_sub(1))
                                } else if right {
                                    Some((index + 1).min(entry_count - 1))
                                } else if up {
                                    Some(index.saturating_sub(HISTORY_COLUMNS))
                                } else if down {
                                    Some((index + HISTORY_COLUMNS).min(entry_count - 1))
                                } else {
                                    None
                                };
                                if let Some(target) = target {
                                    self.history_focus = target;
                                    self.history_focus_pending = true;
                                    ui.ctx().request_repaint();
                                }
                                if lock {
                                    actions.push(AppAction::ToggleHistoryLock(index));
                                }
                            }
                        }
                    }
                });
                if row + 1 < rows {
                    ui.add_space(SWATCH_GAP);
                }
            }
            ui.add_space(12.0);
            hline(ui);
            ui.add_space(12.0);

            ui.horizontal(|ui| {
                ui.label(T::section_label(self.t(K::Eyedropper)));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (rect, _) = ui.allocate_exact_size(vec2(32.0, 22.0), Sense::hover());
                    let painter = ui.painter();
                    paint_checker(painter, rect);
                    if let Some(rgba) = sample {
                        painter.rect_filled(
                            rect,
                            CornerRadius::same(5),
                            Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3]),
                        );
                    }
                    painter.rect_stroke(
                        rect,
                        CornerRadius::same(5),
                        Stroke::new(1.0, T::BORDER_STRONG),
                        StrokeKind::Inside,
                    );
                });
            });
            ui.add_space(6.0);

            let codes = sample.map(color_codes).unwrap_or_else(|| {
                [
                    ("OKLCH", "—".to_owned()),
                    ("OKLAB", "—".to_owned()),
                    ("HEX", "—".to_owned()),
                    ("RGB", "—".to_owned()),
                    ("RGBA", "—".to_owned()),
                    ("HSL", "—".to_owned()),
                ]
            });
            let control = ui.input(|input| input.modifiers.command);
            let clickable = sample.is_some();
            egui::Grid::new("color_codes_grid")
                .num_columns(2)
                .spacing(vec2(10.0, 4.0))
                .show(ui, |ui| {
                    for (label, value) in codes {
                        ui.label(
                            RichText::new(label)
                                .size(T::SMALL_TEXT_SIZE)
                                .color(T::TEXT_FAINT)
                                .strong(),
                        );
                        if clickable {
                            let tooltip = i18n::copy_code_tooltip(self.language, label);
                            let response =
                                code_value(ui, &value, true).on_hover_text(tooltip.as_str());
                            if response.clicked() {
                                actions.push(AppAction::CopyColor {
                                    label: label.to_owned(),
                                    text: code_copy_text(label, &value, control),
                                });
                            }
                        } else {
                            ui.label(
                                RichText::new(&value)
                                    .font(FontId::monospace(T::SMALL_TEXT_SIZE))
                                    .color(T::TEXT_DISABLED),
                            );
                        }
                        ui.end_row();
                    }
                });
            ui.add_space(2.0);
        });
        actions
    }
}

fn code_value(ui: &mut egui::Ui, value: &str, enabled: bool) -> egui::Response {
    let color = if enabled {
        T::TEXT_MUTED
    } else {
        T::TEXT_DISABLED
    };
    let galley = ui.painter().layout_no_wrap(
        value.to_owned(),
        FontId::monospace(T::SMALL_TEXT_SIZE),
        color,
    );
    let padding = vec2(5.0, 2.0);
    let (rect, response) = ui.allocate_exact_size(
        galley.size() + padding * 2.0,
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    if enabled && response.hovered() {
        ui.painter()
            .rect_filled(rect, CornerRadius::same(5), T::SURFACE_3);
    }
    let text_color = if enabled && response.hovered() {
        T::TEXT
    } else {
        color
    };
    ui.painter().galley(rect.min + padding, galley, text_color);
    if enabled {
        response.on_hover_cursor(CursorIcon::PointingHand)
    } else {
        response
    }
}

fn code_copy_text(label: &str, value: &str, control: bool) -> String {
    if control && label == "HEX" {
        format!("HEX{value}")
    } else {
        value.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ctrl_copy_keeps_css_functions_and_prefixes_hex() {
        assert_eq!(code_copy_text("HEX", "#605342", false), "#605342");
        assert_eq!(code_copy_text("HEX", "#605342", true), "HEX#605342");
        assert_eq!(
            code_copy_text("RGBA", "rgba(96, 83, 66, 1.00)", true),
            "rgba(96, 83, 66, 1.00)"
        );
    }
}
