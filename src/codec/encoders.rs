use std::io::Cursor;

use image::{ImageFormat, RgbaImage};
use mozjpeg_rs::{Encoder as JpegEncoder, Preset as JpegPreset, Subsampling};
use rgb::FromSlice;
use zune_core::bit_depth::BitDepth as ZuneBitDepth;
use zune_core::colorspace::ColorSpace;
use zune_core::options::EncoderOptions as ZuneEncoderOptions;
use zune_jpegxl::JxlSimpleEncoder;

use crate::domain::color::flatten_rgba;
use crate::domain::error::EncodeError;
use crate::domain::format::OutputFormat;

use super::EncodedOutput;

pub(super) fn encode_webp(
    source: &RgbaImage,
    quality: u8,
    lossless: bool,
    method: u8,
    alpha_quality: u8,
) -> Result<EncodedOutput, EncodeError> {
    let mut config = webp::WebPConfig::new().map_err(|_| EncodeError::Webp {
        message: "no se pudo inicializar la configuración de WebP".to_owned(),
    })?;
    config.lossless = i32::from(lossless);
    config.quality = quality as f32;
    config.method = i32::from(method.min(6));
    config.alpha_quality = i32::from(alpha_quality.min(100));
    config.alpha_compression = if lossless { 0 } else { 1 };
    let memory = webp::Encoder::from_rgba(source.as_raw(), source.width(), source.height())
        .encode_advanced(&config)
        .map_err(|error| EncodeError::Webp {
            message: format!("{error:?}"),
        })?;
    let bytes = memory.to_vec();
    let preview = decode_preview(ImageFormat::WebP, &bytes)?;
    Ok(output(source, bytes, preview))
}

pub(super) fn encode_jpg(
    source: &RgbaImage,
    quality: u8,
    background: [u8; 4],
) -> Result<EncodedOutput, EncodeError> {
    let flattened = flatten_rgba(source, background);
    let subsampling = match quality {
        0..=79 => Subsampling::S420,
        80..=89 => Subsampling::S422,
        _ => Subsampling::S444,
    };
    let bytes = JpegEncoder::new(JpegPreset::ProgressiveBalanced)
        .quality(quality)
        .subsampling(subsampling)
        .encode_rgb(flattened.as_raw(), source.width(), source.height())
        .map_err(|source| EncodeError::Jpg {
            source: Box::new(source),
        })?;
    let preview = decode_preview(ImageFormat::Jpeg, &bytes)?;
    Ok(output(source, bytes, preview))
}

pub(super) fn encode_png(source: &RgbaImage) -> Result<EncodedOutput, EncodeError> {
    let raw = oxipng::RawImage::new(
        source.width(),
        source.height(),
        oxipng::ColorType::RGBA,
        oxipng::BitDepth::Eight,
        source.as_raw().clone(),
    )
    .map_err(|source| EncodeError::PngStart {
        source: Box::new(source),
    })?;
    let bytes = raw
        .create_optimized_png(&oxipng::Options::from_preset(2))
        .map_err(|source| EncodeError::Png {
            source: Box::new(source),
        })?;
    let preview = decode_preview(ImageFormat::Png, &bytes)?;
    Ok(output(source, bytes, preview))
}

pub(super) fn encode_avif(
    source: &RgbaImage,
    quality: u8,
    speed: u8,
    alpha_quality: u8,
) -> Result<EncodedOutput, EncodeError> {
    let quality = quality.clamp(1, 100) as f32;
    let alpha_quality = if alpha_quality == 0 {
        quality
    } else {
        f32::from(alpha_quality.clamp(1, 100))
    };
    let encoded = ravif::Encoder::new()
        .with_quality(quality)
        .with_alpha_quality(alpha_quality)
        .with_speed(speed.clamp(1, 10))
        .with_bit_depth(ravif::BitDepth::Eight)
        .encode_rgba(ravif::Img::new(
            source.as_raw().as_rgba(),
            source.width() as usize,
            source.height() as usize,
        ))
        .map_err(|source| EncodeError::Avif {
            source: Box::new(source),
        })?;
    let bytes = encoded.avif_file;
    let preview = decode_avif_preview(&bytes)?;
    Ok(output(source, bytes, preview))
}

pub(super) fn encode_jxl(source: &RgbaImage) -> Result<EncodedOutput, EncodeError> {
    let options = ZuneEncoderOptions::new(
        source.width() as usize,
        source.height() as usize,
        ColorSpace::RGBA,
        ZuneBitDepth::Eight,
    );
    let mut bytes = Vec::new();
    JxlSimpleEncoder::new(source.as_raw(), options)
        .encode(&mut bytes)
        .map_err(|source| EncodeError::Jxl {
            source: Box::new(source),
        })?;
    let preview = decode_jxl_preview(&bytes)?;
    Ok(output(source, bytes, preview))
}

fn output(source: &RgbaImage, bytes: Vec<u8>, preview: RgbaImage) -> EncodedOutput {
    EncodedOutput {
        bytes,
        preview,
        width: source.width(),
        height: source.height(),
        frame_count: 1,
        ico_sizes: Vec::new(),
    }
}

fn decode_preview(format: ImageFormat, bytes: &[u8]) -> Result<RgbaImage, EncodeError> {
    image::load_from_memory_with_format(bytes, format)
        .map(|image| image.to_rgba8())
        .map_err(|source| EncodeError::Preview {
            format: match format {
                ImageFormat::WebP => OutputFormat::Webp,
                ImageFormat::Jpeg => OutputFormat::Jpg,
                ImageFormat::Png => OutputFormat::Png,
                _ => OutputFormat::Png,
            },
            source: Box::new(source),
        })
}

fn decode_avif_preview(bytes: &[u8]) -> Result<RgbaImage, EncodeError> {
    super::avif::decode_rgba(bytes).map_err(|source| EncodeError::Preview {
        format: OutputFormat::Avif,
        source: Box::new(source),
    })
}

fn decode_jxl_preview(bytes: &[u8]) -> Result<RgbaImage, EncodeError> {
    let decoder =
        jxl_oxide::integration::JxlDecoder::new(Cursor::new(bytes)).map_err(|source| {
            EncodeError::Preview {
                format: OutputFormat::Jxl,
                source: Box::new(source),
            }
        })?;
    let image =
        image::DynamicImage::from_decoder(decoder).map_err(|source| EncodeError::Preview {
            format: OutputFormat::Jxl,
            source: Box::new(source),
        })?;
    Ok(image.to_rgba8())
}
