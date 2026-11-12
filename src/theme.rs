use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use egui::{
    Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Frame, Margin, Shadow,
    Stroke, TextStyle,
};

pub const SMALL_TEXT_SIZE: f32 = 12.0;
pub const DEFAULT_PROFILE_ID: &str = "dark";

#[derive(Clone, Copy)]
pub struct Palette {
    pub bg: Color32,
    pub bg_soft: Color32,
    pub surface: Color32,
    pub surface_soft: Color32,
    pub surface_2: Color32,
    pub surface_3: Color32,
    pub border: Color32,
    pub border_strong: Color32,
    pub text: Color32,
    pub text_muted: Color32,
    pub text_faint: Color32,
    pub text_disabled: Color32,
    pub accent: Color32,
    pub accent_hover: Color32,
    pub accent_pressed: Color32,
    pub accent_ink: Color32,
    pub accent_soft: Color32,
    pub warning: Color32,
    pub danger: Color32,
    pub checker_a: Color32,
    pub checker_b: Color32,
    pub window_shadow: Color32,
    pub popup_shadow: Color32,
    pub floating_shadow: Color32,
    pub toast_error_bg: Color32,
    pub toast_error_border: Color32,
    pub crop_shade: Color32,
    pub crop_thirds: Color32,
    pub crop_border: Color32,
    pub reference_frame: Color32,
    pub split_line: Color32,
    pub split_shadow: Color32,
    pub split_handle_bg: Color32,
    pub split_handle_ring: Color32,
    pub sample_ring: Color32,
    pub sample_shadow: Color32,
    pub original_badge_bg: Color32,
    pub original_badge_fg: Color32,
    pub lock_badge: Color32,
}

pub struct ThemeProfile {
    pub id: &'static str,
    pub label: &'static str,
    pub palette: Palette,
}

pub const DARK: Palette = Palette {
    bg: Color32::from_rgb(10, 13, 16),
    bg_soft: Color32::from_rgb(13, 17, 22),
    surface: Color32::from_rgb(17, 22, 28),
    surface_soft: Color32::from_rgb(16, 22, 28),
    surface_2: Color32::from_rgb(23, 30, 37),
    surface_3: Color32::from_rgb(30, 39, 48),
    border: Color32::from_rgb(39, 49, 59),
    border_strong: Color32::from_rgb(55, 68, 82),
    text: Color32::from_rgb(241, 245, 242),
    text_muted: Color32::from_rgb(146, 158, 170),
    text_faint: Color32::from_rgb(132, 144, 156),
    text_disabled: Color32::from_rgb(103, 115, 127),
    accent: Color32::from_rgb(200, 244, 109),
    accent_hover: Color32::from_rgb(216, 255, 144),
    accent_pressed: Color32::from_rgb(180, 226, 87),
    accent_ink: Color32::from_rgb(17, 23, 11),
    accent_soft: Color32::from_rgb(27, 41, 24),
    warning: Color32::from_rgb(255, 190, 122),
    danger: Color32::from_rgb(255, 126, 126),
    checker_a: Color32::from_rgb(32, 40, 48),
    checker_b: Color32::from_rgb(39, 50, 60),
    window_shadow: Color32::from_black_alpha(110),
    popup_shadow: Color32::from_black_alpha(90),
    floating_shadow: Color32::from_black_alpha(120),
    toast_error_bg: Color32::from_rgb(42, 24, 26),
    toast_error_border: Color32::from_rgba_unmultiplied_const(255, 126, 126, 140),
    crop_shade: Color32::from_black_alpha(155),
    crop_thirds: Color32::from_rgba_unmultiplied_const(255, 255, 255, 70),
    crop_border: Color32::from_rgba_unmultiplied_const(255, 255, 255, 180),
    reference_frame: Color32::from_rgba_unmultiplied_const(200, 244, 109, 150),
    split_line: Color32::from_rgba_unmultiplied_const(255, 255, 255, 210),
    split_shadow: Color32::from_black_alpha(80),
    split_handle_bg: Color32::from_rgb(13, 17, 22),
    split_handle_ring: Color32::from_rgba_unmultiplied_const(255, 255, 255, 150),
    sample_ring: Color32::WHITE,
    sample_shadow: Color32::from_black_alpha(180),
    original_badge_bg: Color32::from_rgba_unmultiplied_const(10, 13, 16, 225),
    original_badge_fg: Color32::from_rgb(241, 245, 242),
    lock_badge: Color32::from_black_alpha(190),
};

