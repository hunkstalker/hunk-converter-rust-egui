use std::time::Duration;

use eframe::egui::{self, Align, CornerRadius, Frame, Layout, Margin, RichText, Stroke, vec2};

use crate::codec::OutputFormat;
use crate::domain::{linked_height, rotated_size};
use crate::i18n::{self, K};
use crate::theme as T;

use super::super::actions::AppAction;
use super::super::{ENCODE_DELAY_MS, HunkApp, PREPROCESS_DELAY_MS};
use super::dimension_fields::{DimensionLabels, dimension_fields};
use super::format_picker::{FormatSelector, format_grid};
use super::widgets::secondary_text_button;

pub(super) struct FileCard<'a> {
    pub texture: egui::TextureId,
    pub name: &'a str,
    pub format: &'a str,
    pub file_bytes: u64,
    pub dimensions: (u32, u32),
    pub original_dimensions: (u32, u32),
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
        let dimensions = self.crop_output_dimensions(source.width, source.height);
        let original_dimensions = source.original.dimensions();
        let card = FileCard {
            texture,
            name: &name,
            format: &format,
            file_bytes,
            dimensions,
            original_dimensions,
        };

        egui::Panel::right("settings_panel")
            .resizable(false)
            .exact_size(340.0)
            .show_separator_line(false)
            .frame(Frame::new().fill(T::BG).inner_margin(Margin {
                left: 0,
                right: 12,
                top: 56,
                bottom: 16,
            }))
            .show(root, |ui| {
                T::sidebar_frame().show(ui, |ui| {
                    ui.set_min_height(ui.available_height());
                    ui.set_width(ui.available_width());

                    let footer_height = 84.0;
                    let scroll_height = (ui.available_height() - footer_height - 8.0).max(200.0);
                    egui::ScrollArea::vertical()
                        .id_salt("sidebar_scroll")
                        .auto_shrink([false, false])
                        .max_height(scroll_height)
                        .scroll_bar_visibility(
                            egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded,
                        )
                        .show(ui, |ui| {
                            ui.set_width((ui.available_width() - 6.0).max(0.0));
                            self.show_sidebar_top(ui, &card, &mut actions);
                        });

                    ui.add_space(4.0);
                    let remaining = (ui.available_height() - footer_height).max(0.0);
                    ui.add_space(remaining);
                    self.show_status(ui);
                    ui.add_space(4.0);
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
        ui.label(T::section_label(self.t(K::Image)));
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.add(
                egui::Image::new((card.texture, vec2(48.0, 48.0)))
                    .corner_radius(CornerRadius::same(9)),
            );
            ui.add_space(4.0);
            let text_width = ui.available_width().max(40.0);
            let display_name = fitted_filename(ui, card.name, text_width);
            let response = ui.add_sized(
                [text_width, 48.0],
                egui::Label::new(
                    RichText::new(display_name)
                        .size(T::SMALL_TEXT_SIZE)
                        .color(T::TEXT)
                        .strong(),
                )
                .wrap(),
            );
            response.on_hover_text(card.name);
        });
        ui.add_space(4.0);
        ui.label(
            RichText::new(format!(
                "{} · {} × {} px · {}",
                card.format,
                card.dimensions.0,
                card.dimensions.1,
                i18n::localized_bytes(self.language, card.file_bytes)
            ))
            .size(T::SMALL_TEXT_SIZE)
            .color(T::TEXT_MUTED),
        );
        ui.add_space(10.0);
        let response =
            secondary_text_button(ui, self.t(K::Change)).on_hover_text(self.t(K::ChangeTooltip));
        if response.clicked() {
            actions.push(AppAction::OpenFileDialog);
        }
        ui.add_space(10.0);
        super::widgets::hline(ui);
        ui.add_space(6.0);
        ui.label(T::section_label(self.t(K::OutputFormat)));
        ui.add_space(6.0);

        if let Some(selection) = format_grid(
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

        self.show_preprocess(ui, card.original_dimensions, actions);
        self.show_advanced(ui, actions);
    }

    fn show_preprocess(
        &mut self,
        ui: &mut egui::Ui,
        original_dimensions: (u32, u32),
        actions: &mut Vec<AppAction>,
    ) {
        let before = self.advanced.preprocess.clone();
        let title = self.t(K::Preprocess);
        let resize_label = self.t(K::Resize);
        let width_label = self.t(K::Width);
        let height_label = self.t(K::Height);
        let keep_aspect_label = self.t(K::KeepAspect);
        let palette_label = self.t(K::Palette);
        let colors_label = self.t(K::Colors);
        let reset_label = self.t(K::ResetOptions);
        let dimension_hint = self.t(K::DimensionFieldHint);
        let mut dimension_response = None;
        let mut reset = false;

        ui.add_space(8.0);
        let preprocess_section = section_heading("preprocess_section", title).show(ui, |ui| {
            let body_width = ui.available_width();
            T::soft_frame().show(ui, |ui| {
                let inner_width = (body_width - 30.0).max(0.0);
                ui.set_width(inner_width);
                ui.set_max_width(inner_width);
                ui.spacing_mut().item_spacing.y = 7.0;

                ui.checkbox(&mut self.advanced.preprocess.resize_enabled, resize_label);

                if self.advanced.preprocess.resize_enabled {
                    let (rotated_width, rotated_height) = rotated_size(
                        original_dimensions.0,
                        original_dimensions.1,
                        self.advanced.preprocess.rotation,
                    );
                    if self.advanced.preprocess.width == 0 {
                        self.advanced.preprocess.width = rotated_width;
                    }
                    if self.advanced.preprocess.height == 0 {
                        self.advanced.preprocess.height = rotated_height;
                    }
                    let aspect = rotated_width as f32 / rotated_height.max(1) as f32;
                    if ui
                        .checkbox(&mut self.advanced.preprocess.keep_aspect, keep_aspect_label)
                        .changed()
                        && self.advanced.preprocess.keep_aspect
                    {
                        self.advanced.preprocess.height =
                            linked_height(self.advanced.preprocess.width, aspect, 16_384);
                    }
                    dimension_response = Some(dimension_fields(
                        ui,
                        &mut self.advanced.preprocess.width,
                        &mut self.advanced.preprocess.height,
                        aspect,
                        self.advanced.preprocess.keep_aspect,
                        DimensionLabels {
                            width: width_label,
                            height: height_label,
                            hint: dimension_hint,
                        },
                    ));
                }

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

                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.small_button(reset_label).clicked() {
                            reset = true;
                        }
                    });
                });
            });
        });
        if preprocess_section.header_response.clicked() {
            request_section_scroll(ui, "preprocess", preprocess_section.header_response.rect);
        }
        apply_pending_section_scroll(ui, "preprocess", &preprocess_section);
        if reset {
            let mut options = crate::domain::PreprocessOptions::default();
            options.reset_for_source(original_dimensions.0, original_dimensions.1);
            self.advanced.preprocess = options;
        }

        if self.advanced.preprocess == before {
            return;
        }
        let resize_toggled = self.advanced.preprocess.resize_enabled != before.resize_enabled;
        let keep_aspect_toggled = self.advanced.preprocess.keep_aspect != before.keep_aspect;
        let geometry_changed = resize_toggled
            || self.advanced.preprocess.width != before.width
            || self.advanced.preprocess.height != before.height
            || keep_aspect_toggled;
        if geometry_changed {
            let immediate = reset || resize_toggled || keep_aspect_toggled;
            let delayed = dimension_response.is_some_and(|response| {
                response.dragging
                    || (!response.committed && (response.width_changed || response.height_changed))
            });
            let delay = if immediate || !delayed {
                None
            } else {
                Some(Duration::from_millis(PREPROCESS_DELAY_MS))
            };
            actions.push(AppAction::RequestPreprocess(delay));
        } else if self.advanced.preprocess.palette_colors != before.palette_colors {
            actions.push(AppAction::RequestEncode(Some(Duration::from_millis(
                ENCODE_DELAY_MS,
            ))));
        }
    }

    fn show_advanced(&mut self, ui: &mut egui::Ui, actions: &mut Vec<AppAction>) {
        if self.format == OutputFormat::Ico || self.format == OutputFormat::Jxl {
            return;
        }
        let before = self.advanced.clone();
        let title = self.t(K::Advanced);
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

        ui.add_space(8.0);
        let advanced_section = section_heading("advanced_section", title).show(ui, |ui| {
            let body_width = ui.available_width();
            T::soft_frame().show(ui, |ui| {
                let inner_width = (body_width - 30.0).max(0.0);
                ui.set_width(inner_width);
                ui.set_max_width(inner_width);
                ui.spacing_mut().item_spacing.y = 7.0;
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
                            let current = self.advanced.jpg_subsampling;
                            egui::ComboBox::from_id_salt("jpg_subsampling")
                                .selected_text(selected)
                                .width(ui.available_width())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.advanced.jpg_subsampling,
                                        0,
                                        choice_text(auto_label, current == 0),
                                    );
                                    ui.selectable_value(
                                        &mut self.advanced.jpg_subsampling,
                                        1,
                                        choice_text("4:2:0", current == 1),
                                    );
                                    ui.selectable_value(
                                        &mut self.advanced.jpg_subsampling,
                                        2,
                                        choice_text("4:2:2", current == 2),
                                    );
                                    ui.selectable_value(
                                        &mut self.advanced.jpg_subsampling,
                                        3,
                                        choice_text("4:4:4", current == 3),
                                    );
                                });
                        });
                    }
                    OutputFormat::Png => {
                        ui.horizontal(|ui| {
                            ui.label(level_label);
                            let current = self.advanced.png_level;
                            egui::ComboBox::from_id_salt("png_level")
                                .selected_text(match self.advanced.png_level {
                                    0 | 1 => fast_label,
                                    6 => maximum_label,
                                    _ => balanced_label,
                                })
                                .width(ui.available_width())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.advanced.png_level,
                                        0,
                                        choice_text(fast_label, current == 0),
                                    );
                                    ui.selectable_value(
                                        &mut self.advanced.png_level,
                                        2,
                                        choice_text(balanced_label, current == 2),
                                    );
                                    ui.selectable_value(
                                        &mut self.advanced.png_level,
                                        6,
                                        choice_text(maximum_label, current == 6),
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
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.small_button(reset_label).clicked() {
                            let preprocess = self.advanced.preprocess.clone();
                            self.advanced = crate::settings::AdvancedSettings::default();
                            self.advanced.preprocess = preprocess;
                        }
                    });
                });
            });
        });
        if advanced_section.header_response.clicked() {
            request_section_scroll(ui, "advanced", advanced_section.header_response.rect);
        }
        apply_pending_section_scroll(ui, "advanced", &advanced_section);
        if self.advanced != before {
            actions.push(AppAction::RequestEncode(Some(Duration::from_millis(
                ENCODE_DELAY_MS,
            ))));
        }
    }

    fn show_status(&mut self, ui: &mut egui::Ui) {
        let source_size = self.source.as_ref().map(|source| source.file_bytes);
        let output_size = self
            .result
            .as_ref()
            .filter(|result| result.format == self.format && !self.busy())
            .map(|result| result.size);

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 5.0;
            ui.label(T::section_label(self.t(K::Original)));
            ui.label(
                RichText::new(
                    source_size
                        .map(|bytes| i18n::localized_bytes(self.language, bytes))
                        .unwrap_or_else(|| "—".to_owned()),
                )
                .size(T::SMALL_TEXT_SIZE)
                .color(T::TEXT),
            );
            T::bullet_separator(ui);
            ui.label(T::section_label(self.t(K::Result)));
            ui.label(
                RichText::new(
                    output_size
                        .map(|bytes| i18n::localized_bytes(self.language, bytes))
                        .unwrap_or_else(|| "—".to_owned()),
                )
                .size(T::SMALL_TEXT_SIZE)
                .color(T::ACCENT),
            );
            if let (Some(original), Some(output)) = (source_size, output_size)
                && original > 0
            {
                let change = (1.0 - output as f64 / original as f64) * 100.0;
                let (text, color) = if change > 0.5 {
                    (format!("−{change:.0} %"), T::ACCENT)
                } else if change < -0.5 {
                    (format!("+{:.0} %", change.abs()), T::TEXT_MUTED)
                } else {
                    (self.t(K::EqualWeight).to_owned(), T::TEXT_MUTED)
                };
                ui.add_space(2.0);
                ui.label(RichText::new(text).size(T::SMALL_TEXT_SIZE).color(color));
            }
        });
    }

    pub(crate) fn show_export(&mut self, ui: &mut egui::Ui, actions: &mut Vec<AppAction>) {
        let enabled = self.can_export();
        let label = format!(
            "{}   Ctrl+S",
            i18n::export_label(self.language, self.format.extension())
        );
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
        ui.add_space(6.0);
    }
}

