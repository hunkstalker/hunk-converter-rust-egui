use eframe::egui::{self, Align, CornerRadius, Frame, Layout, Margin, RichText, Stroke, vec2};

use crate::codec::OutputFormat;
use crate::i18n::{self, K};
use crate::theme as T;

use super::super::HunkApp;
use super::super::actions::AppAction;
use super::format_picker::{FormatSelector, format_selector};
use super::widgets::small_text_button;

pub(super) struct FileCard<'a> {
    pub texture: egui::TextureId,
    pub name: &'a str,
    pub format: &'a str,
    pub file_bytes: u64,
    pub dimensions: (u32, u32),
}

impl HunkApp {
    pub(crate) fn show_sidebar(&mut self, root: &mut egui::Ui) -> Vec<AppAction> {
        let mut actions = Vec::new();
        let Some(source) = self.source.as_ref() else {
            return actions;
        };
        let texture = source.texture.id();
        let name = source.name.clone();
        let format = source.format.clone();
        let file_bytes = source.file_bytes;
        let dimensions = (source.width, source.height);
        let card = FileCard {
            texture,
            name: &name,
            format: &format,
            file_bytes,
            dimensions,
        };

        egui::Panel::left("settings_panel")
            .resizable(false)
            .exact_size(390.0)
            .show_separator_line(false)
            .frame(Frame::new().fill(T::BG).inner_margin(Margin {
                left: 30,
                right: 0,
                top: 14,
                bottom: 0,
            }))
            .show(root, |ui| {
                T::sidebar_frame().show(ui, |ui| {
                    ui.set_min_height(ui.available_height());
                    ui.set_width(ui.available_width());

                    let export_height = 68.0;
                    let scroll_height =
                        (ui.available_height() - self.color_panel_height - export_height - 8.0)
                            .max(160.0);
                    let mut settings_scroll = egui::ScrollArea::vertical()
                        .id_salt("sidebar_scroll")
                        .auto_shrink([false, false])
                        .max_height(scroll_height);
                    if scroll_height < 300.0 {
                        settings_scroll = settings_scroll.scroll_bar_visibility(
                            egui::scroll_area::ScrollBarVisibility::AlwaysVisible,
                        );
                    }
                    settings_scroll.show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        self.show_sidebar_top(ui, &card, &mut actions);
                    });

                    ui.add_space(4.0);
                    let mut color_actions = Vec::new();
                    let color_panel = ui.scope(|ui| {
                        color_actions = self.show_color_panel(ui);
                    });
                    actions.extend(color_actions);
                    self.color_panel_height = color_panel.response.rect.height();
                    let remaining = (ui.available_height() - export_height).max(0.0);
                    ui.add_space(remaining);
                    self.show_export(ui, &mut actions);
                });
            });
        actions
    }

    fn show_sidebar_top(
        &mut self,
        ui: &mut egui::Ui,
        card: &FileCard<'_>,
        actions: &mut Vec<AppAction>,
    ) {
        ui.horizontal(|ui| {
            ui.label(T::eyebrow(self.t(K::Image)));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let response = small_text_button(ui, self.t(K::Change))
                    .on_hover_text(self.t(K::ChangeTooltip));
                if response.clicked() {
                    actions.push(AppAction::OpenFileDialog);
                }
            });
        });
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.add(
                egui::Image::new((card.texture, vec2(48.0, 48.0)))
                    .corner_radius(CornerRadius::same(9)),
            );
            ui.add_space(4.0);
            ui.vertical(|ui| {
                ui.add_space(3.0);
                ui.label(RichText::new(card.name).size(12.5).color(T::TEXT).strong());
                ui.label(
                    RichText::new(format!(
                        "{} · {} × {} px · {}",
                        card.format,
                        card.dimensions.0,
                        card.dimensions.1,
                        i18n::localized_bytes(self.language, card.file_bytes)
                    ))
                    .size(9.5)
                    .color(T::TEXT_MUTED),
                );
            });
        });

        ui.add_space(6.0);
        super::widgets::hline(ui);
        ui.add_space(6.0);
        ui.label(T::eyebrow(self.t(K::OutputFormat)));
        ui.add_space(6.0);

        if let Some(selection) = format_selector(
            ui,
            FormatSelector {
                selection: self.format_selection,
                current: self.format,
                language: self.language,
                source_format: Some(card.format),
            },
        ) {
            let changed = self.set_format_selection(selection);
            actions.push(AppAction::PersistSettings);
            if changed {
                actions.push(AppAction::RequestEncode(None));
            }
        }

        ui.add_space(10.0);
        if self.format == OutputFormat::Ico {
            self.show_ico_sizes(ui, actions);
        } else {
            self.show_quality(ui, actions);
        }

        self.show_preprocess(ui, card.dimensions, actions);
        self.show_advanced(ui, actions);
    }

    fn show_preprocess(
        &mut self,
        ui: &mut egui::Ui,
        source_dimensions: (u32, u32),
        actions: &mut Vec<AppAction>,
    ) {
        let before = self.advanced.preprocess.clone();
        let title = T::eyebrow(self.t(K::Preprocess));
        let resize_label = self.t(K::Resize);
        let width_label = self.t(K::Width);
        let height_label = self.t(K::Height);
        let keep_aspect_label = self.t(K::KeepAspect);
        let rotate_label = self.t(K::Rotate);
        let palette_label = self.t(K::Palette);
        let colors_label = self.t(K::Colors);
        ui.add_space(10.0);
        egui::CollapsingHeader::new(title)
            .default_open(false)
            .show(ui, |ui| {
                ui.checkbox(&mut self.advanced.preprocess.resize_enabled, resize_label);
                if self.advanced.preprocess.resize_enabled {
                    if self.advanced.preprocess.width == 0 {
                        self.advanced.preprocess.width = source_dimensions.0;
                    }
                    if self.advanced.preprocess.height == 0 {
                        self.advanced.preprocess.height = source_dimensions.1;
                    }
                    ui.horizontal(|ui| {
                        ui.label(width_label);
                        ui.add(
                            egui::DragValue::new(&mut self.advanced.preprocess.width)
                                .range(1..=16_384)
                                .speed(1),
                        );
                        ui.label(height_label);
                        ui.add(
                            egui::DragValue::new(&mut self.advanced.preprocess.height)
                                .range(1..=16_384)
                                .speed(1),
                        );
                    });
                    ui.checkbox(&mut self.advanced.preprocess.keep_aspect, keep_aspect_label);
                }

                ui.horizontal(|ui| {
                    ui.label(rotate_label);
                    for (degrees, label) in
                        [(0u16, "0°"), (90, "90°"), (180, "180°"), (270, "270°")]
                    {
                        ui.selectable_value(&mut self.advanced.preprocess.rotation, degrees, label);
                    }
                });

                let mut palette_enabled = self.advanced.preprocess.palette_colors.is_some();
                if ui.checkbox(&mut palette_enabled, palette_label).changed() {
                    self.advanced.preprocess.palette_colors = palette_enabled.then_some(128);
                }
                if let Some(colors) = &mut self.advanced.preprocess.palette_colors {
                    ui.horizontal(|ui| {
                        ui.label(colors_label);
                        ui.add(egui::DragValue::new(colors).range(2..=256).speed(1));
                    });
                }
            });

        if self.advanced.preprocess == before {
            return;
        }
        let geometry_changed = self.advanced.preprocess.resize_enabled != before.resize_enabled
            || self.advanced.preprocess.width != before.width
            || self.advanced.preprocess.height != before.height
            || self.advanced.preprocess.keep_aspect != before.keep_aspect
            || self.advanced.preprocess.rotation != before.rotation;
        if geometry_changed {
            self.refresh_preprocessed_source(ui.ctx());
        } else if self.advanced.preprocess.palette_colors != before.palette_colors {
            actions.push(AppAction::RequestEncode(None));
        }
        actions.push(AppAction::PersistSettings);
    }

    fn show_advanced(&mut self, ui: &mut egui::Ui, actions: &mut Vec<AppAction>) {
        if self.format == OutputFormat::Ico || self.format == OutputFormat::Jxl {
            return;
        }
        let before = self.advanced.clone();
        let advanced_label = self.t(K::Advanced).to_uppercase();
        let progressive_label = self.t(K::Progressive);
        let subsampling_label = self.t(K::Subsampling);
        let auto_label = self.t(K::SubsamplingAuto);
        let level_label = self.t(K::PngLevel);
        let fast_label = self.t(K::PngFast);
        let balanced_label = self.t(K::PngBalanced);
        let maximum_label = self.t(K::PngMaximum);
        let interlaced_label = self.t(K::Interlaced);
        let lossless_label = self.t(K::WebpLossless);
        let method_label = self.t(K::WebpMethod);
        let alpha_label = self.t(K::AlphaQuality);
        let speed_label = self.t(K::AvifSpeed);
        let reset_label = self.t(K::ResetOptions);

        let summary = match self.format {
            OutputFormat::Jpg => format!(
                "{} · {}",
                if self.advanced.jpg_progressive {
                    progressive_label
                } else {
                    "Baseline"
                },
                match self.advanced.jpg_subsampling {
                    1 => "4:2:0",
                    2 => "4:2:2",
                    3 => "4:4:4",
                    _ => auto_label,
                }
            ),
            OutputFormat::Png => format!(
                "{} · {}",
                match self.advanced.png_level {
                    0 | 1 => fast_label,
                    6 => maximum_label,
                    _ => balanced_label,
                },
                if self.advanced.png_interlace {
                    interlaced_label
                } else {
                    "—"
                }
            ),
            OutputFormat::Webp => {
                if self.advanced.webp_lossless {
                    format!(
                        "{lossless_label} · {} {}",
                        method_label, self.advanced.webp_method
                    )
                } else {
                    format!("{} {}", method_label, self.advanced.webp_method)
                }
            }
            OutputFormat::Avif => format!("{} {}", speed_label, self.advanced.avif_speed),
            OutputFormat::Ico | OutputFormat::Jxl => String::new(),
        };

        ui.add_space(6.0);
        egui::CollapsingHeader::new(format!("{advanced_label} · {summary}"))
            .default_open(false)
            .show(ui, |ui| {
                match self.format {
                    OutputFormat::Jpg => {
                        ui.checkbox(&mut self.advanced.jpg_progressive, progressive_label);
                        ui.horizontal(|ui| {
                            ui.label(subsampling_label);
                            let selected = match self.advanced.jpg_subsampling {
                                1 => "4:2:0",
                                2 => "4:2:2",
                                3 => "4:4:4",
                                _ => auto_label,
                            };
                            egui::ComboBox::from_id_salt("jpg_subsampling")
                                .selected_text(selected)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.advanced.jpg_subsampling,
                                        0,
                                        auto_label,
                                    );
                                    ui.selectable_value(
                                        &mut self.advanced.jpg_subsampling,
                                        1,
                                        "4:2:0",
                                    );
                                    ui.selectable_value(
                                        &mut self.advanced.jpg_subsampling,
                                        2,
                                        "4:2:2",
                                    );
                                    ui.selectable_value(
                                        &mut self.advanced.jpg_subsampling,
                                        3,
                                        "4:4:4",
                                    );
                                });
                        });
                    }
                    OutputFormat::Png => {
                        ui.horizontal(|ui| {
                            ui.label(level_label);
                            egui::ComboBox::from_id_salt("png_level")
                                .selected_text(match self.advanced.png_level {
                                    0 | 1 => fast_label,
                                    6 => maximum_label,
                                    _ => balanced_label,
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.advanced.png_level,
                                        0,
                                        fast_label,
                                    );
                                    ui.selectable_value(
                                        &mut self.advanced.png_level,
                                        2,
                                        balanced_label,
                                    );
                                    ui.selectable_value(
                                        &mut self.advanced.png_level,
                                        6,
                                        maximum_label,
                                    );
                                });
                        });
                        ui.checkbox(&mut self.advanced.png_interlace, interlaced_label);
                    }
                    OutputFormat::Webp => {
                        ui.checkbox(&mut self.advanced.webp_lossless, lossless_label);
                        ui.horizontal(|ui| {
                            ui.label(method_label);
                            ui.add(
                                egui::DragValue::new(&mut self.advanced.webp_method)
                                    .range(0..=6)
                                    .speed(1),
                            );
                        });
                        if !self.advanced.webp_lossless {
                            ui.horizontal(|ui| {
                                ui.label(alpha_label);
                                ui.add(
                                    egui::DragValue::new(&mut self.advanced.webp_alpha_quality)
                                        .range(0..=100)
                                        .speed(1),
                                );
                            });
                        }
                    }
                    OutputFormat::Avif => {
                        ui.horizontal(|ui| {
                            ui.label(speed_label);
                            ui.add(
                                egui::DragValue::new(&mut self.advanced.avif_speed)
                                    .range(1..=10)
                                    .speed(1),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label(alpha_label);
                            ui.add(
                                egui::DragValue::new(&mut self.advanced.avif_alpha_quality)
                                    .range(0..=100)
                                    .speed(1),
                            );
                        });
                    }
                    OutputFormat::Ico | OutputFormat::Jxl => {}
                }
                if ui.small_button(reset_label).clicked() {
                    let preprocess = self.advanced.preprocess.clone();
                    self.advanced = crate::settings::AdvancedSettings::default();
                    self.advanced.preprocess = preprocess;
                }
            });

        if self.advanced != before {
            actions.push(AppAction::PersistSettings);
            actions.push(AppAction::RequestEncode(None));
        }
    }

    pub(crate) fn show_export(&mut self, ui: &mut egui::Ui, actions: &mut Vec<AppAction>) {
        let enabled = self.can_export();
        let label = i18n::export_label(self.language, self.format.extension());
        let button = egui::Button::new(
            RichText::new(&label)
                .size(13.0)
                .color(if enabled {
                    T::ACCENT_INK
                } else {
                    T::TEXT_DISABLED
                })
                .strong(),
        )
        .fill(if enabled { T::ACCENT } else { T::SURFACE_3 })
        .stroke(Stroke::new(
            1.0,
            if enabled { T::ACCENT } else { T::BORDER },
        ))
        .corner_radius(CornerRadius::same(12))
        .min_size(vec2(ui.available_width(), 43.0));
        let response = ui.add(button);
        if response.has_focus() {
            super::widgets::paint_focus_ring(ui.painter(), response.rect, 12);
        }
        if response.clicked() && enabled {
            actions.push(AppAction::Export);
        }
        ui.add_space(4.0);
        let hint = if self.busy() {
            self.t(K::Calculating)
        } else {
            self.t(K::ExportHint)
        };
        ui.label(RichText::new(hint).size(9.0).color(T::TEXT_FAINT));
    }
}