pub const LIGHT: Palette = Palette {
    bg: Color32::from_rgb(243, 241, 234),
    bg_soft: Color32::from_rgb(235, 232, 223),
    surface: Color32::from_rgb(250, 249, 245),
    surface_soft: Color32::from_rgb(242, 240, 233),
    surface_2: Color32::from_rgb(238, 236, 228),
    surface_3: Color32::from_rgb(228, 225, 215),
    border: Color32::from_rgb(211, 207, 196),
    border_strong: Color32::from_rgb(184, 178, 166),
    text: Color32::from_rgb(12, 16, 11),
    text_muted: Color32::from_rgb(38, 46, 38),
    text_faint: Color32::from_rgb(52, 61, 52),
    text_disabled: Color32::from_rgb(96, 106, 96),
    accent: Color32::from_rgb(66, 88, 22),
    accent_hover: Color32::from_rgb(76, 99, 28),
    accent_pressed: Color32::from_rgb(55, 74, 18),
    accent_ink: Color32::from_rgb(247, 248, 238),
    accent_soft: Color32::from_rgb(230, 236, 201),
    warning: Color32::from_rgb(138, 90, 0),
    danger: Color32::from_rgb(180, 35, 24),
    checker_a: Color32::from_rgb(228, 225, 216),
    checker_b: Color32::from_rgb(215, 211, 200),
    window_shadow: Color32::from_black_alpha(30),
    popup_shadow: Color32::from_black_alpha(20),
    floating_shadow: Color32::from_black_alpha(50),
    toast_error_bg: Color32::from_rgb(253, 236, 234),
    toast_error_border: Color32::from_rgba_unmultiplied_const(180, 35, 24, 140),
    crop_shade: Color32::from_black_alpha(120),
    crop_thirds: Color32::from_black_alpha(90),
    crop_border: Color32::from_black_alpha(150),
    reference_frame: Color32::from_rgba_unmultiplied_const(66, 88, 22, 160),
    split_line: Color32::from_rgba_unmultiplied_const(255, 255, 255, 235),
    split_shadow: Color32::from_black_alpha(90),
    split_handle_bg: Color32::from_rgb(250, 249, 245),
    split_handle_ring: Color32::from_rgb(184, 178, 166),
    sample_ring: Color32::WHITE,
    sample_shadow: Color32::from_black_alpha(180),
    original_badge_bg: Color32::from_rgba_unmultiplied_const(29, 35, 28, 220),
    original_badge_fg: Color32::from_rgb(247, 248, 238),
    lock_badge: Color32::from_black_alpha(150),
};

pub static PROFILES: &[ThemeProfile] = &[
    ThemeProfile {
        id: "dark",
        label: "Dark",
        palette: DARK,
    },
    ThemeProfile {
        id: "light",
        label: "Light",
        palette: LIGHT,
    },
];

static ACTIVE_PROFILE: AtomicUsize = AtomicUsize::new(0);

pub fn profile(id: &str) -> Option<&'static ThemeProfile> {
    PROFILES
        .iter()
        .find(|profile| profile.id.eq_ignore_ascii_case(id.trim()))
}

pub fn active_profile() -> &'static ThemeProfile {
    &PROFILES[ACTIVE_PROFILE
        .load(Ordering::Relaxed)
        .min(PROFILES.len().saturating_sub(1))]
}

