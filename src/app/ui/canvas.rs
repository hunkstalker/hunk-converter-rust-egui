use std::sync::Arc;

use eframe::egui::{
    self, Align2, Color32, CornerRadius, CursorIcon, FontId, Rect, Stroke, StrokeKind, Vec2, pos2,
    vec2,
};

use crate::codec::OutputFormat;
use crate::domain::{CropRect, rotated_size};
use crate::i18n::{self, K};
use crate::theme as T;

use super::super::HunkApp;
use super::super::actions::AppAction;
use super::super::state::{CropDrag, CropHandle, CropPreset};
use super::super::{FIT_ZOOM, MAX_ZOOM, MIN_ZOOM};
use super::widgets::{Badge, fit_rect, paint_badge, paint_checker};

pub(crate) const CANVAS_ID: &str = "hunk_preview_canvas";
const MIN_SPLIT: f32 = 0.015;
const MAX_SPLIT: f32 = 0.985;

pub(super) struct CanvasContext<'a> {
    pub ui: &'a egui::Ui,
    pub rect: Rect,
    pub response: &'a egui::Response,
    pub ctx: &'a egui::Context,
}

struct ZoomInput {
    factor: f32,
    pointer: egui::Pos2,
    metrics: CanvasMetrics,
    area: Rect,
}

#[derive(Clone, Copy)]
struct CanvasMetrics {
    content: CropRect,
    content_size: Vec2,
    fit_size: Vec2,
    fit_scale: f32,
}

impl HunkApp {
    fn canvas_metrics(
        &self,
        source_size: Vec2,
        content: CropRect,
        reference_size: Option<Vec2>,
        area: Rect,
    ) -> CanvasMetrics {
        let content_size = vec2(
            source_size.x * content.width,
            source_size.y * content.height,
        );
        let fit_reference = reference_size.unwrap_or(content_size);
        let fit_rect = fit_rect(fit_reference, area);
        let fit_scale = (fit_rect.width() / fit_reference.x.max(f32::EPSILON)).max(f32::EPSILON);
        CanvasMetrics {
            content,
            content_size,
            fit_size: content_size * fit_scale,
            fit_scale,
        }
    }

    fn content_display_size(&self, metrics: CanvasMetrics) -> Vec2 {
        if self.actual_size {
            metrics.content_size
        } else {
            metrics.fit_size * self.zoom
        }
    }

    fn canvas_image_rect(&self, source_size: Vec2, metrics: CanvasMetrics, area: Rect) -> Rect {
        let content_rect =
            Rect::from_center_size(area.center() + self.pan, self.content_display_size(metrics));
        let scale = content_rect.width() / metrics.content_size.x.max(f32::EPSILON);
        let image_size = source_size * scale;
        let offset = vec2(
            metrics.content.x * source_size.x * scale,
            metrics.content.y * source_size.y * scale,
        );
        Rect::from_min_size(content_rect.min - offset, image_size)
    }

    fn clamp_pan(&mut self, content_size: Vec2, area: Rect) {
        if !self.actual_size && (self.zoom - FIT_ZOOM).abs() < f32::EPSILON {
            self.pan = Vec2::ZERO;
            return;
        }
        let max_x = ((content_size.x - area.width()) * 0.5).max(0.0);
        let max_y = ((content_size.y - area.height()) * 0.5).max(0.0);
        self.pan.x = self.pan.x.clamp(-max_x, max_x);
        self.pan.y = self.pan.y.clamp(-max_y, max_y);
    }

    pub(crate) fn fit_view(&mut self) {
        self.actual_size = false;
        self.zoom = FIT_ZOOM;
        self.pan = Vec2::ZERO;
        self.pending_zoom = None;
    }

    pub(crate) fn actual_size_view(&mut self) {
        self.actual_size = true;
        self.pan = Vec2::ZERO;
        self.pending_zoom = None;
    }

