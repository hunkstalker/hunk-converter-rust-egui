use image::{Rgb, RgbImage, RgbaImage};

pub fn color_codes(rgba: [u8; 4]) -> [(&'static str, String); 6] {
    let [r, g, b, a] = rgba;
    let alpha = a as f32 / 255.0;
    let (h, s, l) = rgb_to_hsl(r, g, b);
    let (ok_l, ok_a, ok_b) = rgb_to_oklab(r, g, b);
    let (ok_chroma, ok_hue) = oklab_to_oklch(ok_a, ok_b);

    [
        (
            "OKLCH",
            format!("oklch({:.1}% {ok_chroma:.3} {ok_hue:.1})", ok_l * 100.0),
        ),
        (
            "OKLAB",
            format!("oklab({:.1}% {ok_a:.3} {ok_b:.3})", ok_l * 100.0),
        ),
        ("HEX", format!("#{r:02X}{g:02X}{b:02X}")),
        ("RGB", format!("rgb({r}, {g}, {b})")),
        ("RGBA", format!("rgba({r}, {g}, {b}, {alpha:.2})")),
        ("HSL", format!("hsl({h:.0}, {s:.0}%, {l:.0}%)")),
    ]
}

pub fn rgb_to_oklab(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = srgb_to_linear(r as f32 / 255.0);
    let g = srgb_to_linear(g as f32 / 255.0);
    let b = srgb_to_linear(b as f32 / 255.0);

    let l = 0.412_221_47 * r + 0.536_332_55 * g + 0.051_445_995 * b;
    let m = 0.211_903_5 * r + 0.680_699_5 * g + 0.107_396_96 * b;
    let s = 0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_7 * b;

    let l = l.cbrt();
    let m = m.cbrt();
    let s = s.cbrt();

    (
        0.210_454_26 * l + 0.793_617_8 * m - 0.004_072_047 * s,
        1.977_998_5 * l - 2.428_592_2 * m + 0.450_593_7 * s,
        0.025_904_037 * l + 0.782_771_77 * m - 0.808_675_77 * s,
    )
}

pub fn oklab_to_oklch(a: f32, b: f32) -> (f32, f32) {
    let chroma = (a * a + b * b).sqrt();
    let hue = b.atan2(a).to_degrees();
    (chroma, if hue < 0.0 { hue + 360.0 } else { hue })
}

fn srgb_to_linear(value: f32) -> f32 {
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;
    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let delta = max - min;
    let lightness = (max + min) / 2.0;

    if delta <= f32::EPSILON {
        return (0.0, 0.0, lightness * 100.0);
    }

    let saturation = delta / (1.0 - (2.0 * lightness - 1.0).abs());
    let hue = if max == rf {
        60.0 * (((gf - bf) / delta) % 6.0)
    } else if max == gf {
        60.0 * (((bf - rf) / delta) + 2.0)
    } else {
        60.0 * (((rf - gf) / delta) + 4.0)
    };
    let hue = if hue < 0.0 { hue + 360.0 } else { hue };
    (hue, saturation * 100.0, lightness * 100.0)
}

pub fn flatten_rgba(source: &RgbaImage, background: [u8; 4]) -> RgbImage {
    let bg = [
        background[0] as f32,
        background[1] as f32,
        background[2] as f32,
    ];
    let mut output = RgbImage::new(source.width(), source.height());
    for (x, y, pixel) in source.enumerate_pixels() {
        let alpha = pixel[3] as f32 / 255.0;
        let blended = [
            (pixel[0] as f32 * alpha + bg[0] * (1.0 - alpha)).round() as u8,
            (pixel[1] as f32 * alpha + bg[1] * (1.0 - alpha)).round() as u8,
            (pixel[2] as f32 * alpha + bg[2] * (1.0 - alpha)).round() as u8,
        ];
        output.put_pixel(x, y, Rgb(blended));
    }
    output
}

pub fn parse_hex_rgba(value: &str) -> Option<[u8; 4]> {
    let value = value.trim().trim_start_matches('#');
    if !value.chars().all(|character| character.is_ascii_hexdigit()) {
        return None;
    }
    match value.len() {
        6 => Some([
            u8::from_str_radix(&value[0..2], 16).ok()?,
            u8::from_str_radix(&value[2..4], 16).ok()?,
            u8::from_str_radix(&value[4..6], 16).ok()?,
            255,
        ]),
        8 => Some([
            u8::from_str_radix(&value[0..2], 16).ok()?,
            u8::from_str_radix(&value[2..4], 16).ok()?,
            u8::from_str_radix(&value[4..6], 16).ok()?,
            u8::from_str_radix(&value[6..8], 16).ok()?,
        ]),
        _ => None,
    }
}

pub fn hex_rgba(rgba: [u8; 4]) -> String {
    format!(
        "#{:02X}{:02X}{:02X}{:02X}",
        rgba[0], rgba[1], rgba[2], rgba[3]
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};

    #[test]
    fn red_color_codes() {
        let codes = color_codes([255, 0, 0, 255]);
        assert_eq!(codes[0].0, "OKLCH");
        assert_eq!(codes[0].1, "oklch(62.8% 0.258 29.2)");
        assert_eq!(codes[1].0, "OKLAB");
        assert_eq!(codes[1].1, "oklab(62.8% 0.225 0.126)");
        assert_eq!(codes[2].1, "#FF0000");
        assert_eq!(codes[3].1, "rgb(255, 0, 0)");
        assert_eq!(codes[4].1, "rgba(255, 0, 0, 1.00)");
        assert_eq!(codes[5].1, "hsl(0, 100%, 50%)");
    }

    #[test]
    fn converts_to_oklab_reference_values() {
        let (l, a, b) = rgb_to_oklab(255, 0, 0);
        assert!((l - 0.627_955).abs() < 0.001);
        assert!((a - 0.224_863).abs() < 0.001);
        assert!((b - 0.125_846).abs() < 0.001);
        let (chroma, hue) = oklab_to_oklch(a, b);
        assert!((chroma - 0.257_683).abs() < 0.001);
        assert!((hue - 29.234).abs() < 0.1);
    }

    #[test]
    fn flattens_transparent_pixels_on_background() {
        let mut image = RgbaImage::new(2, 1);
        image.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
        image.put_pixel(1, 0, Rgba([0, 0, 0, 0]));
        let flattened = flatten_rgba(&image, [255, 255, 255, 255]);
        assert_eq!(flattened.get_pixel(0, 0).0, [255, 0, 0]);
        assert_eq!(flattened.get_pixel(1, 0).0, [255, 255, 255]);
    }

    #[test]
    fn parses_hex_with_and_without_alpha() {
        assert_eq!(parse_hex_rgba("#FF0000"), Some([255, 0, 0, 255]));
        assert_eq!(parse_hex_rgba("00FF0080"), Some([0, 255, 0, 128]));
        assert_eq!(parse_hex_rgba("#12345"), None);
        assert_eq!(hex_rgba([124, 126, 120, 255]), "#7C7E78FF");
    }
}