fn section_heading(id: &str, title: &str) -> egui::CollapsingHeader {
    egui::CollapsingHeader::new(RichText::new(title).size(11.0).color(T::TEXT).strong()).id_salt(id)
}

fn fitted_filename(ui: &egui::Ui, name: &str, width: f32) -> String {
    let limit = (width - 6.0).max(20.0);
    let font = egui::FontId::proportional(T::SMALL_TEXT_SIZE);
    let measure = |text: &str| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font.clone(), T::TEXT)
            .size()
            .x
    };

    if measure(name) <= limit {
        return name.to_owned();
    }

    let mut split = None;
    for (index, _) in name.char_indices().skip(1) {
        if measure(&name[..index]) > limit {
            break;
        }
        if measure(&name[index..]) <= limit {
            split = Some(index);
        }
    }
    if let Some(index) = split {
        let mut chosen = index;
        for (part_index, character) in name[..index].char_indices().rev() {
            if matches!(character, ' ' | '_' | '-') && part_index > index / 3 {
                chosen = part_index + character.len_utf8();
                break;
            }
        }
        if chosen != index && measure(&name[..chosen]) <= limit && measure(&name[chosen..]) <= limit
        {
            return format!("{}\n{}", &name[..chosen], &name[chosen..]);
        }
        return format!("{}\n{}", &name[..index], &name[index..]);
    }

    let (stem, extension) = name
        .rfind('.')
        .filter(|index| *index > 0)
        .map_or((name, ""), |index| (&name[..index], &name[index..]));
    let stem_chars: Vec<char> = stem.chars().collect();
    let ellipsis = "…";

    let mut head_count = 0;
    for count in 1..=stem_chars.len() {
        let candidate: String = stem_chars[..count].iter().collect();
        if measure(&candidate) + measure(ellipsis) <= limit {
            head_count = count;
        } else {
            break;
        }
    }

    let mut tail_count = 0;
    for count in 1..=stem_chars.len() {
        let candidate: String = stem_chars[stem_chars.len() - count..].iter().collect();
        if measure(&candidate) + measure(ellipsis) + measure(extension) <= limit {
            tail_count = count;
        } else {
            break;
        }
    }

    if head_count + tail_count > stem_chars.len() {
        tail_count = stem_chars.len().saturating_sub(head_count);
    }
    let head: String = stem_chars[..head_count].iter().collect();
    let tail: String = stem_chars[stem_chars.len() - tail_count..].iter().collect();
    format!("{head}{ellipsis}\n{ellipsis}{tail}{extension}")
}

