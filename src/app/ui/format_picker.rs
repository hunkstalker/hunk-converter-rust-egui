use eframe::egui::{self, CornerRadius, FontId, RichText, Sense, Stroke, StrokeKind, vec2};

use crate::domain::{FormatSelection, OutputFormat};
use crate::i18n::{self, K, Lang};
use crate::theme as T;

pub(super) struct FormatSelector<'a> {
    pub selection: FormatSelection,
    pub current: OutputFormat,
    pub language: Lang,
    pub source_format: Option<&'a str>,
}

pub(super) fn format_grid(ui: &mut egui::Ui, spec: FormatSelector<'_>) -> Option<FormatSelection> {
    let FormatSelector {
        selection,
        current,
        language,
        source_format,
    } = spec;
    let options = [
        FormatSelection::Fixed(OutputFormat::Webp),
        FormatSelection::Fixed(OutputFormat::Jpg),
        FormatSelection::Fixed(OutputFormat::Png),
        FormatSelection::Fixed(OutputFormat::Avif),
        FormatSelection::Auto,
        FormatSelection::Fixed(OutputFormat::Ico),
    ];
    let gap = 6.0;
    let columns = 3;
    let width = ((ui.available_width() - gap * (columns - 1) as f32) / columns as f32).max(64.0);
    let mut choice = selection;

    ui.scope(|ui| {
        ui.spacing_mut().item_spacing = vec2(gap, gap);
        for row in options.chunks(columns) {
            ui.horizontal(|ui| {
                for option in row {
                    if format_card(ui, *option, choice, width, language) {
                        choice = *option;
                    }
                }
            });
        }
    });

    let input_matches = source_format
        .and_then(OutputFormat::from_input_label)
        .is_some_and(|input_format| input_format == current);
    if input_matches && current.is_lossy() {
        ui.label(
            RichText::new(i18n::t(language, K::RecompressWarning))
                .size(9.0)
                .color(T::WARNING),
        );
    }

    (choice != selection).then_some(choice)
}

fn format_card(
    ui: &mut egui::Ui,
    option: FormatSelection,
    selected: FormatSelection,
    width: f32,
    language: Lang,
) -> bool {
    let (label, hint, full_hint) = match option {
        FormatSelection::Auto => (
            i18n::t(language, K::FormatAutoShort),
            i18n::t(language, K::FormatAutoCardHint),
            i18n::t(language, K::FormatAutoHint),
        ),
        FormatSelection::Fixed(format) => (
            format.label(),
            i18n::format_hint(language, format),
            i18n::format_hint(language, format),
        ),
    };
    let is_selected = option == selected;
    let (rect, response) = ui.allocate_exact_size(vec2(width, 52.0), Sense::click());
    let hovered = response.hovered();
    let fill = if is_selected {
        T::ACCENT_SOFT
    } else if hovered {
        T::SURFACE_3
    } else {
        T::SURFACE_2
    };
    let stroke = if is_selected {
        Stroke::new(1.2, T::ACCENT)
    } else if hovered {
        Stroke::new(1.0, T::BORDER_STRONG)
    } else {
        Stroke::new(1.0, T::BORDER)
    };
    let painter = ui.painter();
    painter.rect(
        rect,
        CornerRadius::same(10),
        fill,
        stroke,
        StrokeKind::Inside,
    );
    painter.text(
        rect.left_top() + vec2(10.0, 9.0),
        egui::Align2::LEFT_TOP,
        label,
        FontId::proportional(12.0),
        T::TEXT,
    );
    painter.text(
        rect.left_top() + vec2(10.0, 27.0),
        egui::Align2::LEFT_TOP,
        hint,
        FontId::proportional(9.0),
        if is_selected {
            T::ACCENT
        } else {
            T::TEXT_MUTED
        },
    );

    if response.has_focus() {
        super::widgets::paint_focus_ring(ui.painter(), rect, 10);
    }
    let response = response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(full_hint);
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::Role::RadioButton,
            true,
            is_selected,
            format!("{label} · {hint}"),
        )
    });
    let clicked = response.clicked();
    response.accessible_name(format!("{}: {}", i18n::t(language, K::OutputFormat), label));

    clicked && option != selected
}
