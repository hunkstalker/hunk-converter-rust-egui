use eframe::egui::{
    self, Align2, CornerRadius, CursorIcon, FontId, Margin, Rect, Sense, Stroke, StrokeKind,
    TextEdit, vec2,
};
use egui::text::CCursorRange;
use egui::widgets::text_edit::TextEditState;

use crate::domain::{linked_height, linked_width, parse_dimension_expression};
use crate::theme as T;

const FIELD_HEIGHT: f32 = 30.0;
const MAX_SIDE: u32 = 16_384;

#[derive(Clone, Default)]
struct FieldState {
    editing: bool,
    buffer: String,
    error: bool,
    focus_pending: bool,
    drag_start_value: Option<f32>,
}

pub(super) struct DimensionLabels<'a> {
    pub width: &'a str,
    pub height: &'a str,
    pub hint: &'a str,
}

#[derive(Clone, Copy)]
pub(super) struct DimensionFieldsResponse {
    pub width_changed: bool,
    pub height_changed: bool,
    pub dragging: bool,
    pub committed: bool,
}

pub(super) fn dimension_fields(
    ui: &mut egui::Ui,
    width: &mut u32,
    height: &mut u32,
    aspect: f32,
    keep_aspect: bool,
    labels: DimensionLabels<'_>,
) -> DimensionFieldsResponse {
    let width_id = ui.make_persistent_id("resize_width_field");
    let height_id = ui.make_persistent_id("resize_height_field");
    let mut width_state = load_state(ui, width_id);
    let mut height_state = load_state(ui, height_id);

    let label_font = FontId::proportional(12.0);
    let width_label =
        ui.painter()
            .layout_no_wrap(labels.width.to_owned(), label_font.clone(), T::TEXT_MUTED);
    let height_label =
        ui.painter()
            .layout_no_wrap(labels.height.to_owned(), label_font, T::TEXT_MUTED);
    let available = ui.available_width();
    let gap = 8.0;
    let field_width = ((available - width_label.size().x - height_label.size().x - gap * 3.0)
        / 2.0)
        .clamp(56.0, 96.0);
    let (row_rect, _) = ui.allocate_exact_size(vec2(available, FIELD_HEIGHT), Sense::hover());

    let mut x = row_rect.left();
    let width_label_rect = Rect::from_min_size(
        egui::pos2(x, row_rect.top()),
        vec2(width_label.size().x, FIELD_HEIGHT),
    );
    x += width_label.size().x + gap;
    let width_rect = Rect::from_min_size(
        egui::pos2(x, row_rect.top()),
        vec2(field_width, FIELD_HEIGHT),
    );
    x += field_width + gap;
    let height_label_rect = Rect::from_min_size(
        egui::pos2(x, row_rect.top()),
        vec2(height_label.size().x, FIELD_HEIGHT),
    );
    x += height_label.size().x + gap;
    let height_rect = Rect::from_min_size(
        egui::pos2(x, row_rect.top()),
        vec2(field_width, FIELD_HEIGHT),
    );

    let mut width_changed = false;
    let mut height_changed = false;
    let mut committed = false;
    let mut dragging = false;
    let mut width_text_shown = false;
    let mut height_text_shown = false;

    let mut width_response = None;
    let mut height_response = None;

    if width_state.editing {
        width_text_shown = true;
        let (changed, committed_now, response) =
            show_editor(ui, width_id, width_rect, &mut width_state, width);
        width_changed |= changed;
        committed |= committed_now;
        width_response = Some(response);
    } else if height_state.editing {
        height_text_shown = true;
        let (changed, committed_now, response) =
            show_editor(ui, height_id, height_rect, &mut height_state, height);
        height_changed |= changed;
        committed |= committed_now;
        height_response = Some(response);
    }

    if !width_state.editing && !width_text_shown {
        let response = ui
            .interact(width_rect, width_id, Sense::click_and_drag())
            .on_hover_cursor(CursorIcon::ResizeVertical)
            .on_hover_text(labels.hint);
        if response.clicked()
            || (response.has_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter)))
        {
            if height_state.editing
                && let Some(parsed) = parse_dimension_expression(&height_state.buffer)
            {
                *height = parsed.clamp(1, MAX_SIDE);
                height_changed = true;
                committed = true;
            }
            height_state.editing = false;
            height_state.error = false;
            width_state.editing = true;
            width_state.buffer = width.to_string();
            width_state.error = false;
            width_state.focus_pending = true;
        } else if response.dragged() {
            dragging = true;
            let start = *width_state.drag_start_value.get_or_insert(*width as f32);
            let total_y = response
                .total_drag_delta()
                .map(|delta| delta.y)
                .unwrap_or(0.0);
            let speed = if ui.input(|input| input.modifiers.shift) {
                0.1
            } else {
                1.0
            };
            let next = (start - total_y * speed)
                .round()
                .clamp(1.0, MAX_SIDE as f32) as u32;
            if next != *width {
                *width = next;
                width_changed = true;
            }
        }
        if !response.dragged() {
            width_state.drag_start_value = None;
        }
        width_response = Some(response);
    }

    if !height_state.editing && !height_text_shown {
        let response = ui
            .interact(height_rect, height_id, Sense::click_and_drag())
            .on_hover_cursor(CursorIcon::ResizeVertical)
            .on_hover_text(labels.hint);
        if response.clicked()
            || (response.has_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter)))
        {
            if width_state.editing
                && let Some(parsed) = parse_dimension_expression(&width_state.buffer)
            {
                *width = parsed.clamp(1, MAX_SIDE);
                width_changed = true;
                committed = true;
            }
            width_state.editing = false;
            width_state.error = false;
            height_state.editing = true;
            height_state.buffer = height.to_string();
            height_state.error = false;
            height_state.focus_pending = true;
        } else if response.dragged() {
            dragging = true;
            let start = *height_state.drag_start_value.get_or_insert(*height as f32);
            let total_y = response
                .total_drag_delta()
                .map(|delta| delta.y)
                .unwrap_or(0.0);
            let speed = if ui.input(|input| input.modifiers.shift) {
                0.1
            } else {
                1.0
            };
            let next = (start - total_y * speed)
                .round()
                .clamp(1.0, MAX_SIDE as f32) as u32;
            if next != *height {
                *height = next;
                height_changed = true;
            }
        }
        if !response.dragged() {
            height_state.drag_start_value = None;
        }
        height_response = Some(response);
    }

    if keep_aspect {
        if width_changed {
            let next = linked_height(*width, aspect, MAX_SIDE);
            if *height != next {
                *height = next;
                height_changed = true;
            }
        } else if height_changed {
            let next = linked_width(*height, aspect, MAX_SIDE);
            if *width != next {
                *width = next;
                width_changed = true;
            }
        }
    }
    if width_state.focus_pending {
        width_state.buffer = width.to_string();
    }
    if height_state.focus_pending {
        height_state.buffer = height.to_string();
    }

    ui.painter().galley(
        width_label_rect.left_top() + vec2(0.0, (FIELD_HEIGHT - width_label.size().y) * 0.5),
        width_label,
        T::TEXT_MUTED,
    );
    ui.painter().galley(
        height_label_rect.left_top() + vec2(0.0, (FIELD_HEIGHT - height_label.size().y) * 0.5),
        height_label,
        T::TEXT_MUTED,
    );

    if !width_state.editing || !width_text_shown {
        paint_field(
            ui,
            width_rect,
            *width,
            width_response.as_ref(),
            labels.width,
            width_state.error,
        );
    }
    if !height_state.editing || !height_text_shown {
        paint_field(
            ui,
            height_rect,
            *height,
            height_response.as_ref(),
            labels.height,
            height_state.error,
        );
    }
    if width_state.error {
        ui.painter().rect_stroke(
            width_rect,
            CornerRadius::same(8),
            Stroke::new(1.2, T::DANGER),
            StrokeKind::Inside,
        );
    }
    if height_state.error {
        ui.painter().rect_stroke(
            height_rect,
            CornerRadius::same(8),
            Stroke::new(1.2, T::DANGER),
            StrokeKind::Inside,
        );
    }

    store_state(ui, width_id, width_state);
    store_state(ui, height_id, height_state);

    DimensionFieldsResponse {
        width_changed,
        height_changed,
        dragging,
        committed,
    }
}