    fn zoom_by(&mut self, input: ZoomInput) {
        let ZoomInput {
            factor,
            pointer,
            metrics,
            area,
        } = input;
        if metrics.content_size.x <= 0.0
            || metrics.content_size.y <= 0.0
            || (factor - 1.0).abs() < f32::EPSILON
        {
            return;
        }

        let old_rect =
            Rect::from_center_size(area.center() + self.pan, self.content_display_size(metrics));
        if old_rect.width() <= 0.0 || old_rect.height() <= 0.0 {
            return;
        }

        let new_zoom = if self.actual_size {
            let relative = factor / metrics.fit_scale;
            if relative < MIN_ZOOM {
                self.fit_view();
                return;
            }
            self.actual_size = false;
            relative.clamp(MIN_ZOOM, MAX_ZOOM)
        } else {
            let zoom = (self.zoom * factor).clamp(MIN_ZOOM, MAX_ZOOM);
            if (zoom - self.zoom).abs() < f32::EPSILON {
                return;
            }
            zoom
        };

        let normalized = vec2(
            ((pointer.x - old_rect.left()) / old_rect.width()).clamp(0.0, 1.0),
            ((pointer.y - old_rect.top()) / old_rect.height()).clamp(0.0, 1.0),
        );
        let new_size = metrics.fit_size * new_zoom;
        let new_center = pointer - normalized * new_size + new_size * 0.5;

        self.zoom = new_zoom;
        self.pan = new_center - area.center();
        self.clamp_pan(new_size, area);
    }
    pub(super) fn paint_canvas(&mut self, context: CanvasContext<'_>) -> Vec<AppAction> {
        let CanvasContext {
            ui,
            rect,
            response,
            ctx,
        } = context;
        let mut actions = Vec::new();
        let painter = ui.painter();
        let area = rect.shrink(12.0);
        let Some(source) = self.source.as_ref() else {
            return actions;
        };
        let source_texture = source.texture.id();
        let source_width = source.width;
        let source_height = source.height;
        let source_pixels = Arc::clone(&source.pixels);
        let original_size = source.original.dimensions();
        let (original_width, original_height) = rotated_size(
            original_size.0,
            original_size.1,
            self.advanced.preprocess.rotation,
        );
        let crop_applied = self.crop;
        let crop_edit = self.crop_edit;
        let crop_active = crop_edit.or(crop_applied);
        let reference_frame = crop_active.is_none()
            && self.advanced.preprocess.resize_enabled
            && (original_width, original_height) != (source_width, source_height);
        let view_content = if crop_edit.is_none() {
            crop_applied.unwrap_or(CropRect::FULL)
        } else {
            CropRect::FULL
        };
        let source_size = vec2(source_width as f32, source_height as f32);
        let icon_mode = self.format == OutputFormat::Ico;

        if icon_mode {
            response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::Role::Image,
                    true,
                    format!(
                        "{} — {} × {} px · {}",
                        self.t(K::Preview),
                        source_width,
                        source_height,
                        self.format.label()
                    ),
                )
            });
            if let Some(result) = self
                .result
                .as_ref()
                .filter(|result| result.format == self.format)
            {
                let icon_size = vec2(result.width as f32, result.height as f32);
                let icon_rect = fit_rect(icon_size, area);
                paint_checker(painter, icon_rect);
                painter.image(
                    result.texture.id(),
                    icon_rect,
                    Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                    Color32::WHITE,
                );
                paint_badge(
                    painter,
                    Badge {
                        anchor: pos2(icon_rect.right() - 10.0, icon_rect.top() + 10.0),
                        align: Align2::RIGHT_TOP,
                        text: i18n::ico_badge(self.language, result.frame_count).as_str(),
                        background: T::accent(),
                        foreground: T::accent_ink(),
                    },
                );
            } else {
                painter.text(
                    area.center(),
                    Align2::CENTER_CENTER,
                    self.t(K::GeneratingIcon),
                    FontId::proportional(13.0),
                    T::text_faint(),
                );
            }
            return actions;
        }

        let reference_size =
            reference_frame.then(|| vec2(original_width as f32, original_height as f32));
        let metrics = self.canvas_metrics(source_size, view_content, reference_size, area);
        if let Some(previous) = self.view_content
            && previous != view_content
        {
            let was_fit = !self.actual_size && (self.zoom - FIT_ZOOM).abs() < f32::EPSILON;
            if was_fit {
                self.zoom = FIT_ZOOM;
                self.pan = Vec2::ZERO;
            } else {
                let old_scale = if self.actual_size {
                    1.0
                } else {
                    self.fit_scale * self.zoom
                };
                if !self.actual_size {
                    self.zoom = (old_scale / metrics.fit_scale).clamp(MIN_ZOOM, MAX_ZOOM);
                }
                let new_scale = if self.actual_size {
                    1.0
                } else {
                    metrics.fit_scale * self.zoom
                };
                let old_pan = self.pan;
                if view_content != CropRect::FULL {
                    let center = crop_center(view_content);
                    self.pan = old_pan
                        + vec2(
                            (center.x - 0.5) * source_size.x * old_scale,
                            (center.y - 0.5) * source_size.y * old_scale,
                        );
                } else if previous != CropRect::FULL {
                    let center = crop_center(previous);
                    self.pan = old_pan
                        + vec2(
                            (0.5 - center.x) * source_size.x * new_scale,
                            (0.5 - center.y) * source_size.y * new_scale,
                        );
                }
            }
        }
        self.view_content = Some(view_content);
        self.fit_scale = metrics.fit_scale;
        self.clamp_pan(self.content_display_size(metrics), area);
        let mut image_rect = self.canvas_image_rect(source_size, metrics, area);

        if let Some(factor) = self.pending_zoom.take() {
            self.zoom_by(ZoomInput {
                factor,
                pointer: area.center(),
                metrics,
                area,
            });
            image_rect = self.canvas_image_rect(source_size, metrics, area);
        }

        let actual_percent = if self.actual_size {
            100.0
        } else {
            self.zoom * self.fit_scale * 100.0
        };
        response.widget_info(|| {
            egui::WidgetInfo::slider(
                true,
                f64::from(self.split * 100.0),
                format!(
                    "{} — {} × {} px · {} · {:.0} %",
                    self.t(K::Preview),
                    source_width,
                    source_height,
                    self.format.label(),
                    actual_percent
                ),
            )
        });
        ctx.accesskit_node_builder(response.id, |builder| {
            builder.add_action(egui::accesskit::Action::Increment);
            builder.add_action(egui::accesskit::Action::Decrement);
            builder.add_action(egui::accesskit::Action::SetValue);
        });
        let (increase, decrease, set_value) = ctx.input(|input| {
            let increase = input
                .num_accesskit_action_requests(response.id, egui::accesskit::Action::Increment);
            let decrease = input
                .num_accesskit_action_requests(response.id, egui::accesskit::Action::Decrement);
            let set_value = input
                .accesskit_action_requests(response.id, egui::accesskit::Action::SetValue)
                .last()
                .and_then(|request| match &request.data {
                    Some(egui::accesskit::ActionData::NumericValue(value)) => Some(*value),
                    _ => None,
                });
            (increase, decrease, set_value)
        });
        if increase > 0 {
            self.split = nudge_split(self.split, 0.01 * increase as f32);
        }
        if decrease > 0 {
            self.split = nudge_split(self.split, -0.01 * decrease as f32);
        }
        if let Some(value) = set_value {
            self.split = (value as f32 / 100.0).clamp(MIN_SPLIT, MAX_SPLIT);
        }

        if response.has_focus() && !response.gained_focus() {
            ui.memory_mut(|memory| {
                memory.set_focus_lock_filter(
                    response.id,
                    egui::EventFilter {
                        horizontal_arrows: true,
                        vertical_arrows: true,
                        escape: true,
                        ..Default::default()
                    },
                );
            });
            if self.eyedropper_active {
                let seed = self
                    .sampled_position
                    .unwrap_or((source_width / 2, source_height / 2));
                let (left, right, up, down, shift, commit) = ctx.input(|input| {
                    (
                        input.key_pressed(egui::Key::ArrowLeft),
                        input.key_pressed(egui::Key::ArrowRight),
                        input.key_pressed(egui::Key::ArrowUp),
                        input.key_pressed(egui::Key::ArrowDown),
                        input.modifiers.shift,
                        input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Space),
                    )
                });
                let step = if shift { 10 } else { 1 };
                let next = move_sample(
                    seed,
                    if left {
                        -step
                    } else if right {
                        step
                    } else {
                        0
                    },
                    if up {
                        -step
                    } else if down {
                        step
                    } else {
                        0
                    },
                    source_width,
                    source_height,
                );
                if next != seed || self.sampled_position.is_none() {
                    self.sampled_position = Some(next);
                    let pixel = source_pixels.get_pixel(next.0, next.1);
                    self.sampled_color = Some(pixel.0);
                    self.sample_pending_history = false;
                }
                if commit && let Some(rgba) = self.sampled_color {
                    actions.push(AppAction::RememberColor(rgba));
                }
            } else {
                let (left, right, up, down, home, end, shift) = ctx.input(|input| {
                    (
                        input.key_pressed(egui::Key::ArrowLeft),
                        input.key_pressed(egui::Key::ArrowRight),
                        input.key_pressed(egui::Key::ArrowUp),
                        input.key_pressed(egui::Key::ArrowDown),
                        input.key_pressed(egui::Key::Home),
                        input.key_pressed(egui::Key::End),
                        input.modifiers.shift,
                    )
                });
                if !shift && (left || right || home || end) {
                    if home {
                        self.split = MIN_SPLIT;
                    } else if end {
                        self.split = MAX_SPLIT;
                    } else if left {
                        self.split = nudge_split(self.split, -0.01);
                    } else if right {
                        self.split = nudge_split(self.split, 0.01);
                    }
                } else if self.actual_size || self.zoom > FIT_ZOOM {
                    let mut delta = Vec2::ZERO;
                    if up {
                        delta.y -= 24.0;
                    }
                    if down {
                        delta.y += 24.0;
                    }
                    if shift && left {
                        delta.x -= 24.0;
                    }
                    if shift && right {
                        delta.x += 24.0;
                    }
                    if delta != Vec2::ZERO {
                        self.pan += delta;
                        self.clamp_pan(self.content_display_size(metrics), area);
                        image_rect = self.canvas_image_rect(source_size, metrics, area);
                    }
                }
            }
        }

        let pointer = ctx.input(|input| input.pointer.hover_pos());
        let right_clicked =
            ctx.input(|input| input.pointer.button_pressed(egui::PointerButton::Secondary));
        let deactivated_eyedropper = self.eyedropper_active && right_clicked;
        if deactivated_eyedropper {
            self.eyedropper_active = false;
            self.sample_pending_history = false;
        }
        if response.hovered()
            && let Some(position) = pointer
        {
            let (scroll, pinch) =
                ctx.input(|input| (input.smooth_scroll_delta.y, input.zoom_delta()));
            let factor = if (pinch - 1.0).abs() > f32::EPSILON {
                pinch
            } else {
                (scroll * 0.0025).exp()
            };
            if (factor - 1.0).abs() > f32::EPSILON {
                self.zoom_by(ZoomInput {
                    factor,
                    pointer: position,
                    metrics,
                    area,
                });
                image_rect = self.canvas_image_rect(source_size, metrics, area);
            }
        }
        if !deactivated_eyedropper
            && response.dragged_by(egui::PointerButton::Secondary)
            && (self.actual_size || self.zoom > FIT_ZOOM)
        {
            let delta = ctx.input(|input| input.pointer.delta());
            self.pan += delta;
            self.clamp_pan(self.content_display_size(metrics), area);
            image_rect = self.canvas_image_rect(source_size, metrics, area);
        }

        if crop_edit.is_some() {
            self.update_crop_drag(ctx, response, pointer, image_rect);
        } else {
            self.crop_drag = None;
        }

        let crop_edit = self.crop_edit;
        let crop_applied = self.crop;
        let crop_rect = crop_edit
            .or(crop_applied)
            .map(|crop| crop_screen_rect(crop, image_rect))
            .filter(|rect| rect.intersects(area));
        let comparison_rect = if crop_edit.is_some() {
            None
        } else {
            crop_rect.or(Some(image_rect))
        };

        let view_rect = image_rect.intersect(area);
        let mut canvas = painter.clone();
        canvas.set_clip_rect(area);

        paint_checker(&canvas, image_rect);
        canvas.image(
            source_texture,
            image_rect,
            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
            Color32::WHITE,
        );

        let result_texture = self
            .result
            .as_ref()
            .filter(|result| result.format == self.format)
            .map(|result| result.texture.id());
        let split_rect = comparison_rect.unwrap_or(image_rect);
        let split_x = split_rect.left() + split_rect.width() * self.split;
        let over_handle = crop_edit.is_none()
            && result_texture.is_some()
            && pointer.is_some_and(|position| pointer_over_split(split_rect, split_x, position));

        if crop_edit.is_none() {
            if self.eyedropper_active {
                if response.hovered() {
                    ctx.set_cursor_icon(CursorIcon::Crosshair);
                }
                if (ctx.input(|input| input.pointer.primary_down())
                    || response.clicked_by(egui::PointerButton::Primary))
                    && let Some(position) = pointer
                    && split_rect.contains(position)
                    && area.contains(position)
                {
                    let nx =
                        ((position.x - image_rect.left()) / image_rect.width()).clamp(0.0, 1.0);
                    let ny =
                        ((position.y - image_rect.top()) / image_rect.height()).clamp(0.0, 1.0);
                    let x = (nx * source_width.saturating_sub(1) as f32).round() as u32;
                    let y = (ny * source_height.saturating_sub(1) as f32).round() as u32;
                    let pixel = source_pixels.get_pixel(x, y);
                    self.sampled_color = Some(pixel.0);
                    self.sampled_position = Some((x, y));
                    self.sample_pending_history = true;
                }
            } else {
                if result_texture.is_some()
                    && (response.dragged_by(egui::PointerButton::Primary)
                        || response.clicked_by(egui::PointerButton::Primary))
                    && let Some(position) = ctx.input(|input| input.pointer.interact_pos())
                    && split_rect.contains(position)
                {
                    self.split = ((position.x - split_rect.left()) / split_rect.width())
                        .clamp(MIN_SPLIT, MAX_SPLIT);
                }
                if response.hovered() && over_handle {
                    ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
                } else if (self.actual_size || self.zoom > FIT_ZOOM)
                    && (response.hovered() || response.dragged_by(egui::PointerButton::Secondary))
                {
                    ctx.set_cursor_icon(if response.dragged_by(egui::PointerButton::Secondary) {
                        CursorIcon::Grabbing
                    } else {
                        CursorIcon::Grab
                    });
                }
            }
        }

        if self.sample_pending_history && ctx.input(|input| input.pointer.primary_released()) {
            if let Some(rgba) = self.sampled_color {
                actions.push(AppAction::RememberColor(rgba));
            }
            self.sample_pending_history = false;
        }

        if let Some(texture_id) = result_texture
            && let Some(rect) = comparison_rect
            && rect.intersects(area)
        {
            let visible = rect.intersect(area);
            let split_x = rect.left() + rect.width() * self.split;
            let split_clip_x = split_x.clamp(visible.left(), visible.right());
            let mut clipped = canvas.clone();
            clipped.set_clip_rect(Rect::from_min_max(
                visible.min,
                pos2(split_clip_x, visible.max.y),
            ));
            clipped.image(
                texture_id,
                rect,
                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                Color32::WHITE,
            );

            if split_x >= visible.left() && split_x <= visible.right() && visible.height() > 24.0 {
                let line_color = T::split_line();
                canvas.line_segment(
                    [
                        pos2(split_x, visible.top()),
                        pos2(split_x, visible.bottom()),
                    ],
                    Stroke::new(3.0, T::split_shadow()),
                );
                canvas.line_segment(
                    [
                        pos2(split_x, visible.top()),
                        pos2(split_x, visible.bottom()),
                    ],
                    Stroke::new(1.2, line_color),
                );
                let center = pos2(split_x, visible.center().y);
                canvas.circle_filled(center, 18.0, T::split_shadow());
                canvas.circle_filled(center, 16.0, T::split_handle_bg());
                canvas.circle_stroke(center, 16.0, Stroke::new(1.2, T::split_handle_ring()));
                for direction in [-1.0_f32, 1.0] {
                    let tip = center + vec2(direction * 9.5, 0.0);
                    canvas.line_segment(
                        [center + vec2(direction * 4.0, -4.5), tip],
                        Stroke::new(1.4, T::text()),
                    );
                    canvas.line_segment(
                        [tip, center + vec2(direction * 4.0, 4.5)],
                        Stroke::new(1.4, T::text()),
                    );
                }
            }
        } else if self.busy() && crop_edit.is_none() {
            let center = comparison_rect.unwrap_or(image_rect).center();
            canvas.text(
                center,
                Align2::CENTER_CENTER,
                self.t(K::PreparingResult),
                FontId::proportional(13.0),
                T::text_faint(),
            );
        }

        if let Some(crop_rect) = crop_rect {
            paint_crop_shade(&canvas, image_rect, crop_rect, area);
            let editing = crop_edit.is_some();
            paint_crop_border(&canvas, crop_rect, editing);
            if editing {
                paint_crop_handles(&canvas, crop_rect);
                paint_crop_thirds(&canvas, crop_rect);
            }
        }

        if reference_frame && source_width > 0 {
            let scale = image_rect.width() / source_width as f32;
            let original_rect = Rect::from_center_size(
                image_rect.center(),
                vec2(
                    original_width as f32 * scale,
                    original_height as f32 * scale,
                ),
            );
            if original_rect.intersects(area) {
                canvas.rect_stroke(
                    original_rect,
                    CornerRadius::same(2),
                    Stroke::new(1.0, T::reference_frame()),
                    StrokeKind::Inside,
                );
            }
        }

        if self.eyedropper_active
            && let (Some(rgba), Some((sample_x, sample_y))) =
                (self.sampled_color, self.sampled_position)
        {
            let ratio_x = sample_x as f32 / source_width.saturating_sub(1).max(1) as f32;
            let ratio_y = sample_y as f32 / source_height.saturating_sub(1).max(1) as f32;
            let marker =
                image_rect.min + vec2(ratio_x * image_rect.width(), ratio_y * image_rect.height());
            canvas.circle_stroke(marker, 8.0, Stroke::new(3.0, T::sample_shadow()));
            canvas.circle_stroke(marker, 8.0, Stroke::new(1.2, T::sample_ring()));
            canvas.circle_filled(
                marker,
                3.0,
                Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3]),
            );
        }

        let badge_rect = if crop_edit.is_none() {
            crop_rect
                .map(|rect| rect.intersect(area))
                .filter(|rect| rect.width() >= 150.0 && rect.height() >= 60.0)
                .unwrap_or(view_rect)
        } else {
            view_rect
        };
        paint_badge(
            &canvas,
            Badge {
                anchor: badge_rect.left_top() + vec2(10.0, 10.0),
                align: Align2::LEFT_TOP,
                text: self.format.badge(),
                background: T::accent(),
                foreground: T::accent_ink(),
            },
        );
        paint_badge(
            &canvas,
            Badge {
                anchor: badge_rect.right_top() + vec2(-10.0, 10.0),
                align: Align2::RIGHT_TOP,
                text: self.t(K::Original),
                background: T::original_badge_bg(),
                foreground: T::original_badge_fg(),
            },
        );
        actions
    }

    fn update_crop_drag(
        &mut self,
        ctx: &egui::Context,
        response: &egui::Response,
        pointer: Option<egui::Pos2>,
        image_rect: Rect,
    ) {
        let Some(draft) = self.crop_edit else {
            self.crop_drag = None;
            return;
        };
        let crop_rect = crop_screen_rect(draft, image_rect);

        if response.drag_started_by(egui::PointerButton::Primary)
            && let Some(position) = ctx.input(|input| input.pointer.press_origin()).or(pointer)
        {
            self.crop_drag = crop_handle_at(crop_rect, position).map(|handle| CropDrag {
                handle,
                start: draft,
                start_pointer: position,
                start_image_rect: image_rect,
            });
        }

        if response.dragged_by(egui::PointerButton::Primary)
            && let (Some(drag), Some(position)) = (
                self.crop_drag,
                ctx.input(|input| input.pointer.interact_pos()),
            )
        {
            let dx = (position.x - drag.start_pointer.x) / drag.start_image_rect.width();
            let dy = (position.y - drag.start_pointer.y) / drag.start_image_rect.height();
            let min_width = (8.0 / drag.start_image_rect.width()).max(0.005);
            let min_height = (8.0 / drag.start_image_rect.height()).max(0.005);
            self.crop_edit = Some(adjust_crop(
                drag.start,
                drag.handle,
                dx,
                dy,
                min_width,
                min_height,
            ));
            if drag.handle != CropHandle::Move {
                self.crop_preset = CropPreset::Free;
            }
        }

        if response.drag_stopped_by(egui::PointerButton::Primary)
            || !ctx.input(|input| input.pointer.primary_down())
        {
            self.crop_drag = None;
        }

        if response.hovered()
            && let Some(position) = pointer
        {
            let handle = self
                .crop_drag
                .map(|drag| drag.handle)
                .or_else(|| crop_handle_at(crop_rect, position));
            if let Some(handle) = handle {
                ctx.set_cursor_icon(crop_cursor(handle));
            }
        }
    }
}

