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
