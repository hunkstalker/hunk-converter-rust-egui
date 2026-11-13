use eframe::egui::{
    self, Align, Align2, CornerRadius, Frame, Layout, Margin, Order, Rect, RichText, Sense, Stroke,
    pos2, vec2,
};

use crate::codec::OutputFormat;
use crate::i18n::{self, K, Lang};
use crate::settings::PanelSide;
use crate::theme as T;

use super::super::HunkApp;
use super::super::actions::AppAction;
use super::super::state::CropPreset;
use super::canvas::CanvasContext;
use super::icons::{
    EyedropperButton, QUICK_TOOL_GAP, QUICK_TOOL_SIZE, QuickTool, eyedropper_button,
    panel_side_button, quick_tool_button, theme_toggle_button,
};

const QUICK_TOOLS_GAP: f32 = 8.0;
const QUICK_TOOL_COUNT: f32 = 5.0;
const QUICK_TOOLS_HEIGHT: f32 =
    QUICK_TOOL_COUNT * QUICK_TOOL_SIZE + (QUICK_TOOL_COUNT - 1.0) * QUICK_TOOL_GAP;
const CROP_PRESETS_GAP: f32 = 8.0;
const CROP_PRESETS_WIDTH: f32 = 250.0;
const CROP_PRESETS_HEIGHT: f32 = 140.0;
const COLOR_PANEL_WIDTH: f32 = 300.0;
const COLOR_PANEL_MARGIN: f32 = 56.0;