fn crop_center(crop: CropRect) -> Vec2 {
    vec2(crop.x + crop.width * 0.5, crop.y + crop.height * 0.5)
}

fn crop_screen_rect(crop: CropRect, image_rect: Rect) -> Rect {
    let min = image_rect.min + vec2(crop.x * image_rect.width(), crop.y * image_rect.height());
    let max = image_rect.min
        + vec2(
            (crop.x + crop.width) * image_rect.width(),
            (crop.y + crop.height) * image_rect.height(),
        );
    Rect::from_min_max(min, max).intersect(image_rect)
}

fn crop_handle_at(rect: Rect, pointer: egui::Pos2) -> Option<CropHandle> {
    let distance = 10.0;
    let near_left = (pointer.x - rect.left()).abs() <= distance;
    let near_right = (pointer.x - rect.right()).abs() <= distance;
    let near_top = (pointer.y - rect.top()).abs() <= distance;
    let near_bottom = (pointer.y - rect.bottom()).abs() <= distance;
    let within_x = pointer.x >= rect.left() - distance && pointer.x <= rect.right() + distance;
    let within_y = pointer.y >= rect.top() - distance && pointer.y <= rect.bottom() + distance;

    if near_left && near_top {
        Some(CropHandle::NorthWest)
    } else if near_right && near_top {
        Some(CropHandle::NorthEast)
    } else if near_left && near_bottom {
        Some(CropHandle::SouthWest)
    } else if near_right && near_bottom {
        Some(CropHandle::SouthEast)
    } else if near_top && within_x {
        Some(CropHandle::North)
    } else if near_bottom && within_x {
        Some(CropHandle::South)
    } else if near_left && within_y {
        Some(CropHandle::West)
    } else if near_right && within_y {
        Some(CropHandle::East)
    } else if rect.contains(pointer) {
        Some(CropHandle::Move)
    } else {
        None
    }
}

