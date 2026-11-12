use std::time::{Duration, Instant};

use eframe::egui::{self, Align2, CornerRadius, FontId, Id, Sense, Stroke, StrokeKind, vec2};

use crate::theme as T;

use super::super::HunkApp;

impl HunkApp {
    pub(crate) fn show_toast(&mut self, ctx: &egui::Context) {
        let Some(toast) = &self.toast else {
            return;
        };
        let now = Instant::now();
        if now >= toast.until {
            self.toast = None;
            return;
        }
        let remaining = toast.until - now;
        let text_color = if toast.error { T::danger() } else { T::text() };
        let fill = if toast.error {
            T::toast_error_bg()
        } else {
            T::surface_3()
        };
        let stroke_color = if toast.error {
            T::toast_error_border()
        } else {
            T::border_strong()
        };
        let message = toast.message.clone();

        egui::Area::new(Id::unique("toast"))
            .anchor(Align2::CENTER_TOP, vec2(0.0, 68.0))
            .order(egui::Order::Foreground)
            .interactable(false)
            .show(ctx, |ui| {
                ui.scope_builder(
                    egui::UiBuilder::new().accessibility_role(egui::Role::Alert),
                    |ui| {
                        let galley = ui.painter().layout(
                            message,
                            FontId::proportional(13.5),
                            text_color,
                            560.0,
                        );
                        let padding = vec2(20.0, 12.0);
                        let (rect, _) =
                            ui.allocate_exact_size(galley.size() + padding * 2.0, Sense::hover());
                        let painter = ui.painter();
                        painter.rect(
                            rect,
                            CornerRadius::same(12),
                            fill,
                            Stroke::new(1.0, stroke_color),
                            StrokeKind::Inside,
                        );
                        painter.galley(rect.min + padding, galley, text_color);
                    },
                );
            });
        ctx.request_repaint_after(remaining.min(Duration::from_millis(100)));
    }
}
