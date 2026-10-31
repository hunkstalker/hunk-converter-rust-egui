use eframe::egui::{self, RichText};

use crate::domain::{FormatSelection, OutputFormat};
use crate::i18n::{self, K, Lang};
use crate::theme as T;

pub(super) struct FormatSelector<'a> {
    pub selection: FormatSelection,
    pub current: OutputFormat,
    pub language: Lang,
    pub source_format: Option<&'a str>,
}

pub(super) fn format_selector(
    ui: &mut egui::Ui,
    spec: FormatSelector<'_>,
) -> Option<FormatSelection> {
    let FormatSelector {
        selection,
        current,
        language,
        source_format,
    } = spec;
    let auto_label = i18n::t(language, K::FormatAuto);
    let selected_text = match selection {
        FormatSelection::Auto => format!("{auto_label} · {}", current.label()),
        FormatSelection::Fixed(_) => format!(
            "{} · {}",
            current.label(),
            i18n::format_hint(language, current)
        ),
    };

    let mut choice = selection;
    let response = egui::ComboBox::from_id_salt("output_format_selector")
        .selected_text(RichText::new(&selected_text).size(12.0).color(T::TEXT))
        .width(ui.available_width())
        .height(320.0)
        .show_ui(ui, |ui| {
            let auto_selected = choice == FormatSelection::Auto;
            ui.selectable_value(
                &mut choice,
                FormatSelection::Auto,
                RichText::new(auto_label).color(if auto_selected {
                    T::ACCENT_INK
                } else {
                    T::TEXT
                }),
            );
            ui.separator();
            for format in OutputFormat::ALL {
                let selected = choice == FormatSelection::Fixed(format);
                let label = format!(
                    "{} · {}",
                    format.label(),
                    i18n::format_hint(language, format)
                );
                ui.selectable_value(
                    &mut choice,
                    FormatSelection::Fixed(format),
                    RichText::new(label).color(if selected { T::ACCENT_INK } else { T::TEXT }),
                );
            }
        })
        .response;
    let hint = if selection == FormatSelection::Auto {
        i18n::t(language, K::FormatAutoHint)
    } else {
        i18n::format_hint(language, current)
    };
    let response = response.on_hover_text(hint);

    if response.has_focus() {
        super::widgets::paint_focus_ring(ui.painter(), response.rect, 10);
    }
    response.accessible_name(format!(
        "{}: {selected_text}",
        i18n::t(language, K::OutputFormat)
    ));

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
