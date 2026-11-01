use image::{RgbaImage, imageops};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CropRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CropPixels {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl CropRect {
    pub const FULL: Self = Self {
        x: 0.0,
        y: 0.0,
        width: 1.0,
        height: 1.0,
    };

    pub fn clamped(self) -> Self {
        let x = self.x.clamp(0.0, 1.0);
        let y = self.y.clamp(0.0, 1.0);
        let width = self.width.clamp(0.001, (1.0 - x).max(0.001));
        let height = self.height.clamp(0.001, (1.0 - y).max(0.001));
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn is_full(self) -> bool {
        let rect = self.clamped();
        rect.x <= 0.0001 && rect.y <= 0.0001 && rect.width >= 0.9999 && rect.height >= 0.9999
    }

    pub fn pixel_rect(self, source_width: u32, source_height: u32) -> CropPixels {
        let rect = self.clamped();
        if source_width == 0 || source_height == 0 {
            return CropPixels {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
        }

        // Ajusta los bordes que quedan a una milésima de un límite de píxel:
        // los presets de proporción pueden dar valores como 49,99998 por el
        // redondeo de f32.
        const EDGE_EPSILON: f32 = 0.01;
        let x =
            ((rect.x * source_width as f32 + EDGE_EPSILON).floor() as u32).min(source_width - 1);
        let y =
            ((rect.y * source_height as f32 + EDGE_EPSILON).floor() as u32).min(source_height - 1);
        let right = (((rect.x + rect.width) * source_width as f32 - EDGE_EPSILON).ceil() as u32)
            .clamp(x + 1, source_width);
        let bottom = (((rect.y + rect.height) * source_height as f32 - EDGE_EPSILON).ceil() as u32)
            .clamp(y + 1, source_height);

        CropPixels {
            x,
            y,
            width: right - x,
            height: bottom - y,
        }
    }

    pub fn output_dimensions(self, source_width: u32, source_height: u32) -> (u32, u32) {
        let pixels = self.pixel_rect(source_width, source_height);
        (pixels.width, pixels.height)
    }

    pub fn from_aspect(target_aspect: f32, source_aspect: f32) -> Self {
        let target_aspect = if target_aspect.is_finite() && target_aspect > 0.0 {
            target_aspect
        } else {
            1.0
        };
        let source_aspect = if source_aspect.is_finite() && source_aspect > 0.0 {
            source_aspect
        } else {
            1.0
        };
        let aspect = (target_aspect / source_aspect).clamp(0.01, 100.0);
        let (width, height) = if aspect >= 1.0 {
            (1.0, 1.0 / aspect)
        } else {
            (aspect, 1.0)
        };
        Self {
            x: (1.0 - width) * 0.5,
            y: (1.0 - height) * 0.5,
            width,
            height,
        }
    }
}

pub fn crop_rgba(source: &RgbaImage, crop: CropRect) -> RgbaImage {
    let pixels = crop.pixel_rect(source.width(), source.height());
    if pixels.width == 0 || pixels.height == 0 {
        return source.clone();
    }
    imageops::crop_imm(source, pixels.x, pixels.y, pixels.width, pixels.height).to_image()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_rect_is_full() {
        assert!(CropRect::FULL.is_full());
        assert!(!CropRect::from_aspect(1.0, 16.0 / 9.0).is_full());
    }

    #[test]
    fn aspect_rect_is_centered_and_clamped() {
        let rect = CropRect::from_aspect(16.0 / 9.0, 16.0 / 9.0);
        assert!(rect.width <= 1.0);
        assert!(rect.height <= 1.0);
        assert!((rect.x * 2.0 + rect.width - 1.0).abs() < 0.0001);
        assert!((rect.y * 2.0 + rect.height - 1.0).abs() < 0.0001);
    }

    #[test]
    fn clamped_edge_still_has_a_pixel() {
        let rect = CropRect {
            x: 1.0,
            y: 1.0,
            width: 0.0,
            height: 0.0,
        }
        .clamped();
        assert!(rect.width >= 0.001);
        assert!(rect.height >= 0.001);
        assert_eq!(
            rect.pixel_rect(100, 100),
            CropPixels {
                x: 99,
                y: 99,
                width: 1,
                height: 1,
            }
        );
    }

    #[test]
    fn preset_dimensions_match_target_aspect() {
        let source = (1600, 1000);
        for (target, expected) in [
            (16.0 / 9.0, (1600, 900)),
            (4.0 / 3.0, (1334, 1000)),
            (1.0, (1000, 1000)),
            (1600.0 / 1000.0, (1600, 1000)),
        ] {
            let source_aspect = 1600.0 / 1000.0;
            let dimensions =
                CropRect::from_aspect(target, source_aspect).output_dimensions(source.0, source.1);
            assert_eq!(dimensions, expected);
        }
    }

    #[test]
    fn pixel_rect_clamps_to_source() {
        let pixels = CropRect {
            x: 0.25,
            y: 0.2,
            width: 0.5,
            height: 0.4,
        }
        .pixel_rect(1600, 1000);
        assert_eq!(
            pixels,
            CropPixels {
                x: 400,
                y: 200,
                width: 800,
                height: 400
            }
        );
        assert_eq!(CropRect::FULL.output_dimensions(1600, 1000), (1600, 1000));
    }
}