fn crop_cursor(handle: CropHandle) -> CursorIcon {
    match handle {
        CropHandle::Move => CursorIcon::Move,
        CropHandle::North | CropHandle::South => CursorIcon::ResizeVertical,
        CropHandle::East | CropHandle::West => CursorIcon::ResizeHorizontal,
        CropHandle::NorthWest | CropHandle::SouthEast => CursorIcon::ResizeNwSe,
        CropHandle::NorthEast | CropHandle::SouthWest => CursorIcon::ResizeNeSw,
    }
}

fn adjust_crop(
    start: CropRect,
    handle: CropHandle,
    dx: f32,
    dy: f32,
    min_width: f32,
    min_height: f32,
) -> CropRect {
    let mut left = start.x;
    let mut right = start.x + start.width;
    let mut top = start.y;
    let mut bottom = start.y + start.height;

    if handle == CropHandle::Move {
        let x = (start.x + dx).clamp(0.0, 1.0 - start.width);
        let y = (start.y + dy).clamp(0.0, 1.0 - start.height);
        return CropRect {
            x,
            y,
            width: start.width,
            height: start.height,
        };
    }
    if matches!(
        handle,
        CropHandle::West | CropHandle::NorthWest | CropHandle::SouthWest
    ) {
        left = (start.x + dx).clamp(0.0, right - min_width);
    }
    if matches!(
        handle,
        CropHandle::East | CropHandle::NorthEast | CropHandle::SouthEast
    ) {
        right = (right + dx).clamp(left + min_width, 1.0);
    }
    if matches!(
        handle,
        CropHandle::North | CropHandle::NorthWest | CropHandle::NorthEast
    ) {
        top = (start.y + dy).clamp(0.0, bottom - min_height);
    }
    if matches!(
        handle,
        CropHandle::South | CropHandle::SouthWest | CropHandle::SouthEast
    ) {
        bottom = (bottom + dy).clamp(top + min_height, 1.0);
    }

    CropRect {
        x: left,
        y: top,
        width: right - left,
        height: bottom - top,
    }
    .clamped()
}