impl HunkApp {
    pub(crate) fn show_preview(&mut self, root: &mut egui::Ui) -> Vec<AppAction> {
        let mut actions = Vec::new();
        let ctx = root.ctx().clone();
        egui::CentralPanel::default()
            .frame(Frame::new().fill(T::bg()).inner_margin(Margin {
                left: 20,
                right: 16,
                top: 8,
                bottom: 12,
            }))
            .show(root, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(self.t(K::Preview))
                            .size(20.0)
                            .color(T::text()),
                    );
                    ui.add_space(4.0);
                    if self.source.is_some() && self.format != OutputFormat::Ico {
                        ui.add_space(6.0);
                        let actual_percent = if self.actual_size {
                            100.0
                        } else {
                            self.zoom * self.fit_scale * 100.0
                        };
                        let zoom_color = if (actual_percent - 100.0).abs() < 0.5 {
                            T::accent()
                        } else {
                            T::text_faint()
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

                let has_image = self.source.is_some();
                if has_image {
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
                } else {
                    self.paint_empty_canvas(ui, rect);
                }

                let content_rect = ctx.content_rect();
                let mut color_area = egui::Area::new(egui::Id::unique("color_panel_overlay"))
                    .order(Order::Foreground)
                    .constrain_to(content_rect);
                if let Some(position) = self.color_panel_pos {
                    color_area = color_area.fixed_pos(position);
                } else if self.panel_side == PanelSide::Right {
                    color_area =
                        color_area.anchor(Align2::LEFT_BOTTOM, vec2(COLOR_PANEL_MARGIN, -92.0));
                } else {
                    color_area =
                        color_area.anchor(Align2::RIGHT_BOTTOM, vec2(-COLOR_PANEL_MARGIN, -92.0));
                }
                let color_panel = color_area.show(&ctx, |ui| {
                    ui.set_width(COLOR_PANEL_WIDTH);
                    actions.extend(self.show_color_panel(ui));
                });
                let panel_rect = color_panel.response.rect;
                self.color_panel_size = panel_rect.size();

                if !has_image {
                    let empty_rect = empty_content_rect(rect, self.panel_side);
                    actions.extend(self.show_empty_canvas_content(ui, empty_rect));
                }

                if has_image {
                    let (tools_pos, tools_on_left) = quick_tools_position(panel_rect, content_rect);
                    let mut crop_button_rect = None;
                    egui::Area::new(egui::Id::unique("canvas_quick_tools"))
                        .fixed_pos(tools_pos)
                        .order(Order::Foreground)
                        .constrain_to(content_rect)
                        .show(&ctx, |ui| {
                            ui.set_width(QUICK_TOOL_SIZE);
                            ui.spacing_mut().item_spacing.y = QUICK_TOOL_GAP;
                            let icon_mode = self.format == OutputFormat::Ico;
                            let crop_editing = self.crop_edit.is_some();

                            let has_unlocked = self.history.iter().any(|entry| !entry.locked);
                            let clear_label = self.t(K::ClearHistory);
                            if quick_tool_button(
                                ui,
                                QuickTool::Trash,
                                false,
                                has_unlocked,
                                clear_label,
                            )
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
                                        memory.request_focus(egui::Id::unique(
                                            super::canvas::CANVAS_ID,
                                        ));
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
                        let presets_pos =
                            crop_presets_position(anchor, content_rect, tools_on_left);
                        egui::Area::new(egui::Id::unique("crop_presets"))
                            .fixed_pos(presets_pos)
                            .order(Order::Foreground)
                            .constrain_to(content_rect)
                            .show(&ctx, |ui| {
                                self.show_crop_presets(ui, &mut actions);
                            });
                    }
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
            .fill(T::surface())
            .stroke(Stroke::new(1.0, T::border()))
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
                                    T::accent_ink()
                                } else {
                                    T::text()
                                }),
                            )
                            .fill(if selected {
                                T::accent()
                            } else {
                                T::surface_2()
                            })
                            .stroke(Stroke::new(
                                1.0,
                                if selected { T::accent() } else { T::border() },
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
                            .color(T::accent_ink())
                            .strong(),
                    )
                    .fill(T::accent())
                    .stroke(Stroke::new(1.0, T::accent()))
                    .corner_radius(CornerRadius::same(8))
                    .min_size(vec2(100.0, 28.0));
                    if ui.add(apply).clicked() && self.apply_crop() {
                        self.result = None;
                        actions.push(AppAction::RequestEncode(None));
                    }
                    let cancel = egui::Button::new(
                        RichText::new(self.t(K::CropCancel))
                            .size(11.0)
                            .color(T::text_muted()),
                    )
                    .fill(T::surface_2())
                    .stroke(Stroke::new(1.0, T::border()))
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
                            .color(T::danger()),
                    )
                    .fill(T::surface_2())
                    .stroke(Stroke::new(1.0, T::border()))
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
        ui.spacing_mut().item_spacing.x = 8.0;
        if self.source.is_some() {
            let reset_width = Lang::ALL
                .iter()
                .map(|lang| {
                    ui.painter()
                        .layout_no_wrap(
                            i18n::t(*lang, K::Reset).to_owned(),
                            egui::FontId::proportional(11.5),
                            T::text_muted(),
                        )
                        .size()
                        .x
                })
                .fold(0.0_f32, f32::max);
            let reset = egui::Button::new(
                RichText::new(self.t(K::Reset))
                    .size(11.5)
                    .color(T::text_muted()),
            )
            .fill(egui::Color32::TRANSPARENT)
            .stroke(Stroke::new(1.0, T::border()))
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
                    .color(T::text_muted()),
            )
            .width(64.0)
            .show_ui(ui, |ui| {
                for option in Lang::ALL {
                    let selected = language == option;
                    let text = RichText::new(option.name()).color(if selected {
                        T::accent_ink()
                    } else {
                        T::text()
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

        let label = match self.panel_side {
            PanelSide::Right => self.t(K::PanelToLeft),
            PanelSide::Left => self.t(K::PanelToRight),
        };
        let response = panel_side_button(ui, self.panel_side == PanelSide::Right, label);
        if response.clicked() {
            self.panel_side = self.panel_side.toggled();
            actions.push(AppAction::PersistSettings);
            actions.push(AppAction::RequestRepaint);
        }

        let light = self.theme.eq_ignore_ascii_case("light");
        let theme_label = self.t(if light {
            K::ThemeToDark
        } else {
            K::ThemeToLight
        });
        if theme_toggle_button(ui, light, theme_label).clicked() {
            let next = T::toggled_profile_id(&self.theme);
            self.theme = next.to_owned();
            T::apply(ui.ctx(), next);
            actions.push(AppAction::PersistSettings);
            actions.push(AppAction::RequestRepaint);
        }
    }
}

fn empty_content_rect(canvas: Rect, panel_side: PanelSide) -> Rect {
    // Reserva el hueco del panel de color por defecto. No usa su posición real:
    // mover el panel no debe desplazar el mensaje del canvas.
    let margin = 24.0;
    let reserved = COLOR_PANEL_MARGIN + COLOR_PANEL_WIDTH + margin;
    match panel_side {
        PanelSide::Right => Rect::from_min_max(
            pos2(canvas.left() + reserved, canvas.top() + margin),
            canvas.max - vec2(margin, margin),
        ),
        PanelSide::Left => Rect::from_min_max(
            canvas.min + vec2(margin, margin),
            pos2(canvas.right() - reserved, canvas.bottom() - margin),
        ),
    }
}

fn quick_tools_position(panel: Rect, content: Rect) -> (egui::Pos2, bool) {
    // Las herramientas cambian al lado izquierdo en cuanto el panel cruza el
    // centro horizontal de la aplicación, sin esperar al borde derecho.
    let tools_on_left = panel.left() >= content.center().x;
    let x = if tools_on_left {
        panel.left() - QUICK_TOOLS_GAP - QUICK_TOOL_SIZE
    } else {
        panel.right() + QUICK_TOOLS_GAP
    };
    let x = x.clamp(
        content.left(),
        (content.right() - QUICK_TOOL_SIZE).max(content.left()),
    );
    let y = (panel.bottom() - QUICK_TOOLS_HEIGHT).clamp(
        content.top(),
        (content.bottom() - QUICK_TOOLS_HEIGHT).max(content.top()),
    );
    (pos2(x, y), tools_on_left)
}

fn crop_presets_position(anchor: Rect, content: Rect, tools_on_left: bool) -> egui::Pos2 {
    let right_room = content.right() - anchor.right() - CROP_PRESETS_GAP;
    let left_room = anchor.left() - content.left() - CROP_PRESETS_GAP;
    let open_left = if tools_on_left {
        left_room >= CROP_PRESETS_WIDTH || left_room > right_room
    } else {
        right_room < CROP_PRESETS_WIDTH && left_room > right_room
    };
    let x = if open_left {
        anchor.left() - CROP_PRESETS_GAP - CROP_PRESETS_WIDTH
    } else {
        anchor.right() + CROP_PRESETS_GAP
    };
    let x = x.clamp(
        content.left(),
        (content.right() - CROP_PRESETS_WIDTH).max(content.left()),
    );
    let y = anchor.top().clamp(
        content.top(),
        (content.bottom() - CROP_PRESETS_HEIGHT).max(content.top()),
    );
    pos2(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn content() -> Rect {
        Rect::from_min_size(pos2(0.0, 0.0), vec2(1280.0, 1000.0))
    }

    #[test]
    fn quick_tools_flip_when_the_panel_reaches_the_right_edge() {
        let default_panel = Rect::from_min_size(pos2(56.0, 456.0), vec2(300.0, 473.0));
        let (default_pos, on_left) = quick_tools_position(default_panel, content());
        assert!(!on_left);
        assert_eq!(default_pos, pos2(364.0, 715.0));

        let midpoint_panel = Rect::from_min_size(pos2(640.0, 356.0), vec2(300.0, 473.0));
        let (midpoint_pos, on_left) = quick_tools_position(midpoint_panel, content());
        assert!(on_left);
        assert_eq!(midpoint_pos, pos2(594.0, 615.0));

        let right_panel = Rect::from_min_size(pos2(958.0, 356.0), vec2(300.0, 473.0));
        let (right_pos, on_left) = quick_tools_position(right_panel, content());
        assert!(on_left);
        assert_eq!(right_pos, pos2(912.0, 615.0));
    }

    #[test]
    fn empty_content_reserves_the_default_color_panel_side() {
        let canvas = Rect::from_min_size(pos2(20.0, 57.0), vec2(920.0, 886.0));

        assert_eq!(
            empty_content_rect(canvas, PanelSide::Right),
            Rect::from_min_max(pos2(400.0, 81.0), pos2(916.0, 919.0))
        );
        assert_eq!(
            empty_content_rect(canvas, PanelSide::Left),
            Rect::from_min_max(pos2(44.0, 81.0), pos2(560.0, 919.0))
        );
    }

    #[test]
    fn crop_presets_open_away_from_the_tools() {
        let left_anchor = Rect::from_min_size(pos2(916.0, 660.0), vec2(38.0, 38.0));
        assert_eq!(
            crop_presets_position(left_anchor, content(), true),
            pos2(658.0, 660.0)
        );

        let right_anchor = Rect::from_min_size(pos2(362.0, 660.0), vec2(38.0, 38.0));
        assert_eq!(
            crop_presets_position(right_anchor, content(), false),
            pos2(408.0, 660.0)
        );
    }
}
