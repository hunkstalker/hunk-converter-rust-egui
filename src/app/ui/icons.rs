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
    let (rect, response) = ui.allocate_exact_size(vec2(38.0, 38.0), Sense::click_and_drag());
    let hovered = enabled && response.hovered();
    let fill = if active && enabled {
        T::accent()
    } else if hovered {
        T::surface_3()
    } else {
        T::surface_2()
    };
    let border = if active && enabled {
        T::accent()
    } else if hovered {
        T::border_strong()
    } else {
        T::border()
    };
    let icon = if !enabled {
        T::text_disabled()
    } else if active {
        T::accent_ink()
    } else {
        T::text_muted()
    };

    let painter = ui.painter();
    painter.rect(
        rect,
        CornerRadius::same(12),
        fill,
        Stroke::new(1.0, border),
        StrokeKind::Inside,
    );
    paint_eyedropper(painter, rect.shrink2(vec2(10.0, 9.0)), icon);
    response.widget_info(|| egui::WidgetInfo::selected(egui::Role::Switch, enabled, active, label));
    if enabled && response.has_focus() {
        super::widgets::paint_focus_ring(ui.painter(), rect, 12);
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
    let direction = vec2(1.0, -1.0).normalized();
    let perpendicular = vec2(-direction.y, direction.x);
    let tip = center - direction * size * 0.44;
    let bulb = center + direction * size * 0.30;
    let body_start = tip + direction * size * 0.12;
    let half = size * 0.085;

    painter.add(egui::Shape::convex_polygon(
        vec![
            body_start + perpendicular * half,
            body_start - perpendicular * half,
            bulb - perpendicular * half,
            bulb + perpendicular * half,
        ],
        color,
        Stroke::NONE,
    ));
    painter.circle_filled(bulb, size * 0.17, color);

    let collar = bulb - direction * size * 0.10;
    painter.line_segment(
        [
            collar - perpendicular * size * 0.15,
            collar + perpendicular * size * 0.15,
        ],
        Stroke::new(size * 0.07, color),
    );

    painter.add(egui::Shape::convex_polygon(
        vec![
            tip,
            body_start + perpendicular * half,
            body_start - perpendicular * half,
        ],
        color,
        Stroke::NONE,
    ));
}

#[derive(Clone, Copy)]
pub(crate) enum QuickTool {
    Crop,
    Invert,
    Rotate,
    Trash,
}

pub(crate) const QUICK_TOOL_SIZE: f32 = 38.0;
pub(crate) const QUICK_TOOL_GAP: f32 = 6.0;

pub(crate) const TOP_CONTROL_WIDTH: f32 = 64.0;
pub(crate) const TOP_CONTROL_HEIGHT: f32 = 32.0;

pub(crate) fn panel_side_button(
    ui: &mut egui::Ui,
    panel_on_right: bool,
    label: &str,
) -> egui::Response {
    let (rect, response) =
        ui.allocate_exact_size(vec2(TOP_CONTROL_WIDTH, TOP_CONTROL_HEIGHT), Sense::click());
    let hovered = response.hovered();
    let stroke = Stroke::new(
        1.0,
        if hovered {
            T::border_strong()
        } else {
            T::border()
        },
    );
    ui.painter().rect(
        rect,
        CornerRadius::same(10),
        if hovered {
            T::surface_3()
        } else {
            Color32::TRANSPARENT
        },
        stroke,
        StrokeKind::Inside,
    );

    let color = if hovered { T::text() } else { T::text_muted() };
    let icon = Rect::from_center_size(rect.center(), vec2(22.0, 16.0));
    ui.painter().rect_stroke(
        icon,
        CornerRadius::same(3),
        Stroke::new(1.25, color),
        StrokeKind::Inside,
    );
    let bar = if panel_on_right {
        Rect::from_min_max(
            pos2(icon.right() - 7.0, icon.top() + 1.0),
            pos2(icon.right() - 1.0, icon.bottom() - 1.0),
        )
    } else {
        Rect::from_min_max(
            pos2(icon.left() + 1.0, icon.top() + 1.0),
            pos2(icon.left() + 7.0, icon.bottom() - 1.0),
        )
    };
    ui.painter().rect_filled(bar, CornerRadius::same(1), color);

    response.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, label.to_owned()));
    if response.has_focus() {
        super::widgets::paint_focus_ring(ui.painter(), rect, 10);
    }
    response
        .on_hover_cursor(CursorIcon::PointingHand)
        .on_hover_text(label)
        .accessible_name(label)
}

