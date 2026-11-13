use eframe::egui::{self, Align, CornerRadius, Rect, RichText, Stroke, StrokeKind, vec2};

use crate::i18n::K;
use crate::theme as T;

use super::super::HunkApp;
use super::super::actions::AppAction;

impl HunkApp {
    pub(crate) fn paint_empty_canvas(&self, ui: &egui::Ui, rect: Rect) {
        ui.painter().rect(
            rect,
            CornerRadius::same(14),
            T::bg_soft(),
            Stroke::new(1.0, T::border()),
            StrokeKind::Inside,
        );
    }

    pub(crate) fn show_empty_canvas_content(
        &mut self,
        ui: &mut egui::Ui,
        rect: Rect,
    ) -> Vec<AppAction> {
        let mut actions = Vec::new();
        let inner = rect.shrink2(vec2(24.0, 24.0));
        let content_height = 350.0;
        let top_gap = ((inner.height() - content_height) / 2.0).max(0.0);

        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(inner)
                .layout(egui::Layout::top_down(Align::Center)),
            |ui| {
                ui.add_space(top_gap);
                if let Some(icon) = &self.brand_icon {
                    ui.add(
                        egui::Image::new((icon.id(), vec2(84.0, 84.0)))
                            .corner_radius(CornerRadius::same(20)),
                    );
                }
                ui.add_space(16.0);
                ui.label(
                    RichText::new(self.t(K::EmptyTitle))
                        .size(30.0)
                        .color(T::text()),
                );
                ui.add_space(10.0);
                ui.label(
                    RichText::new(self.t(K::EmptySubtitle))
                        .size(12.5)
                        .color(T::text_muted()),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new(self.t(K::EmptyHint))
                        .size(12.5)
                        .color(T::text()),
                );
                ui.add_space(18.0);

                let choose = egui::Button::new(
                    RichText::new(self.t(K::SelectImage))
                        .size(13.5)
                        .color(T::accent_ink())
                        .strong(),
                )
                .fill(T::accent())
                .stroke(Stroke::new(1.0, T::accent()))
                .corner_radius(CornerRadius::same(12))
                .min_size(vec2(230.0, 44.0));
                let response = ui.add(choose);
                if response.has_focus() {
                    super::widgets::paint_focus_ring(ui.painter(), response.rect, 12);
                }
                if response.clicked() {
                    actions.push(AppAction::OpenFileDialog);
                }

                ui.add_space(16.0);
                ui.label(
                    RichText::new("PNG · JPG · WEBP · AVIF · BMP · TGA · TIFF · SVG")
                        .size(9.5)
                        .color(T::text_faint())
                        .strong(),
                );
                ui.add_space(3.0);
                ui.label(
                    RichText::new(self.t(K::PrivacyNote))
                        .size(9.5)
                        .color(T::text_faint()),
                );
            },
        );

        actions
    }
}
