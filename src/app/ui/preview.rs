use eframe::egui::{
    self, Align, Align2, CornerRadius, Frame, Layout, Margin, Order, RichText, Sense, Stroke, vec2,
};

use crate::codec::OutputFormat;
use crate::i18n::{self, K, Lang};
use crate::theme as T;

use super::super::HunkApp;
use super::super::actions::AppAction;
use super::super::state::CropPreset;
use super::canvas::CanvasContext;
use super::icons::{EyedropperButton, QuickTool, eyedropper_button, quick_tool_button};

impl HunkApp {
    pub(crate) fn show_preview(&mut self, root: &mut egui::Ui) -> Vec<AppAction> {
        let mut actions = Vec::new();
        let ctx = root.ctx().clone();
        egui::CentralPanel::default()
            .frame(Frame::new().fill(T::BG).inner_margin(Margin {
                left: 20,
                right: 16,
                top: 8,
                bottom: 12,
            }))
            .show(root, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(self.t(K::Preview)).size(20.0).color(T::TEXT));
                    ui.add_space(4.0);
                    if self.format != OutputFormat::Ico {
                        ui.add_space(6.0);
                        let actual_percent = if self.actual_size {
                            100.0
                        } else {
                            self.zoom * self.fit_scale * 100.0
                        };
                        let zoom_color = if (actual_percent - 100.0).abs() < 0.5 {
                            T::ACCENT
                        } else {
                            T::TEXT_FAINT
                        };
                        ui.allocate_ui_with_layout(
                            vec2(88.0, 26.0),
                            Layout::top_down(Align::Min),
                            |ui| {
                                // Caja de altura fija y desplazamiento óptico para que el
                                // porcentaje quede centrado con el título de 20 px.
                                ui.add_space(8.0);
                                ui.label(
                                    RichText::new(format!("Zoom {actual_percent:.0} %"))
                                        .size(11.5)
                                        .color(zoom_color),
                                );
                            },
                        );
                    }
                });
                ui.add_space(7.0);

                let canvas_height = ui.available_height().max(240.0);
                let size = vec2(ui.available_width(), canvas_height);
                let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
                let response = ui.interact(
                    rect,
                    egui::Id::unique(super::canvas::CANVAS_ID),
                    Sense::click_and_drag(),
                );
                actions.extend(self.paint_canvas(CanvasContext {
                    ui,
                    rect,
                    response: &response,
                    ctx: &ctx,
                }));

                egui::Area::new(egui::Id::unique("color_panel_overlay"))
                    .anchor(Align2::LEFT_BOTTOM, vec2(56.0, -92.0))
                    .order(Order::Foreground)
                    .show(&ctx, |ui| {
                        ui.set_width(300.0);
                        actions.extend(self.show_color_panel(ui));
                    });

                let mut crop_button_rect = None;
                egui::Area::new(egui::Id::unique("canvas_quick_tools"))
                    .anchor(Align2::LEFT_BOTTOM, vec2(368.0, -92.0))
                    .order(Order::Foreground)
                    .show(&ctx, |ui| {
                        ui.set_width(38.0);
                        ui.spacing_mut().item_spacing.y = 6.0;
                        let icon_mode = self.format == OutputFormat::Ico;
                        let crop_editing = self.crop_edit.is_some();

                        let has_unlocked = self.history.iter().any(|entry| !entry.locked);
                        let clear_label = self.t(K::ClearHistory);
                        if quick_tool_button(ui, QuickTool::Trash, false, has_unlocked, clear_label)
                            .clicked()
                        {
                            actions.push(AppAction::ClearHistory);
                        }

                        let crop_label = self.t(K::Crop);
                        let crop_response = quick_tool_button(
                            ui,
                            QuickTool::Crop,
                            crop_editing,
                            !icon_mode,
                            crop_label,
                        );
                        let crop_response = if icon_mode {
                            crop_response.on_hover_text(self.t(K::CropUnavailableIco))
                        } else {
                            crop_response
                        };
                        if crop_response.clicked() {
                            self.toggle_crop();
                        }
                        crop_button_rect = Some(crop_response.rect);
                        let crop_editing = self.crop_edit.is_some();

                        let eyedropper = eyedropper_button(
                            ui,
                            EyedropperButton {
                                active: self.eyedropper_active,
                                enabled: !icon_mode && !crop_editing,
                                label: self.t(K::Eyedropper),
                            },
                        );
                        let eyedropper_hint = if icon_mode {
                            self.t(K::EyedropperUnavailable)
                        } else if self.eyedropper_active {
                            self.t(K::DisableEyedropper)
                        } else {
                            self.t(K::EnableEyedropper)
                        };
                        let eyedropper = eyedropper.on_hover_text(eyedropper_hint);
                        if !icon_mode && !crop_editing {
                            if eyedropper.drag_started() {
                                self.eyedropper_active = true;
                                actions.push(AppAction::RequestRepaint);
                            } else if eyedropper.clicked()
                                || (self.eyedropper_active
                                    && eyedropper.clicked_by(egui::PointerButton::Secondary))
                            {
                                self.eyedropper_active = !self.eyedropper_active;
                                actions.push(AppAction::RequestRepaint);
                            }
                            if self.eyedropper_active {
                                ui.ctx().memory_mut(|memory| {
                                    memory
                                        .request_focus(egui::Id::unique(super::canvas::CANVAS_ID));
                                });
                            }
                        }

                        let invert_label = self.t(K::InvertColors);
                        let invert_active = self.advanced.preprocess.invert;
                        if quick_tool_button(
                            ui,
                            QuickTool::Invert,
                            invert_active,
                            !crop_editing,
                            invert_label,
                        )
                        .clicked()
                        {
                            actions.push(AppAction::ToggleInvert);
                        }
                        let rotate_label = self.t(K::RotateQuick);
                        if quick_tool_button(
                            ui,
                            QuickTool::Rotate,
                            false,
                            !crop_editing,
                            rotate_label,
                        )
                        .clicked()
                        {
                            actions.push(AppAction::RotateQuick);
                        }
                    });

                if let Some(anchor) = crop_button_rect
                    && self.crop_edit.is_some()
                {
                    egui::Area::new(egui::Id::unique("crop_presets"))
                        .fixed_pos(anchor.right_top() + vec2(8.0, 0.0))
                        .order(Order::Foreground)
                        .show(&ctx, |ui| {
                            self.show_crop_presets(ui, &mut actions);
                        });
                }

                egui::Area::new(egui::Id::unique("top_controls_overlay"))
                    .anchor(Align2::RIGHT_TOP, vec2(-24.0, 12.0))
                    .order(Order::Foreground)
                    .show(&ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                self.show_top_controls(ui, &mut actions);
                            });
                        });
                    });
            });
        actions
    }

    fn show_crop_presets(&mut self, ui: &mut egui::Ui, actions: &mut Vec<AppAction>) {
        egui::Frame::new()
            .fill(T::SURFACE)
            .stroke(Stroke::new(1.0, T::BORDER))
            .corner_radius(CornerRadius::same(12))
            .inner_margin(Margin::same(10))
            .show(ui, |ui| {
                ui.set_width(230.0);
                ui.label(T::section_label(self.t(K::Crop)));
                ui.add_space(4.0);

                ui.scope(|ui| {
                    ui.spacing_mut().button_padding = vec2(5.0, 4.0);
                    ui.spacing_mut().item_spacing.x = 4.0;
                    ui.horizontal_wrapped(|ui| {
                        for (preset, label) in [
                            (CropPreset::Free, K::CropFree),
                            (CropPreset::Original, K::CropOriginal),
                            (CropPreset::Square, K::CropSquare),
                            (CropPreset::FourThree, K::CropFourThree),
                            (CropPreset::SixteenNine, K::CropSixteenNine),
                        ] {
                            let selected = self.crop_preset == preset;
                            let button = egui::Button::new(
                                RichText::new(self.t(label)).size(10.5).color(if selected {
                                    T::ACCENT_INK
                                } else {
                                    T::TEXT
                                }),
                            )
                            .fill(if selected { T::ACCENT } else { T::SURFACE_2 })
                            .stroke(Stroke::new(
                                1.0,
                                if selected { T::ACCENT } else { T::BORDER },
                            ))
                            .corner_radius(CornerRadius::same(8))
                            .min_size(vec2(0.0, 26.0));
                            if ui.add(button).clicked() {
                                self.set_crop_preset(preset);
                            }
                        }
                    });
                });

                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    let apply = egui::Button::new(
                        RichText::new(self.t(K::CropApply))
                            .size(11.0)
                            .color(T::ACCENT_INK)
                            .strong(),
                    )
                    .fill(T::ACCENT)
                    .stroke(Stroke::new(1.0, T::ACCENT))
                    .corner_radius(CornerRadius::same(8))
                    .min_size(vec2(100.0, 28.0));
                    if ui.add(apply).clicked() && self.apply_crop() {
                        self.result = None;
                        actions.push(AppAction::RequestEncode(None));
                    }
                    let cancel = egui::Button::new(
                        RichText::new(self.t(K::CropCancel))
                            .size(11.0)
                            .color(T::TEXT_MUTED),
                    )
                    .fill(T::SURFACE_2)
                    .stroke(Stroke::new(1.0, T::BORDER))
                    .corner_radius(CornerRadius::same(8))
                    .min_size(vec2(100.0, 28.0));
                    if ui.add(cancel).clicked() {
                        self.cancel_crop();
                    }
                });

                if self.crop.is_some() {
                    ui.add_space(4.0);
                    let remove = egui::Button::new(
                        RichText::new(self.t(K::RemoveCrop))
                            .size(10.5)
                            .color(T::DANGER),
                    )
                    .fill(T::SURFACE_2)
                    .stroke(Stroke::new(1.0, T::BORDER))
                    .corner_radius(CornerRadius::same(8))
                    .min_size(vec2(ui.available_width(), 26.0));
                    if ui.add(remove).clicked() && self.remove_crop() {
                        self.result = None;
                        actions.push(AppAction::RequestEncode(None));
                    }
                }
            });
    }

    pub(crate) fn show_top_controls(&mut self, ui: &mut egui::Ui, actions: &mut Vec<AppAction>) {
        if self.source.is_some() {
            let reset_width = Lang::ALL
                .iter()
                .map(|lang| {
                    ui.painter()
                        .layout_no_wrap(
                            i18n::t(*lang, K::Reset).to_owned(),
                            egui::FontId::proportional(11.5),
                            T::TEXT_MUTED,
                        )
                        .size()
                        .x
                })
                .fold(0.0_f32, f32::max);
            let reset = egui::Button::new(
                RichText::new(self.t(K::Reset))
                    .size(11.5)
                    .color(T::TEXT_MUTED),
            )
            .fill(egui::Color32::TRANSPARENT)
            .stroke(Stroke::new(1.0, T::BORDER))
            .corner_radius(CornerRadius::same(10))
            .min_size(vec2(reset_width + 28.0, 32.0));
            let response = ui.add(reset);
            if response.has_focus() {
                super::widgets::paint_focus_ring(ui.painter(), response.rect, 10);
            }
            if response.clicked() {
                actions.push(AppAction::Reset);
            }
        }

        let mut language = self.language;
        egui::ComboBox::from_id_salt("language_selector")
            .selected_text(
                RichText::new(language.short())
                    .size(11.0)
                    .color(T::TEXT_MUTED),
            )
            .width(64.0)
            .show_ui(ui, |ui| {
                for option in Lang::ALL {
                    let selected = language == option;
                    let text = RichText::new(option.name()).color(if selected {
                        T::ACCENT_INK
                    } else {
                        T::TEXT
                    });
                    ui.selectable_value(&mut language, option, text);
                }
            })
            .response
            .accessible_name(format!("{}: {}", self.t(K::Language), language.short()))
            .on_hover_text(self.t(K::Language));
        if language != self.language {
            self.language = language;
            actions.push(AppAction::PersistSettings);
            actions.push(AppAction::RequestRepaint);
        }
    }
}