pub fn palette() -> &'static Palette {
    &active_profile().palette
}

pub fn is_light() -> bool {
    active_profile().id == "light"
}

pub fn set_profile(id: &str) -> bool {
    let Some(index) = PROFILES
        .iter()
        .position(|profile| profile.id.eq_ignore_ascii_case(id.trim()))
    else {
        return false;
    };
    ACTIVE_PROFILE.store(index, Ordering::Relaxed);
    true
}

pub fn toggled_profile_id(current: &str) -> &'static str {
    if current.eq_ignore_ascii_case("light") {
        "dark"
    } else {
        "light"
    }
}

macro_rules! palette_accessors {
    ($($name:ident => $field:ident),+ $(,)?) => {
        $(
            pub fn $name() -> Color32 {
                palette().$field
            }
        )+
    };
}

palette_accessors! {
    bg => bg,
    bg_soft => bg_soft,
    surface => surface,
    surface_soft => surface_soft,
    surface_2 => surface_2,
    surface_3 => surface_3,
    border => border,
    border_strong => border_strong,
    text => text,
    text_muted => text_muted,
    text_faint => text_faint,
    text_disabled => text_disabled,
    accent => accent,
    accent_hover => accent_hover,
    accent_pressed => accent_pressed,
    accent_ink => accent_ink,
    accent_soft => accent_soft,
    warning => warning,
    danger => danger,
    checker_a => checker_a,
    checker_b => checker_b,
    window_shadow => window_shadow,
    popup_shadow => popup_shadow,
    floating_shadow => floating_shadow,
    toast_error_bg => toast_error_bg,
    toast_error_border => toast_error_border,
    crop_shade => crop_shade,
    crop_thirds => crop_thirds,
    crop_border => crop_border,
    reference_frame => reference_frame,
    split_line => split_line,
    split_shadow => split_shadow,
    split_handle_bg => split_handle_bg,
    split_handle_ring => split_handle_ring,
    sample_ring => sample_ring,
    sample_shadow => sample_shadow,
    original_badge_bg => original_badge_bg,
    original_badge_fg => original_badge_fg,
    lock_badge => lock_badge,
}

const MANROPE: &[u8] = include_bytes!("../assets/fonts/Manrope.ttf");

pub fn install(ctx: &egui::Context, profile_id: &str) {
    // La barra y los bordes nativos los gestiona el sistema operativo. El
    // perfil de HUNK solo debe cambiar el contenido de la ventana.
    ctx.options_mut(|options| options.sync_window_theme = false);

    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "manrope".to_owned(),
        Arc::new(FontData::from_static(MANROPE)),
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "manrope".to_owned());
    ctx.set_fonts(fonts);

    apply(ctx, profile_id);
}

