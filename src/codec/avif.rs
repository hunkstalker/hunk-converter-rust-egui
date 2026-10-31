use std::ffi::{CStr, CString};
use std::fmt;
use std::io::Cursor;

use image::RgbaImage;
use libavif_sys as sys;

#[derive(Debug)]
pub(super) struct AvifEncodeError {
    message: String,
}

impl AvifEncodeError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for AvifEncodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for AvifEncodeError {}

pub(super) fn encode_rgba(
    source: &RgbaImage,
    quality: u8,
    speed: u8,
    alpha_quality: u8,
) -> Result<Vec<u8>, AvifEncodeError> {
    let quality = quality.clamp(1, 100);
    let alpha_quality = if alpha_quality == 0 {
        quality
    } else {
        alpha_quality.clamp(1, 100)
    };
    let color_quantizer = quantizer_for_quality(quality);
    let alpha_quantizer = quantizer_for_quality(alpha_quality);

    unsafe {
        let image = ImageGuard::new(source)?;
        let encoder = EncoderGuard::new()?;
        encoder.set_threads();
        encoder.set_speed(speed);
        encoder.set_quality(quality, alpha_quality);
        encoder.set_option("end-usage", "q")?;
        encoder.set_option("cq-level", &color_quantizer.to_string())?;
        encoder.set_option("sharpness", "0")?;
        encoder.set_option("color:denoise-noise-level", "0")?;
        encoder.set_option("alpha:cq-level", &alpha_quantizer.to_string())?;
        if color_quantizer <= 32 {
            encoder.set_option("tune", "ssim")?;
        }

        let rgb = RgbGuard::from_rgba(source, image.0);
        let status = sys::avifImageRGBToYUV(image.0, &rgb.0);
        if status != sys::AVIF_RESULT_OK {
            return Err(AvifEncodeError::new(format!(
                "no se pudo convertir RGBA a YUV: {}",
                result_message(status)
            )));
        }

        let mut output = RwDataGuard::default();
        let status = sys::avifEncoderWrite(encoder.0, image.0, &mut output.0);
        if status != sys::AVIF_RESULT_OK {
            return Err(AvifEncodeError::new(format!(
                "AOM no pudo codificar el AVIF: {}",
                result_message(status)
            )));
        }
        if output.0.data.is_null() || output.0.size == 0 {
            return Err(AvifEncodeError::new("AOM devolvió un AVIF vacío"));
        }
        Ok(std::slice::from_raw_parts(output.0.data, output.0.size).to_vec())
    }
}

fn quantizer_for_quality(quality: u8) -> u8 {
    let quality = u32::from(quality.clamp(1, 100));
    (((100 - quality) * 63 + 50) / 100) as u8
}

unsafe fn result_message(status: sys::avifResult) -> String {
    let message = unsafe { sys::avifResultToString(status) };
    if message.is_null() {
        format!("código {status}")
    } else {
        unsafe { CStr::from_ptr(message) }
            .to_string_lossy()
            .into_owned()
    }
}

struct ImageGuard(*mut sys::avifImage);

impl ImageGuard {
    unsafe fn new(source: &RgbaImage) -> Result<Self, AvifEncodeError> {
        let image = unsafe {
            sys::avifImageCreate(
                source.width(),
                source.height(),
                8,
                sys::AVIF_PIXEL_FORMAT_YUV420,
            )
        };
        if image.is_null() {
            return Err(AvifEncodeError::new(
                "libavif no pudo crear la imagen de trabajo",
            ));
        }
        unsafe {
            (*image).matrixCoefficients = sys::AVIF_MATRIX_COEFFICIENTS_BT601 as u16;
        }
        Ok(Self(image))
    }
}

impl Drop for ImageGuard {
    fn drop(&mut self) {
        unsafe { sys::avifImageDestroy(self.0) };
    }
}

struct EncoderGuard(*mut sys::avifEncoder);

impl EncoderGuard {
    unsafe fn new() -> Result<Self, AvifEncodeError> {
        let encoder = unsafe { sys::avifEncoderCreate() };
        if encoder.is_null() {
            return Err(AvifEncodeError::new("libavif no pudo crear el codificador"));
        }
        unsafe {
            (*encoder).codecChoice = sys::AVIF_CODEC_CHOICE_AOM;
        }
        Ok(Self(encoder))
    }

    unsafe fn set_threads(&self) {
        let threads = std::thread::available_parallelism()
            .map_or(4, |value| value.get())
            .min(32) as i32;
        unsafe { (*self.0).maxThreads = threads };
    }

    unsafe fn set_speed(&self, speed: u8) {
        unsafe { (*self.0).speed = i32::from(speed.clamp(1, 10)) };
    }

    unsafe fn set_quality(&self, quality: u8, alpha_quality: u8) {
        unsafe {
            (*self.0).quality = i32::from(quality);
            (*self.0).qualityAlpha = i32::from(alpha_quality);
        }
    }