fn paint_crop_shade(painter: &egui::Painter, image_rect: Rect, crop_rect: Rect, area: Rect) {
    let image = image_rect.intersect(area);
    let crop = crop_rect.intersect(image);
    if crop.width() <= 0.0 || crop.height() <= 0.0 {
        return;
    }
    let color = T::crop_shade();
    let pieces = [
        Rect::from_min_max(image.min, pos2(image.max.x, crop.top())),
        Rect::from_min_max(pos2(image.left(), crop.bottom()), image.max),
        Rect::from_min_max(
            pos2(image.left(), crop.top()),
            pos2(crop.left(), crop.bottom()),
        ),
        Rect::from_min_max(
            pos2(crop.right(), crop.top()),
            pos2(image.max.x, crop.bottom()),
        ),
    ];
    for piece in pieces {
        if piece.width() > 0.0 && piece.height() > 0.0 {
            painter.rect_filled(piece, 0.0, color);
        }
    }
}

fn paint_crop_border(painter: &egui::Painter, crop_rect: Rect, editing: bool) {
    let (width, color) = if editing {
        (1.6, T::accent())
    } else {
        (1.0, T::crop_border())
    };
    painter.rect_stroke(
        crop_rect,
        CornerRadius::same(2),
        Stroke::new(width, color),
        StrokeKind::Inside,
    );
}

