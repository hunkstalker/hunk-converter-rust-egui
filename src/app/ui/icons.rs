use std::f32::consts::PI;

use eframe::egui::{
    self, Color32, CornerRadius, CursorIcon, Rect, Sense, Stroke, StrokeKind, pos2, vec2,
};

use crate::theme as T;

pub(crate) struct EyedropperButton<'a> {
    pub active: bool,
    pub enabled: bool,
    pub label: &'a str,
}

pub(crate) fn eyedropper_button(ui: &mut egui::Ui, spec: EyedropperButton<'_>) -> egui::Response {
    let EyedropperButton {
        active,
        enabled,
        label,
    } = spec;
    let (rect, response) = ui.allocate_exact_size(vec2(32.0, 28.0), Sense::click_and_drag());
    let hovered = enabled && response.hovered();
    let fill = if active && enabled {
        T::ACCENT
    } else if hovered {
        T::SURFACE_3
    } else {
        T::SURFACE_2
    };
    let border = if active && enabled {
        T::ACCENT
    } else if hovered {
        T::BORDER_STRONG
    } else {
        T::BORDER
    };
    let icon = if !enabled {
        T::TEXT_DISABLED
    } else if active {
        T::ACCENT_INK
    } else {
        T::TEXT_MUTED
    };

    let painter = ui.painter();
    painter.rect(
        rect,
        CornerRadius::same(8),
        fill,
        Stroke::new(1.0, border),
        StrokeKind::Inside,
    );
    paint_eyedropper(painter, rect.shrink2(vec2(8.0, 6.0)), icon);
    response.widget_info(|| egui::WidgetInfo::selected(egui::Role::Switch, enabled, active, label));
    if enabled && response.has_focus() {
        super::widgets::paint_focus_ring(ui.painter(), rect, 8);
    }

    if enabled {
        response.on_hover_cursor(CursorIcon::PointingHand)
    } else {
        response
    }
}

pub(crate) fn paint_eyedropper(painter: &egui::Painter, rect: Rect, color: Color32) {
    let size = rect.width().min(rect.height());
    let center = rect.center();
    let tip = pos2(center.x - size * 0.40, center.y + size * 0.40);
    let shoulder = pos2(center.x - size * 0.08, center.y + size * 0.08);
    let bulb_center = pos2(center.x + size * 0.28, center.y - size * 0.28);
    let direction = (bulb_center - shoulder).normalized();
    let perpendicular = vec2(-direction.y, direction.x);

    painter.line_segment(
        [tip + direction * size * 0.05, bulb_center],
        Stroke::new(size * 0.11, color),
    );
    painter.circle_filled(bulb_center, size * 0.22, color);
    painter.line_segment(
        [
            shoulder - perpendicular * size * 0.18,
            shoulder + perpendicular * size * 0.18,
        ],
        Stroke::new(size * 0.08, color),
    );
    painter.line_segment(
        [
            bulb_center + direction * size * 0.14,
            bulb_center + direction * size * 0.26,
        ],
        Stroke::new(size * 0.08, color),
    );

    let tip_base = tip + direction * size * 0.18;
    painter.add(egui::Shape::convex_polygon(
        vec![
            tip,
            tip_base + perpendicular * size * 0.12,
            tip_base - perpendicular * size * 0.12,
        ],
        color,
        Stroke::NONE,
    ));
}

#[derive(Clone, Copy)]
pub(crate) enum QuickTool {
    Invert,
    Rotate,
}

pub(crate) fn quick_tool_button(
    ui: &mut egui::Ui,
    tool: QuickTool,
    active: bool,
    label: &str,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(vec2(38.0, 38.0), Sense::click());
    let hovered = response.hovered();
    let fill = if active {
        T::ACCENT
    } else if hovered {
        T::SURFACE_3
    } else {
        T::SURFACE_2
    };
    let border = if active {
        T::ACCENT
    } else if hovered {
        T::BORDER_STRONG
    } else {
        T::BORDER
    };
    let icon = if active { T::ACCENT_INK } else { T::TEXT_MUTED };
    ui.painter().rect(
        rect,
        CornerRadius::same(12),
        fill,
        Stroke::new(1.0, border),
        StrokeKind::Inside,
    );
    match tool {
        QuickTool::Invert => paint_invert_icon(ui.painter(), rect, icon),
        QuickTool::Rotate => paint_rotate_icon(ui.painter(), rect, icon),
    }
    response.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, label.to_owned()));
    if response.has_focus() {
        super::widgets::paint_focus_ring(ui.painter(), rect, 12);
    }
    response
        .on_hover_cursor(CursorIcon::PointingHand)
        .on_hover_text(label)
}

