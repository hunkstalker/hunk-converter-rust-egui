use std::path::Path;

use crate::domain::FormatSelection;
use crate::domain::color::parse_hex_rgba;
use crate::domain::history::{HISTORY_LIMIT, HistoryEntry};
use crate::i18n::Lang;

use super::{AdvancedSettings, ConfigLoad, ConfigWarning, PanelSide, Settings, WindowState};

pub(super) fn read(path: &Path) -> Result<ConfigLoad, crate::domain::ConfigError> {
    let text =
        std::fs::read_to_string(path).map_err(|source| crate::domain::ConfigError::Read {
            path: path.to_path_buf(),
            source,
        })?;
    Ok(parse(&text))
}

pub(super) fn parse(text: &str) -> ConfigLoad {
    let mut settings = Settings {
        language: Lang::detect(),
        history: Vec::new(),
        maximized: true,
        panel_side: PanelSide::Right,
        color_panel_pos: None,
        output_format: FormatSelection::Auto,
        advanced: AdvancedSettings::default(),
        window: WindowState::default(),
    };
    let mut warnings = Vec::new();
    let mut section = String::new();
    let mut maximized_seen = false;
    let mut color_panel_x = None;
    let mut color_panel_y = None;
    let mut color_panel_x_line = None;
    let mut color_panel_y_line = None;

    for (index, raw_line) in text.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section = line[1..line.len() - 1].trim().to_ascii_lowercase();
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            warnings.push(ConfigWarning::new(line_number, "línea sin '='; se ignora"));
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim();

        if section == "general" && key == "language" {
            match Lang::from_code(value) {
                Some(language) => settings.language = language,
                None => warnings.push(ConfigWarning::new(
                    line_number,
                    format!("idioma desconocido: {value}"),
                )),
            }
        } else if section == "general" && key == "maximized" {
            settings.maximized = parse_bool(value);
            maximized_seen = true;
        } else if section == "general" && key == "panel_side" {
            match PanelSide::from_code(value) {
                Some(side) => settings.panel_side = side,
                None => warnings.push(ConfigWarning::new(
                    line_number,
                    format!("lado del panel desconocido: {value}"),
                )),
            }
        } else if section == "general" && key == "fullscreen" && !maximized_seen {
            settings.maximized = parse_bool(value);
        } else if section == "output" && key == "format" {
            match FormatSelection::from_code(value) {
                Some(format) => settings.output_format = format,
                None => warnings.push(ConfigWarning::new(
                    line_number,
                    format!("formato de salida desconocido: {value}"),
                )),
            }
        } else if section == "window" {
            match key.as_str() {
                "x" | "y" => {
                    let parsed = parse_coordinate(value);
                    if parsed.is_none() {
                        warnings.push(ConfigWarning::new(
                            line_number,
                            format!("coordenada de ventana no válida: {value}"),
                        ));
                    } else if key == "x" {
                        settings.window.x = parsed;
                    } else {
                        settings.window.y = parsed;
                    }
                }
                "width" | "height" => {
                    let Some(parsed) = parse_size(value) else {
                        warnings.push(ConfigWarning::new(
                            line_number,
                            format!("tamaño de ventana no válido: {value}"),
                        ));
                        continue;
                    };
                    if key == "width" {
                        settings.window.width = parsed;
                    } else {
                        settings.window.height = parsed;
                    }
                }
                "color_panel_x" | "color_panel_y" => {
                    let Some(parsed) = parse_coordinate(value) else {
                        warnings.push(ConfigWarning::new(
                            line_number,
                            format!("posición del panel de color no válida: {value}"),
                        ));
                        continue;
                    };
                    if key == "color_panel_x" {
                        color_panel_x = Some(parsed);
                        color_panel_x_line = Some(line_number);
                    } else {
                        color_panel_y = Some(parsed);
                        color_panel_y_line = Some(line_number);
                    }
                }
                _ => {}
            }
        } else if section == "history" && key.starts_with("color") {
            let Some(index) = key
                .strip_prefix("color")
                .and_then(|index| index.parse::<usize>().ok())
            else {
                warnings.push(ConfigWarning::new(
                    line_number,
                    format!("clave de color no válida: {key}"),
                ));
                continue;
            };
            let Some(entry) = parse_history_entry(value) else {
                warnings.push(ConfigWarning::new(
                    line_number,
                    format!("color no válido: {value}"),
                ));
                continue;
            };
            if index == 0 {
                warnings.push(ConfigWarning::new(
                    line_number,
                    "los colores empiezan en color1",
                ));
                continue;
            }
            if settings.history.len() >= HISTORY_LIMIT {
                warnings.push(ConfigWarning::new(
                    line_number,
                    "hay más colores que el límite del historial",
                ));
                continue;
            }
            if settings
                .history
                .iter()
                .any(|existing| existing.rgba == entry.rgba)
            {
                warnings.push(ConfigWarning::new(
                    line_number,
                    "color duplicado; se ignora",
                ));
                continue;
            }
            settings.history.push(entry);
        }
    }

    match (color_panel_x, color_panel_y) {
        (Some(x), Some(y)) => settings.color_panel_pos = Some((x, y)),
        (Some(_), None) => warnings.push(ConfigWarning::new(
            color_panel_x_line.unwrap_or_default(),
            "falta color_panel_y",
        )),
        (None, Some(_)) => warnings.push(ConfigWarning::new(
            color_panel_y_line.unwrap_or_default(),
            "falta color_panel_x",
        )),
        (None, None) => {}
    }

    ConfigLoad { settings, warnings }
}