fn choice_text(label: &str, selected: bool) -> RichText {
    RichText::new(label).color(if selected { T::ACCENT_INK } else { T::TEXT })
}

const SECTION_SCROLL_FRAMES: u64 = 24;

fn section_scroll_id(section: &'static str) -> egui::Id {
    egui::Id::unique(("sidebar_section_scroll", section))
}

fn request_section_scroll(ui: &egui::Ui, section: &'static str, rect: egui::Rect) {
    let deadline = ui.ctx().cumulative_frame_nr() + SECTION_SCROLL_FRAMES;
    ui.data_mut(|data| data.insert_temp(section_scroll_id(section), (rect, deadline)));
}

fn apply_pending_section_scroll<T>(
    ui: &egui::Ui,
    section_name: &'static str,
    section: &egui::CollapsingResponse<T>,
) {
    let id = section_scroll_id(section_name);
    let Some((rect, deadline)) = ui.data_mut(|data| data.get_temp::<(egui::Rect, u64)>(id)) else {
        return;
    };
    let frame = ui.ctx().cumulative_frame_nr();
    if section.fully_closed() || frame > deadline {
        ui.data_mut(|data| data.remove::<(egui::Rect, u64)>(id));
        return;
    }
    ui.scroll_to_rect_animation(
        rect.translate(vec2(0.0, -6.0)),
        Some(Align::TOP),
        egui::style::ScrollAnimation::none(),
    );
    if frame == deadline {
        ui.data_mut(|data| data.remove::<(egui::Rect, u64)>(id));
    } else {
        ui.ctx().request_repaint();
    }
}
