use eframe::egui::{self, Align, Layout, Margin, RichText};

use crate::i18n::K;
use crate::theme as T;

use super::super::HunkApp;

impl HunkApp {
    pub(crate) fn show_footer(&self, ui: &mut egui::Ui) {
        egui::Panel::bottom("footer")
            .frame(
                eframe::egui::Frame::new()
                    .fill(T::bg())
                    .inner_margin(Margin::symmetric(30, 9)),
            )
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("HUNK Converter · v{}", env!("CARGO_PKG_VERSION")))
                            .size(T::SMALL_TEXT_SIZE)
                            .color(T::text_faint()),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let response = ui
                            .hyperlink_to(
                                RichText::new("github.com/hunkstalker/hunk-converter-rust-egui")
                                    .size(T::SMALL_TEXT_SIZE)
                                    .color(T::text_muted()),
                                "https://github.com/hunkstalker/hunk-converter-rust-egui",
                            )
                            .on_hover_text(self.t(K::OpenRepository));
                        if response.has_focus() {
                            super::widgets::paint_focus_ring(ui.painter(), response.rect, 6);
                        }
                    });
                });
            });
    }
}
