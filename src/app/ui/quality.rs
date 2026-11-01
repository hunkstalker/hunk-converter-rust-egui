use std::time::Duration;

use eframe::egui::{self, Align, Layout, RichText, Stroke};

use crate::i18n::K;
use crate::theme as T;

use super::super::ENCODE_DELAY_MS;
use super::super::HunkApp;
use super::super::actions::AppAction;

impl HunkApp {
    pub(crate) fn show_quality(&mut self, ui: &mut egui::Ui, actions: &mut Vec<AppAction>) {
        let lossy = self.format.is_lossy();
        let hint = if lossy {
            self.t(K::QualityHint)
        } else {
            self.t(K::LosslessHint)
        };
        let header = ui.horizontal(|ui| {
            ui.label(T::section_label(self.t(K::Quality)));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let value = if lossy {
                    format!("{} %", self.quality)
                } else {
                    self.t(K::Lossless).to_owned()
                };
                ui.label(RichText::new(value).size(T::SMALL_TEXT_SIZE).color(T::TEXT));
            });
        });
        header.response.on_hover_text(hint);

        ui.add_space(3.0);
        ui.spacing_mut().slider_width = (ui.available_width() - 8.0).max(120.0);
        let quality_label = self.t(K::Quality);
        let slider = egui::Slider::new(&mut self.quality, 1..=100)
            .show_value(false)
            .trailing_fill(true);
        ui.scope(|ui| {
            if !lossy {
                let visuals = ui.visuals_mut();
                visuals.selection.bg_fill = T::SURFACE_3;
                visuals.selection.stroke = Stroke::new(1.0, T::TEXT_DISABLED);
                visuals.widgets.inactive.bg_fill = T::SURFACE_3;
                visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, T::TEXT_DISABLED);
                visuals.widgets.noninteractive.bg_fill = T::SURFACE_3;
                visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, T::TEXT_DISABLED);
            }
            let response = ui.add_enabled(lossy, slider).accessible_name(quality_label);
            if response.has_focus() {
                super::widgets::paint_focus_ring(ui.painter(), response.rect, 6);
            }
            response.clone().on_hover_text(hint);
            if lossy && response.changed() {
                actions.push(AppAction::RequestEncode(Some(Duration::from_millis(
                    ENCODE_DELAY_MS,
                ))));
            }
        });
    }
}