pub fn apply(ctx: &egui::Context, profile_id: &str) {
    let profile = profile(profile_id)
        .or_else(|| profile(DEFAULT_PROFILE_ID))
        .expect("dark theme must exist");
    ACTIVE_PROFILE.store(
        PROFILES
            .iter()
            .position(|candidate| candidate.id == profile.id)
            .unwrap_or(0),
        Ordering::Relaxed,
    );

    let palette = &profile.palette;
    let theme = if profile.id == "light" {
        egui::Theme::Light
    } else {
        egui::Theme::Dark
    };
    let mut style = (*ctx.global_style()).clone();
    style
        .text_styles
        .insert(TextStyle::Body, FontId::new(13.5, FontFamily::Proportional));
    style.text_styles.insert(
        TextStyle::Button,
        FontId::new(13.5, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Small,
        FontId::new(11.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(17.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Monospace,
        FontId::new(11.0, FontFamily::Monospace),
    );
    style.spacing.item_spacing = egui::vec2(10.0, 8.0);
    style.spacing.button_padding = egui::vec2(14.0, 8.0);
    style.spacing.slider_width = 230.0;
    style.spacing.interact_size.y = 32.0;
    style.spacing.scroll = egui::style::ScrollStyle::solid();
    style.spacing.scroll.bar_width = 8.0;

    let visuals = &mut style.visuals;
    *visuals = if theme == egui::Theme::Light {
        egui::Visuals::light()
    } else {
        egui::Visuals::dark()
    };
    visuals.dark_mode = theme == egui::Theme::Dark;
    visuals.panel_fill = palette.bg;
    visuals.window_fill = palette.surface;
    visuals.extreme_bg_color = palette.bg_soft;
    visuals.faint_bg_color = palette.surface_2;
    visuals.window_corner_radius = CornerRadius::same(16);
    visuals.menu_corner_radius = CornerRadius::same(10);
    visuals.window_stroke = Stroke::new(1.0, palette.border);
    visuals.window_shadow = Shadow {
        offset: [0, 14],
        blur: 30,
        spread: 0,
        color: palette.window_shadow,
    };
    visuals.popup_shadow = Shadow {
        offset: [0, 8],
        blur: 18,
        spread: 0,
        color: palette.popup_shadow,
    };
    visuals.override_text_color = Some(palette.text);
    visuals.weak_text_alpha = if theme == egui::Theme::Light {
        0.95
    } else {
        0.82
    };
    visuals.selection.bg_fill = palette.accent;
    visuals.selection.stroke = Stroke::new(1.0, palette.accent_ink);
    visuals.hyperlink_color = palette.accent;
    visuals.slider_trailing_fill = true;
    visuals.error_fg_color = palette.danger;
    visuals.warn_fg_color = palette.warning;

    let widgets = &mut visuals.widgets;
    widgets.noninteractive.bg_fill = palette.surface;
    widgets.noninteractive.weak_bg_fill = palette.surface;
    widgets.noninteractive.bg_stroke = Stroke::new(1.0, palette.border);
    widgets.noninteractive.fg_stroke = Stroke::new(1.0, palette.text);
    widgets.noninteractive.corner_radius = CornerRadius::same(10);

    widgets.inactive.bg_fill = palette.surface_3;
    widgets.inactive.weak_bg_fill = palette.surface_2;
    widgets.inactive.bg_stroke = Stroke::new(1.0, palette.border);
    widgets.inactive.fg_stroke = Stroke::new(1.0, palette.text);
    widgets.inactive.corner_radius = CornerRadius::same(10);
    widgets.inactive.expansion = 0.0;

    widgets.hovered.bg_fill = palette.surface_3;
    widgets.hovered.weak_bg_fill = palette.surface_3;
    widgets.hovered.bg_stroke = Stroke::new(1.0, palette.border_strong);
    widgets.hovered.fg_stroke = Stroke::new(1.0, palette.text);
    widgets.hovered.corner_radius = CornerRadius::same(10);
    widgets.hovered.expansion = 0.0;

    widgets.active.bg_fill = palette.surface_2;
    widgets.active.weak_bg_fill = palette.surface_2;
    widgets.active.bg_stroke = Stroke::new(1.0, palette.accent);
    widgets.active.fg_stroke = Stroke::new(1.0, palette.text);
    widgets.active.corner_radius = CornerRadius::same(10);
    widgets.active.expansion = 0.0;

    widgets.open.bg_fill = palette.surface_3;
    widgets.open.weak_bg_fill = palette.surface_3;
    widgets.open.bg_stroke = Stroke::new(1.0, palette.border_strong);
    widgets.open.fg_stroke = Stroke::new(1.0, palette.text);
    widgets.open.corner_radius = CornerRadius::same(10);

    ctx.set_theme(theme);
    ctx.set_style_of(theme, style);
    ctx.request_repaint();
}

pub fn sidebar_frame(corner_radius: CornerRadius) -> Frame {
    Frame::new()
        .fill(surface())
        .corner_radius(corner_radius)
        .inner_margin(Margin::symmetric(20, 14))
}

pub fn floating_frame() -> Frame {
    soft_frame()
        .corner_radius(CornerRadius::same(16))
        .shadow(Shadow {
            offset: [0, 10],
            blur: 28,
            spread: 0,
            color: floating_shadow(),
        })
}

pub fn card_frame() -> Frame {
    Frame::new()
        .fill(surface())
        .stroke(Stroke::new(1.0, border()))
        .corner_radius(CornerRadius::same(20))
        .inner_margin(Margin::same(18))
}

pub fn soft_frame() -> Frame {
    Frame::new()
        .fill(surface_soft())
        .stroke(Stroke::new(1.0, border()))
        .corner_radius(CornerRadius::same(12))
        .inner_margin(Margin::symmetric(12, 10))
}

pub fn muted(size: f32) -> egui::RichText {
    egui::RichText::new("").size(size).color(text_muted())
}

pub fn eyebrow(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .size(9.5)
        .color(text_faint())
        .strong()
}

pub fn section_label(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .size(SMALL_TEXT_SIZE)
        .color(text_faint())
        .strong()
}

pub fn section_title(title: &str) -> egui::RichText {
    egui::RichText::new(title).size(17.0).color(self::text())
}

pub fn bullet_separator(ui: &mut egui::Ui) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(1.0, 16.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, border());
}

#[cfg(test)]
mod contrast_tests {
    use super::*;

    fn luminance(color: Color32) -> f32 {
        let [r, g, b, _] = color.to_normalized_gamma_f32();
        let linear = |channel: f32| {
            if channel <= 0.039_28 {
                channel / 12.92
            } else {
                ((channel + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * linear(r) + 0.7152 * linear(g) + 0.0722 * linear(b)
    }

    fn contrast(a: Color32, b: Color32) -> f32 {
        let (a, b) = (luminance(a), luminance(b));
        let (light, dark) = if a > b { (a, b) } else { (b, a) };
        (light + 0.05) / (dark + 0.05)
    }

    #[test]
    fn informative_text_and_focus_meet_wcag_in_every_profile() {
        for profile in PROFILES {
            let palette = &profile.palette;
            for background in [
                palette.bg,
                palette.bg_soft,
                palette.surface,
                palette.surface_soft,
                palette.surface_2,
                palette.surface_3,
            ] {
                for foreground in [palette.text, palette.text_muted, palette.text_faint] {
                    assert!(
                        contrast(foreground, background) >= 4.5,
                        "contraste insuficiente en {}: {foreground:?} sobre {background:?}",
                        profile.label
                    );
                }
                assert!(
                    contrast(palette.text_disabled, background) >= 3.0,
                    "contraste de texto deshabilitado insuficiente en {}: {:?} sobre {background:?}",
                    profile.label,
                    palette.text_disabled
                );
                assert!(
                    contrast(palette.accent, background) >= 4.5,
                    "contraste de acento insuficiente en {}: {:?} sobre {background:?}",
                    profile.label,
                    palette.accent
                );
            }
            assert!(
                contrast(palette.accent_ink, palette.accent) >= 4.5,
                "contraste de acento insuficiente en {}",
                profile.label
            );
            assert!(
                contrast(palette.accent, palette.surface_3) >= 3.0,
                "contraste de foco insuficiente en {}",
                profile.label
            );
            assert!(
                contrast(palette.danger, palette.toast_error_bg) >= 4.5,
                "contraste del toast de error insuficiente en {}",
                profile.label
            );
        }
    }

    #[test]
    fn profiles_are_looked_up_case_insensitively_and_toggle() {
        assert_eq!(profile(" DARK ").map(|profile| profile.id), Some("dark"));
        assert_eq!(profile("Light").map(|profile| profile.id), Some("light"));
        assert!(profile("sepia").is_none());
        assert_eq!(toggled_profile_id("dark"), "light");
        assert_eq!(toggled_profile_id("light"), "dark");
    }
}