fn show_editor(
    ui: &mut egui::Ui,
    id: egui::Id,
    rect: Rect,
    state: &mut FieldState,
    value: &mut u32,
) -> (bool, bool, egui::Response) {
    let mut buffer = state.buffer.clone();
    let response = ui.put(
        rect,
        TextEdit::singleline(&mut buffer)
            .id(id)
            .font(FontId::proportional(12.5))
            .align(Align2::RIGHT_CENTER)
            .margin(Margin::symmetric(6, 4))
            .background_color(T::BG_SOFT)
            .text_color(T::TEXT),
    );
    state.buffer = buffer;
    if state.focus_pending {
        response.request_focus();
        if let Some(mut edit_state) = TextEditState::load(ui.ctx(), id) {
            edit_state
                .cursor
                .set_char_range(Some(CCursorRange::select_all_str(&state.buffer)));
            edit_state.store(ui.ctx(), id);
        }
        state.focus_pending = false;
    }
    if response.changed() {
        state.error = false;
    }

    let (escape, enter) = ui.input(|input| {
        (
            input.key_pressed(egui::Key::Escape),
            input.key_pressed(egui::Key::Enter),
        )
    });
    if escape {
        state.editing = false;
        state.error = false;
        response.surrender_focus();
        return (false, false, response);
    }
    if enter || response.lost_focus() {
        if let Some(parsed) = parse_dimension_expression(&state.buffer) {
            *value = parsed.clamp(1, MAX_SIDE);
            state.editing = false;
            state.error = false;
            response.surrender_focus();
            return (true, true, response);
        }
        if enter {
            state.error = true;
            response.request_focus();
        } else {
            state.editing = false;
            state.error = false;
        }
    }
    (false, false, response)
}