fn paint_invert_icon(painter: &egui::Painter, rect: Rect, color: Color32) {
    let center = rect.center();
    let radius = 8.0;
    painter.circle_stroke(center, radius, Stroke::new(1.4, color));
    let mut half = Vec::with_capacity(17);
    half.push(center);
    for step in 0..=16 {
        let angle = -PI / 2.0 + step as f32 / 16.0 * PI;
        half.push(center + vec2(angle.cos(), angle.sin()) * radius);
    }
    painter.add(egui::Shape::convex_polygon(half, color, Stroke::NONE));
}

fn paint_rotate_icon(painter: &egui::Painter, rect: Rect, color: Color32) {
    let center = rect.center();
    let radius = 8.0;
    let mut arc = Vec::with_capacity(24);
    for step in 0..=20 {
        let angle = -0.75 * PI + step as f32 / 20.0 * 1.65 * PI;
        arc.push(center + vec2(angle.cos(), angle.sin()) * radius);
    }
    painter.add(egui::Shape::line(arc.clone(), Stroke::new(1.4, color)));
    if let (Some(end), Some(previous)) = (arc.last(), arc.get(arc.len().saturating_sub(2))) {
        let direction = (*end - *previous).normalized();
        let normal = vec2(-direction.y, direction.x);
        painter.add(egui::Shape::convex_polygon(
            vec![
                *end + direction * 3.0,
                *end - normal * 3.0 - direction * 1.5,
                *end + normal * 3.0 - direction * 1.5,
            ],
            color,
            Stroke::NONE,
        ));
    }
}

#[derive(Clone, Copy)]
pub(crate) enum IconAction {
    Copy,
    Trash,
}

pub(crate) fn icon_action_button(
    ui: &mut egui::Ui,
    action: IconAction,
    enabled: bool,
    label: &str,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(vec2(22.0, 20.0), Sense::click());
    let hovered = enabled && response.hovered();
    if hovered {
        ui.painter()
            .rect_filled(rect, CornerRadius::same(5), T::SURFACE_3);
    }
    let color = if !enabled {
        T::TEXT_DISABLED
    } else if hovered {
        T::TEXT
    } else {
        T::TEXT_MUTED
    };
    match action {
        IconAction::Copy => paint_copy_icon(ui.painter(), rect, color),
        IconAction::Trash => paint_trash_icon(ui.painter(), rect, color),
    }
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, enabled, label.to_owned()));
    if enabled && response.has_focus() {
        super::widgets::paint_focus_ring(ui.painter(), rect, 5);
    }
    if enabled {
        response.on_hover_cursor(CursorIcon::PointingHand)
    } else {
        response
    }
}

pub(crate) fn paint_copy_icon(painter: &egui::Painter, rect: Rect, color: Color32) {
    let body = Rect::from_center_size(rect.center() + vec2(0.0, 1.0), vec2(10.0, 12.0));
    painter.rect_stroke(
        body,
        CornerRadius::same(2),
        Stroke::new(1.2, color),
        StrokeKind::Inside,
    );
    let clip = Rect::from_center_size(body.center_top() + vec2(0.0, 1.0), vec2(5.0, 3.0));
    painter.rect_filled(clip, CornerRadius::same(1), color);
    for offset in [-2.0_f32, 2.0] {
        painter.line_segment(
            [
                pos2(body.left() + 2.5, body.center().y + offset),
                pos2(body.right() - 2.5, body.center().y + offset),
            ],
            Stroke::new(1.0, color),
        );
    }
}

pub(crate) fn paint_trash_icon(painter: &egui::Painter, rect: Rect, color: Color32) {
    let center = rect.center();
    painter.line_segment(
        [
            pos2(center.x - 6.0, center.y - 5.0),
            pos2(center.x + 6.0, center.y - 5.0),
        ],
        Stroke::new(1.3, color),
    );
    painter.line_segment(
        [
            pos2(center.x - 2.5, center.y - 7.0),
            pos2(center.x + 2.5, center.y - 7.0),
        ],
        Stroke::new(1.3, color),
    );
    painter.line_segment(
        [
            pos2(center.x - 4.5, center.y - 3.0),
            pos2(center.x - 3.5, center.y + 7.0),
        ],
        Stroke::new(1.2, color),
    );
    painter.line_segment(
        [
            pos2(center.x + 4.5, center.y - 3.0),
            pos2(center.x + 3.5, center.y + 7.0),
        ],
        Stroke::new(1.2, color),
    );
    painter.line_segment(
        [
            pos2(center.x - 3.5, center.y + 7.0),
            pos2(center.x + 3.5, center.y + 7.0),
        ],
        Stroke::new(1.2, color),
    );
    painter.line_segment(
        [
            pos2(center.x - 1.0, center.y - 1.0),
            pos2(center.x - 0.5, center.y + 4.0),
        ],
        Stroke::new(1.0, color),
    );
    painter.line_segment(
        [
            pos2(center.x + 1.0, center.y - 1.0),
            pos2(center.x + 0.5, center.y + 4.0),
        ],
        Stroke::new(1.0, color),
    );
}
