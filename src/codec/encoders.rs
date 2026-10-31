use std::io::Cursor;

use image::codecs::avif::AvifEncoder;
use image::{ExtendedColorType, ImageEncoder, ImageFormat, RgbaImage};
use mozjpeg_rs::{Encoder as JpegEncoder, Preset as JpegPreset, Subsampling};
use zune_core::bit_depth::BitDepth as ZuneBitDepth;
use zune_core::colorspace::ColorSpace;
use zune_core::options::EncoderOptions as ZuneEncoderOptions;
use zune_jpegxl::JxlSimpleEncoder;

use crate::domain::color::flatten_rgba;
use crate::domain::error::EncodeError;
use crate::domain::format::OutputFormat;

use super::EncodedOutput;

pub(super) fn encode_webp(source: &RgbaImage, quality: u8) -> Result<EncodedOutput, EncodeError> {
    let memory = webp::Encoder::from_rgba(source.as_raw(), source.width(), source.height())
        .encode(quality as f32);
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

pub(super) fn encode_avif(source: &RgbaImage, quality: u8) -> Result<EncodedOutput, EncodeError> {
    let mut bytes = Vec::new();
    AvifEncoder::new_with_speed_quality(Cursor::new(&mut bytes), 4, quality)
        .write_image(
            source.as_raw(),
            source.width(),
            source.height(),
            ExtendedColorType::Rgba8,
        )
        .map_err(|source| EncodeError::Avif {
            source: Box::new(source),
        })?;
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