    unsafe fn set_option(&self, key: &str, value: &str) -> Result<(), AvifEncodeError> {
        let key = CString::new(key).map_err(|_| AvifEncodeError::new("opción AVIF no válida"))?;
        let value =
            CString::new(value).map_err(|_| AvifEncodeError::new("valor AVIF no válido"))?;
        let status =
            unsafe { sys::avifEncoderSetCodecSpecificOption(self.0, key.as_ptr(), value.as_ptr()) };
        if status == sys::AVIF_RESULT_OK {
            Ok(())
        } else {
            Err(AvifEncodeError::new(format!(
                "opción AVIF rechazada: {}",
                unsafe { result_message(status) }
            )))
        }
    }
}

impl Drop for EncoderGuard {
    fn drop(&mut self) {
        unsafe { sys::avifEncoderDestroy(self.0) };
    }
}

struct RgbGuard(sys::avifRGBImage);

impl RgbGuard {
    unsafe fn from_rgba(source: &RgbaImage, image: *const sys::avifImage) -> Self {
        let mut rgb = sys::avifRGBImage::default();
        unsafe { sys::avifRGBImageSetDefaults(&mut rgb, image) };
        rgb.pixels = source.as_raw().as_ptr() as *mut u8;
        rgb.rowBytes = source.width() * 4;
        Self(rgb)
    }
}

#[derive(Default)]
struct RwDataGuard(sys::avifRWData);

impl Drop for RwDataGuard {
    fn drop(&mut self) {
        unsafe { sys::avifRWDataFree(&mut self.0) };
    }
}

pub(super) fn dimensions(bytes: &[u8]) -> Result<(u32, u32), avif_parse::Error> {
    let data = avif_parse::AvifData::from_reader(&mut Cursor::new(bytes))?;
    let metadata = data.primary_item_metadata()?;
    Ok((
        metadata.max_frame_width.get(),
        metadata.max_frame_height.get(),
    ))
}

pub(super) fn decode_rgba(bytes: &[u8]) -> Result<RgbaImage, avif_decode::Error> {
    let image = avif_decode::Decoder::from_avif(bytes).and_then(|decoder| decoder.to_image())?;

    let (width, height, pixels) = match image {
        avif_decode::Image::Rgba8(image) => {
            let pixels = image
                .buf()
                .iter()
                .flat_map(|pixel| [pixel.r, pixel.g, pixel.b, pixel.a])
                .collect();
            (image.width() as u32, image.height() as u32, pixels)
        }
        avif_decode::Image::Rgb8(image) => {
            let pixels = image
                .buf()
                .iter()
                .flat_map(|pixel| [pixel.r, pixel.g, pixel.b, 255])
                .collect();
            (image.width() as u32, image.height() as u32, pixels)
        }
        avif_decode::Image::Gray8(image) => {
            let pixels = image
                .buf()
                .iter()
                .flat_map(|pixel| {
                    let value = pixel.value();
                    [value, value, value, 255]
                })
                .collect();
            (image.width() as u32, image.height() as u32, pixels)
        }
        avif_decode::Image::Rgba16(image) => {
            let pixels = image
                .buf()
                .iter()
                .flat_map(|pixel| {
                    [
                        u16_to_u8(pixel.r),
                        u16_to_u8(pixel.g),
                        u16_to_u8(pixel.b),
                        u16_to_u8(pixel.a),
                    ]
                })
                .collect();
            (image.width() as u32, image.height() as u32, pixels)
        }
        avif_decode::Image::Rgb16(image) => {
            let pixels = image
                .buf()
                .iter()
                .flat_map(|pixel| {
                    [
                        u16_to_u8(pixel.r),
                        u16_to_u8(pixel.g),
                        u16_to_u8(pixel.b),
                        255,
                    ]
                })
                .collect();
            (image.width() as u32, image.height() as u32, pixels)
        }
        avif_decode::Image::Gray16(image) => {
            let pixels = image
                .buf()
                .iter()
                .flat_map(|pixel| {
                    let value = u16_to_u8(pixel.value());
                    [value, value, value, 255]
                })
                .collect();
            (image.width() as u32, image.height() as u32, pixels)
        }
    };

    RgbaImage::from_raw(width, height, pixels).ok_or(avif_decode::Error::InternalError(
        "AVIF no produjo una imagen RGBA válida",
    ))
}

fn u16_to_u8(value: u16) -> u8 {
    ((u32::from(value) * 255 + 32_767) / 65_535) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgba;

    #[test]
    fn quality_maps_to_the_squoosh_quantizer_curve() {
        assert_eq!(quantizer_for_quality(100), 0);
        assert_eq!(quantizer_for_quality(52), 30);
        assert_eq!(quantizer_for_quality(50), 32);
        assert_eq!(quantizer_for_quality(1), 62);
    }

    #[test]
    fn aom_roundtrip_preserves_dimensions_and_alpha() {
        let source = RgbaImage::from_fn(24, 16, |x, y| {
            Rgba([
                (x * 10) as u8,
                (y * 15) as u8,
                ((x + y) * 5) as u8,
                if x < 8 { 0 } else { 255 },
            ])
        });
        let bytes = encode_rgba(&source, 80, 4, 0).unwrap();
        let decoded = decode_rgba(&bytes).unwrap();
        assert_eq!(decoded.dimensions(), source.dimensions());
        assert_eq!(decoded.get_pixel(0, 0)[3], 0);
        assert!(decoded.get_pixel(23, 15)[3] > 240);
    }
}