fn paint_field(
    ui: &egui::Ui,
    rect: Rect,
    value: u32,
    response: Option<&egui::Response>,
    label: &str,
    error: bool,
) {
    let hovered = response.is_some_and(|response| response.hovered());
    let focused = response.is_some_and(|response| response.has_focus());
    let fill = if hovered { T::SURFACE_3 } else { T::BG_SOFT };
    let border = if error {
        T::DANGER
    } else if focused {
        T::ACCENT
    } else {
        T::BORDER
    };
    ui.painter().rect_filled(rect, CornerRadius::same(8), fill);
    ui.painter().rect_stroke(
        rect,
        CornerRadius::same(8),
        Stroke::new(1.0, border),
        StrokeKind::Inside,
    );
    ui.painter().text(
        rect.right_center() - vec2(8.0, 0.0),
        Align2::RIGHT_CENTER,
        value.to_string(),
        FontId::proportional(12.5),
        T::TEXT,
    );
    if let Some(response) = response {
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::Role::SpinButton, true, format!("{label}: {value}"))
        });
        if focused {
            super::widgets::paint_focus_ring(ui.painter(), rect, 8);
        }
    }
}

fn load_state(ui: &egui::Ui, id: egui::Id) -> FieldState {
    ui.ctx()
        .data_mut(|data| data.get_temp_mut_or_default::<FieldState>(id).clone())
}

fn store_state(ui: &egui::Ui, id: egui::Id, state: FieldState) {
    ui.ctx().data_mut(|data| {
        *data.get_temp_mut_or_default::<FieldState>(id) = state;
    });
}