pub(super) fn serialize(settings: &Settings) -> String {
    let mut text = format!(
        "[general]\nlanguage={}\nmaximized={}\npanel_side={}\n\n[output]\nformat={}\n\n[window]\n",
        settings.language.code(),
        u8::from(settings.maximized),
        settings.panel_side.code(),
        settings.output_format.code(),
    );
    if let Some(x) = settings.window.x {
        text.push_str(&format!("x={x}\n"));
    }
    if let Some(y) = settings.window.y {
        text.push_str(&format!("y={y}\n"));
    }
    text.push_str(&format!(
        "width={}\nheight={}\n",
        settings.window.width, settings.window.height
    ));
    if let Some((x, y)) = settings.color_panel_pos {
        text.push_str(&format!("color_panel_x={x}\ncolor_panel_y={y}\n"));
    }
    text.push_str(&format!("\n[history]\ncount={}\n", settings.history.len()));
    for (index, entry) in settings.history.iter().take(HISTORY_LIMIT).enumerate() {
        text.push_str(&format!(
            "color{}={}|{}\n",
            index + 1,
            crate::domain::hex_rgba(entry.rgba),
            u8::from(entry.locked)
        ));
    }
    text
}

fn parse_bool(value: &str) -> bool {
    !matches!(value.to_ascii_lowercase().as_str(), "0" | "false" | "no")
}

fn parse_coordinate(value: &str) -> Option<f32> {
    value.parse::<f32>().ok().filter(|value| value.is_finite())
}

fn parse_size(value: &str) -> Option<f32> {
    parse_coordinate(value).filter(|value| *value > 0.0)
}