pub(crate) fn theme_toggle_button(ui: &mut egui::Ui, light: bool, label: &str) -> egui::Response {
    let (rect, response) =
        ui.allocate_exact_size(vec2(TOP_CONTROL_WIDTH, TOP_CONTROL_HEIGHT), Sense::click());
    let hovered = response.hovered();
    let stroke = Stroke::new(
        1.0,
        if hovered {
            T::border_strong()
        } else {
            T::border()
        },
    );
    ui.painter().rect(
        rect,
        CornerRadius::same(10),
        if hovered {
            T::surface_3()
        } else {
            Color32::TRANSPARENT
        },
        stroke,
        StrokeKind::Inside,
    );

    let color = if hovered { T::text() } else { T::text_muted() };
    if light {
        paint_sun_icon(ui.painter(), rect.center(), color);
    } else {
        paint_moon_icon(ui.painter(), rect.center(), color);
    }

    response.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, label.to_owned()));
    if response.has_focus() {
        super::widgets::paint_focus_ring(ui.painter(), rect, 10);
    }
    response
        .on_hover_cursor(CursorIcon::PointingHand)
        .on_hover_text(label)
        .accessible_name(label)
}

fn paint_sun_icon(painter: &egui::Painter, center: egui::Pos2, color: Color32) {
    painter.circle_filled(center, 3.4, color);
    for index in 0..8 {
        let angle = index as f32 * PI / 4.0;
        let direction = vec2(angle.cos(), angle.sin());
        painter.line_segment(
            [center + direction * 5.6, center + direction * 7.4],
            Stroke::new(1.4, color),
        );
    }
}

fn paint_moon_icon(painter: &egui::Painter, center: egui::Pos2, color: Color32) {
    const STEPS: usize = 18;
    let outer_radius = 7.4;
    let inner_radius = 6.6;
    let inner_center = center + vec2(2.9, 0.0);
    let outer_from = 64.0_f32.to_radians();
    let outer_to = 296.0_f32.to_radians();
    let inner_from = 84.0_f32.to_radians();
    let inner_to = 276.0_f32.to_radians();

    let mut mesh = egui::Mesh::default();
    for step in 0..=STEPS {
        let t = step as f32 / STEPS as f32;
        let outer_angle = outer_from + (outer_to - outer_from) * t;
        let inner_angle = inner_from + (inner_to - inner_from) * t;
        mesh.colored_vertex(
            center + vec2(outer_angle.cos(), outer_angle.sin()) * outer_radius,
            color,
        );
        mesh.colored_vertex(
            inner_center + vec2(inner_angle.cos(), inner_angle.sin()) * inner_radius,
            color,
        );
        if step < STEPS {
            let index = step as u32 * 2;
            mesh.add_triangle(index, index + 1, index + 2);
            mesh.add_triangle(index + 1, index + 3, index + 2);
        }
    }
    painter.add(egui::Shape::mesh(mesh));
}

pub(crate) fn quick_tool_button(
    ui: &mut egui::Ui,
    tool: QuickTool,
    active: bool,
    enabled: bool,
    label: &str,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(
        vec2(QUICK_TOOL_SIZE, QUICK_TOOL_SIZE),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    let hovered = enabled && response.hovered();
    let fill = if active && enabled {
        T::accent()
    } else if hovered {
        T::surface_3()
    } else {
        T::surface_2()
    };
    let border = if active && enabled {
        T::accent()
    } else if hovered {
        T::border_strong()
    } else {
        T::border()
    };
    let icon = if !enabled {
        T::text_disabled()
    } else if active {
        T::accent_ink()
    } else {
        T::text_muted()
    };
    ui.painter().rect(
        rect,
        CornerRadius::same(12),
        fill,
        Stroke::new(1.0, border),
        StrokeKind::Inside,
    );
    match tool {
        QuickTool::Crop => paint_crop_icon(ui.painter(), rect, icon),
        QuickTool::Invert => paint_invert_icon(ui.painter(), rect, icon),
        QuickTool::Rotate => paint_rotate_icon(ui.painter(), rect, icon),
        QuickTool::Trash => paint_trash_icon(ui.painter(), rect.shrink(9.0), icon),
    }
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, enabled, label.to_owned()));
    if enabled && response.has_focus() {
        super::widgets::paint_focus_ring(ui.painter(), rect, 12);
    }
    if enabled {
        response
            .on_hover_cursor(CursorIcon::PointingHand)
            .on_hover_text(label)
    } else {
        response
    }
}

fn paint_crop_icon(painter: &egui::Painter, rect: Rect, color: Color32) {
    let inner = rect.shrink(9.0);
    let len = inner.width() * 0.32;
    let stroke = Stroke::new(1.7, color);
    for (corner, dx, dy) in [
        (inner.left_top(), 1.0, 1.0),
        (inner.right_top(), -1.0, 1.0),
        (inner.left_bottom(), 1.0, -1.0),
        (inner.right_bottom(), -1.0, -1.0),
    ] {
        painter.line_segment([corner, corner + vec2(dx * len, 0.0)], stroke);
        painter.line_segment([corner, corner + vec2(0.0, dy * len)], stroke);
    }
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