fn paint_crop_thirds(painter: &egui::Painter, crop_rect: Rect) {
    let color = T::crop_thirds();
    for factor in [1.0 / 3.0, 2.0 / 3.0] {
        let x = crop_rect.left() + crop_rect.width() * factor;
        let y = crop_rect.top() + crop_rect.height() * factor;
        painter.line_segment(
            [pos2(x, crop_rect.top()), pos2(x, crop_rect.bottom())],
            Stroke::new(1.0, color),
        );
        painter.line_segment(
            [pos2(crop_rect.left(), y), pos2(crop_rect.right(), y)],
            Stroke::new(1.0, color),
        );
    }
}

fn paint_crop_handles(painter: &egui::Painter, crop_rect: Rect) {
    let positions = [
        crop_rect.left_top(),
        crop_rect.center_top(),
        crop_rect.right_top(),
        crop_rect.left_center(),
        crop_rect.right_center(),
        crop_rect.left_bottom(),
        crop_rect.center_bottom(),
        crop_rect.right_bottom(),
    ];
    for position in positions {
        let handle = Rect::from_center_size(position, vec2(9.0, 9.0));
        painter.rect_filled(handle, CornerRadius::same(2), T::accent());
        painter.rect_stroke(
            handle,
            CornerRadius::same(2),
            Stroke::new(1.0, T::accent_ink()),
            StrokeKind::Inside,
        );
    }
}