fn parse_history_entry(value: &str) -> Option<HistoryEntry> {
    let (color, locked) = value.split_once('|').unwrap_or((value, "0"));
    let rgba = parse_hex_rgba(color)?;
    Some(HistoryEntry::new(rgba, locked.trim() == "1"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locked_colors_survive_a_round_trip() {
        let text = "[general]\nlanguage=es\nmaximized=0\n\n[history]\ncount=2\ncolor1=#7C7E78FF|1\ncolor2=#B38E75FF|0\n";
        let loaded = parse(text);
        assert_eq!(loaded.settings.language, Lang::Es);
        assert!(!loaded.settings.maximized);
        assert_eq!(loaded.settings.history.len(), 2);
        assert!(loaded.settings.history[0].locked);
        assert!(!loaded.settings.history[1].locked);

        let round_trip = parse(&serialize(&loaded.settings));
        assert_eq!(round_trip.settings.history, loaded.settings.history);
    }

    #[test]
    fn advanced_and_preprocess_settings_are_not_persisted() {
        let text = "[advanced]\njpg_progressive=0\njpg_subsampling=444\npng_level=6\npng_interlace=1\nwebp_lossless=1\nwebp_method=6\nwebp_alpha_quality=80\navif_speed=10\navif_alpha_quality=90\npreprocess_resize=1\npreprocess_width=800\npreprocess_height=600\npreprocess_keep_aspect=0\npreprocess_rotation=270\npreprocess_palette=64\n";
        let loaded = parse(text);
        assert_eq!(loaded.settings.advanced, AdvancedSettings::default());
        assert!(loaded.warnings.is_empty());

        let mut settings = Settings::default();
        settings.advanced.jpg_progressive = false;
        settings.advanced.preprocess.resize_enabled = true;
        settings.advanced.preprocess.width = 800;
        let serialized = serialize(&settings);
        assert!(!serialized.contains("[advanced]"));
        assert!(!serialized.contains("preprocess_"));
        assert!(!serialized.contains("jpg_progressive"));
        assert_eq!(
            parse(&serialized).settings.advanced,
            AdvancedSettings::default()
        );
    }

    #[test]
    fn parses_window_position_and_size() {
        let loaded = parse("[window]\nx=-20\ny=30\nwidth=1000\nheight=700\n");
        assert_eq!(loaded.settings.window.x, Some(-20.0));
        assert_eq!(loaded.settings.window.y, Some(30.0));
        assert_eq!(loaded.settings.window.width, 1000.0);
        assert_eq!(loaded.settings.window.height, 700.0);
        assert!(loaded.warnings.is_empty());
    }

    #[test]
    fn rejects_invalid_window_size_with_warning() {
        let loaded = parse("[window]\nwidth=0\nheight=no\n");
        assert_eq!(loaded.settings.window.width, 1280.0);
        assert_eq!(loaded.settings.window.height, 800.0);
        assert_eq!(loaded.warnings.len(), 2);
    }

    #[test]
    fn parses_output_format_and_falls_back_to_auto() {
        let loaded = parse(
            "[output]
format=auto
",
        );
        assert_eq!(loaded.settings.output_format, FormatSelection::Auto);
        assert!(loaded.warnings.is_empty());

        let loaded = parse(
            "[output]
format=avif
",
        );
        assert_eq!(
            loaded.settings.output_format,
            FormatSelection::Fixed(crate::domain::OutputFormat::Avif)
        );

        let loaded = parse(
            "[output]
format=desconocido
",
        );
        assert_eq!(loaded.settings.output_format, FormatSelection::Auto);
        assert_eq!(loaded.warnings.len(), 1);

        let loaded = parse(
            "[output]
format=jxl
",
        );
        assert_eq!(loaded.settings.output_format, FormatSelection::Auto);
        assert_eq!(loaded.warnings.len(), 1);
    }

    #[test]
    fn old_fullscreen_key_is_still_accepted() {
        let loaded = parse("[general]\nfullscreen=0\n");
        assert!(!loaded.settings.maximized);

        let loaded = parse("[general]\nfullscreen=1\nmaximized=0\n");
        assert!(!loaded.settings.maximized);
    }

    #[test]
    fn malformed_history_entries_are_reported_not_silenced() {
        let loaded = parse("[history]\ncolor1=no-es-un-color|1\ncolor2=#112233FF|1\n");
        assert_eq!(loaded.settings.history.len(), 1);
        assert!(loaded.settings.history[0].locked);
        assert_eq!(loaded.warnings.len(), 1);
    }
}
