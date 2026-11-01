use image::{Rgba, RgbaImage, imageops};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreprocessOptions {
    pub resize_enabled: bool,
    pub width: u32,
    pub height: u32,
    pub keep_aspect: bool,
    pub rotation: u16,
    pub palette_colors: Option<u16>,
}

impl Default for PreprocessOptions {
    fn default() -> Self {
        Self {
            resize_enabled: false,
            width: 0,
            height: 0,
            keep_aspect: true,
            rotation: 0,
            palette_colors: None,
        }
    }
}

impl PreprocessOptions {
    pub fn reset_for_source(&mut self, width: u32, height: u32) {
        self.rotation = 0;
        self.palette_colors = None;
        if !self.resize_enabled || self.width == 0 || self.height == 0 {
            self.width = width;
            self.height = height;
        }
    }
}

pub fn rotated_size(width: u32, height: u32, rotation: u16) -> (u32, u32) {
    match rotation % 360 {
        90 | 270 => (height, width),
        _ => (width, height),
    }
}

pub fn linked_height(width: u32, aspect: f32, max_side: u32) -> u32 {
    if !aspect.is_finite() || aspect <= 0.0 {
        return width.clamp(1, max_side);
    }
    ((width as f32 / aspect).round() as u32).clamp(1, max_side)
}

pub fn linked_width(height: u32, aspect: f32, max_side: u32) -> u32 {
    if !aspect.is_finite() || aspect <= 0.0 {
        return height.clamp(1, max_side);
    }
    ((height as f32 * aspect).round() as u32).clamp(1, max_side)
}

pub fn parse_dimension_expression(input: &str) -> Option<u32> {
    let normalized: String = input
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect();
    if normalized.is_empty() {
        return None;
    }
    if let Ok(value) = normalized.parse::<u32>() {
        return Some(value);
    }

    let mut operator = None;
    for (index, character) in normalized.char_indices().skip(1) {
        if matches!(character, 'x' | 'X' | '*' | '/' | '+' | '-') {
            operator = Some((index, character));
            break;
        }
    }
    let (index, operator) = operator?;
    let left = normalized[..index].parse::<u64>().ok()?;
    let right = normalized[index + operator.len_utf8()..]
        .parse::<u64>()
        .ok()?;
    let result = match operator {
        'x' | 'X' | '*' => left.checked_mul(right)?,
        '/' => {
            if right == 0 {
                return None;
            }
            (left as f64 / right as f64).round() as u64
        }
        '+' => left.checked_add(right)?,
        '-' => left.checked_sub(right)?,
        _ => return None,
    };
    u32::try_from(result).ok()
}

pub fn apply_geometry(source: &RgbaImage, options: &PreprocessOptions) -> RgbaImage {
    let rotated = match options.rotation % 360 {
        90 => imageops::rotate90(source),
        180 => imageops::rotate180(source),
        270 => imageops::rotate270(source),
        _ => source.clone(),
    };

    if !options.resize_enabled {
        return rotated;
    }

    let width = if options.width == 0 {
        rotated.width()
    } else {
        options.width.min(16_384)
    };
    let height = if options.height == 0 {
        rotated.height()
    } else {
        options.height.min(16_384)
    };

    if width == rotated.width() && height == rotated.height() {
        return rotated;
    }

    let (target_width, target_height) = if options.keep_aspect {
        let scale = (width as f32 / rotated.width() as f32)
            .min(height as f32 / rotated.height() as f32)
            .max(0.0001);
        (
            (rotated.width() as f32 * scale).round().max(1.0) as u32,
            (rotated.height() as f32 * scale).round().max(1.0) as u32,
        )
    } else {
        (width, height)
    };

    imageops::resize(
        &rotated,
        target_width,
        target_height,
        imageops::FilterType::Lanczos3,
    )
}

pub fn apply_palette(source: &RgbaImage, colors: u16) -> RgbaImage {
    let colors = colors.clamp(2, 256);
    let rgb = image::DynamicImage::ImageRgba8(source.clone()).to_rgb8();
    let Ok(input) = quantette::ImageBuf::try_from(rgb) else {
        return source.clone();
    };
    let quantized = quantette::Pipeline::new()
        .palette_size(quantette::PaletteSize::from_u16_clamped(colors))
        .ditherer(None)
        .parallel(true)
        .input_image(input.as_ref())
        .output_srgb8_image();
    let quantized: image::RgbImage = quantized.into();

    let mut output = RgbaImage::new(source.width(), source.height());
    for (x, y, pixel) in quantized.enumerate_pixels() {
        let alpha = source.get_pixel(x, y)[3];
        output.put_pixel(x, y, Rgba([pixel[0], pixel[1], pixel[2], alpha]));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn source() -> RgbaImage {
        RgbaImage::from_fn(16, 8, |x, y| {
            Rgba([
                (x * 16) as u8,
                (y * 32) as u8,
                ((x + y) * 8) as u8,
                if (x + y) % 5 == 0 { 128 } else { 255 },
            ])
        })
    }

    #[test]
    fn rotates_and_preserves_alpha() {
        let source = source();
        let output = apply_geometry(
            &source,
            &PreprocessOptions {
                rotation: 90,
                ..PreprocessOptions::default()
            },
        );
        assert_eq!(output.dimensions(), (8, 16));
        assert_eq!(output.get_pixel(0, 0).0[3], source.get_pixel(0, 7).0[3]);
    }

    #[test]
    fn resizes_with_aspect_ratio() {
        let source = source();
        let output = apply_geometry(
            &source,
            &PreprocessOptions {
                resize_enabled: true,
                width: 8,
                height: 8,
                keep_aspect: true,
                ..PreprocessOptions::default()
            },
        );
        assert_eq!(output.dimensions(), (8, 4));
    }

    #[test]
    fn parses_blender_style_dimension_expressions() {
        assert_eq!(parse_dimension_expression("1600x2"), Some(3200));
        assert_eq!(parse_dimension_expression("1600 X 2"), Some(3200));
        assert_eq!(parse_dimension_expression("1600/4"), Some(400));
        assert_eq!(parse_dimension_expression("1600*2"), Some(3200));
        assert_eq!(parse_dimension_expression("1600-200"), Some(1400));
        assert_eq!(parse_dimension_expression("1600+200"), Some(1800));
        assert_eq!(parse_dimension_expression("4000/3"), Some(1333));
        assert_eq!(parse_dimension_expression("1600/0"), None);
        assert_eq!(parse_dimension_expression("1600-2000"), None);
        assert_eq!(parse_dimension_expression("1600x2x3"), None);
        assert_eq!(parse_dimension_expression("alto"), None);
    }

    #[test]
    fn links_dimensions_with_the_rotated_aspect_ratio() {
        assert_eq!(rotated_size(1600, 1000, 90), (1000, 1600));
        assert_eq!(rotated_size(1600, 1000, 180), (1600, 1000));
        assert_eq!(linked_height(800, 1.6, 16_384), 500);
        assert_eq!(linked_width(500, 1.6, 16_384), 800);
        assert_eq!(linked_height(16_384, 1.6, 16_384), 10_240);
    }

    #[test]
    fn palette_reduces_colors_and_preserves_alpha() {
        let source = source();
        let output = apply_palette(&source, 8);
        assert_eq!(output.dimensions(), source.dimensions());
        let colors: HashSet<[u8; 3]> = output
            .pixels()
            .map(|pixel| [pixel[0], pixel[1], pixel[2]])
            .collect();
        assert!(colors.len() <= 8);
        assert!(
            output
                .pixels()
                .zip(source.pixels())
                .all(|(a, b)| a[3] == b[3])
        );
    }
}