fn nudge_split(split: f32, delta: f32) -> f32 {
    (split + delta).clamp(MIN_SPLIT, MAX_SPLIT)
}

fn move_sample(position: (u32, u32), dx: i32, dy: i32, width: u32, height: u32) -> (u32, u32) {
    let x =
        (i64::from(position.0) + i64::from(dx)).clamp(0, i64::from(width.saturating_sub(1))) as u32;
    let y = (i64::from(position.1) + i64::from(dy)).clamp(0, i64::from(height.saturating_sub(1)))
        as u32;
    (x, y)
}

pub(super) fn pointer_over_split(image_rect: Rect, split_x: f32, pointer: egui::Pos2) -> bool {
    let over_line = (pointer.x - split_x).abs() <= 10.0
        && pointer.y >= image_rect.top()
        && pointer.y <= image_rect.bottom();
    let over_circle = pointer.distance(pos2(split_x, image_rect.center().y)) <= 20.0;
    over_line || over_circle
}

#[cfg(test)]
mod keyboard_tests {
    use super::*;

    #[test]
    fn split_stays_inside_the_image() {
        assert!((nudge_split(0.5, -0.1) - 0.4).abs() < f32::EPSILON);
        assert_eq!(nudge_split(0.01, -0.1), MIN_SPLIT);
        assert_eq!(nudge_split(0.99, 0.1), MAX_SPLIT);
    }

    #[test]
    fn keyboard_sample_stays_inside_the_source() {
        assert_eq!(move_sample((0, 0), -1, -1, 10, 10), (0, 0));
        assert_eq!(move_sample((9, 9), 1, 1, 10, 10), (9, 9));
        assert_eq!(move_sample((5, 5), 10, -10, 10, 10), (9, 0));
    }
}
