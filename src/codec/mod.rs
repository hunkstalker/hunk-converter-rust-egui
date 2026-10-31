use std::io::Cursor;
use std::path::Path;

use image::{ExtendedColorType, ImageDecoder, ImageFormat, RgbaImage, imageops};

use crate::domain::error::{EncodeError, LoadError};

mod avif;
mod encoders;
mod ico;
mod svg;

pub use ico::{IcoOutput, encode_ico};

pub use crate::domain::format::{OutputFormat, format_bytes};

pub const PREVIEW_MAX_SIDE: u32 = 2048;
pub const MAX_SOURCE_SIDE: u32 = 16_384;
pub const MAX_SOURCE_PIXELS: u32 = 40_000_000;
pub const ICO_SIZES: [u32; 6] = [16, 32, 48, 64, 128, 256];
const MAX_SOURCE_BYTES: u64 = 200 * 1024 * 1024;

#[derive(Clone, Debug)]
pub struct EncodeOptions {
    pub format: OutputFormat,
    pub quality: u8,
    pub ico_sizes: Vec<u32>,
    pub background: [u8; 4],
}

pub struct EncodedOutput {
    pub bytes: Vec<u8>,
    pub preview: RgbaImage,
    pub width: u32,
    pub height: u32,
    pub frame_count: usize,
    pub ico_sizes: Vec<u32>,
}

pub fn encode(source: &RgbaImage, options: &EncodeOptions) -> Result<EncodedOutput, EncodeError> {
    if source.width() == 0 || source.height() == 0 {
        return Err(EncodeError::InvalidDimensions);
    }
    let quality = options.quality.clamp(1, 100);

    let mut result = match options.format {
        OutputFormat::Webp => encoders::encode_webp(source, quality)?,
        OutputFormat::Jpg => encoders::encode_jpg(source, quality, options.background)?,
        OutputFormat::Png => encoders::encode_png(source)?,
        OutputFormat::Avif => encoders::encode_avif(source, quality)?,
        OutputFormat::Jxl => encoders::encode_jxl(source)?,
        OutputFormat::Ico => {
            let ico = ico::encode_ico(source, &options.ico_sizes)?;
            EncodedOutput {
                bytes: ico.bytes,
                preview: ico.preview,
                width: ico.width,
                height: ico.height,
                frame_count: ico.sizes.len(),
                ico_sizes: ico.sizes,
            }
        }
    };

    result.preview = bounded_preview(&result.preview);
    Ok(result)
}

pub struct LoadedImage {
    pub pixels: RgbaImage,
    pub format: String,
    pub file_bytes: u64,
}

pub fn load_image_file(path: &Path) -> Result<LoadedImage, LoadError> {
    let metadata = std::fs::metadata(path).map_err(|source| LoadError::Read {
        path: path.to_path_buf(),
        source,
    })?;
    if metadata.len() > MAX_SOURCE_BYTES {
        return Err(LoadError::TooLarge {
            bytes: metadata.len(),
            limit: MAX_SOURCE_BYTES,
        });
    }

    let bytes = std::fs::read(path).map_err(|source| LoadError::Read {
        path: path.to_path_buf(),
        source,
    })?;

    if path
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("svg"))
    {
        let pixels = svg::decode_svg(&bytes, path)?;
        return Ok(LoadedImage {
            pixels,
            format: "SVG".to_owned(),
            file_bytes: bytes.len() as u64,
        });
    }

    let guessed = image::guess_format(&bytes)
        .or_else(|_| ImageFormat::from_path(path))
        .map_err(|_| LoadError::Unsupported)?;
    let pixels = if guessed == ImageFormat::Avif {
        decode_avif(&bytes)?
    } else {
        decode_raster(&bytes, guessed)?
    };
    validate_dimensions(pixels.width(), pixels.height())?;

    Ok(LoadedImage {
        pixels,
        format: format_name(guessed).to_owned(),
        file_bytes: bytes.len() as u64,
    })
}

fn decode_avif(bytes: &[u8]) -> Result<RgbaImage, LoadError> {
    let (width, height) = avif::dimensions(bytes).map_err(|source| LoadError::Decode {
        source: Box::new(source),
    })?;
    validate_dimensions(width, height)?;
    avif::decode_rgba(bytes).map_err(|source| LoadError::Decode {
        source: Box::new(source),
    })
}

fn decode_raster(bytes: &[u8], format: ImageFormat) -> Result<RgbaImage, LoadError> {
    let mut reader = image::ImageReader::with_format(Cursor::new(bytes), format);
    reader.limits(image::Limits::default());

    let mut decoder = reader.into_decoder().map_err(|source| LoadError::Decode {
        source: Box::new(source),
    })?;
    let (width, height) = decoder.dimensions();
    validate_dimensions(width, height)?;
    if format == ImageFormat::Tiff {
        validate_tiff_color_type(decoder.original_color_type())?;
    }

    let orientation = decoder.orientation().map_err(|source| LoadError::Decode {
        source: Box::new(source),
    })?;
    let mut image =
        image::DynamicImage::from_decoder(decoder).map_err(|source| LoadError::Decode {
            source: Box::new(source),
        })?;
    image.apply_orientation(orientation);
    Ok(image.to_rgba8())
}

fn validate_tiff_color_type(color_type: ExtendedColorType) -> Result<(), LoadError> {
    match color_type {
        ExtendedColorType::L8
        | ExtendedColorType::La8
        | ExtendedColorType::Rgb8
        | ExtendedColorType::Rgba8 => Ok(()),
        _ => Err(LoadError::UnsupportedTiff),
    }
}

fn validate_dimensions(width: u32, height: u32) -> Result<(), LoadError> {
    if width == 0 || height == 0 {
        return Err(LoadError::InvalidDimensions);
    }
    if width > MAX_SOURCE_SIDE
        || height > MAX_SOURCE_SIDE
        || u64::from(width) * u64::from(height) > u64::from(MAX_SOURCE_PIXELS)
    {
        return Err(LoadError::DimensionsLimit);
    }
    Ok(())
}

fn bounded_preview(image: &RgbaImage) -> RgbaImage {
    let longest = image.width().max(image.height());
    if longest <= PREVIEW_MAX_SIDE {
        return image.clone();
    }
    let scale = PREVIEW_MAX_SIDE as f32 / longest as f32;
    imageops::resize(
        image,
        ((image.width() as f32 * scale).round() as u32).max(1),
        ((image.height() as f32 * scale).round() as u32).max(1),
        imageops::FilterType::Triangle,
    )
}

fn format_name(format: ImageFormat) -> &'static str {
    match format {
        ImageFormat::Png => "PNG",
        ImageFormat::Jpeg => "JPG",
        ImageFormat::WebP => "WEBP",
        ImageFormat::Bmp => "BMP",
        ImageFormat::Tga => "TGA",
        ImageFormat::Tiff => "TIFF",
        ImageFormat::Avif => "AVIF",
        _ => "IMG",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::codecs::jpeg::JpegEncoder;
    use image::{ExtendedColorType, ImageEncoder, Rgb, RgbImage, Rgba};
    use tiff::encoder::{Compression, DeflateLevel, Predictor, TiffEncoder, colortype};
    use tiff::tags::Tag;

    fn noise(width: u32, height: u32) -> RgbaImage {
        let mut seed = 0xA11CE_u32;
        RgbaImage::from_fn(width, height, |_, _| {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            Rgba([
                (seed & 0xFF) as u8,
                ((seed >> 8) & 0xFF) as u8,
                ((seed >> 16) & 0xFF) as u8,
                255,
            ])
        })
    }

    #[test]
    fn renders_svg() {
        let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="64" height="40"><rect width="64" height="40" fill="#ff0000"/></svg>"##;
        let image = svg::decode_svg(svg, Path::new("test.svg")).unwrap();
        assert_eq!(image.dimensions(), (64, 40));
        let pixel = image.get_pixel(32, 20);
        assert!(pixel[0] > 240 && pixel[1] < 10 && pixel[2] < 10);
    }

    #[test]
    fn encodes_supported_formats() {
        let source = noise(120, 80);
        for format in [
            OutputFormat::Webp,
            OutputFormat::Jpg,
            OutputFormat::Png,
            OutputFormat::Avif,
            OutputFormat::Jxl,
        ] {
            let output = encode(
                &source,
                &EncodeOptions {
                    format,
                    quality: 82,
                    ico_sizes: Vec::new(),
                    background: [255, 255, 255, 255],
                },
            )
            .unwrap();
            assert!(!output.bytes.is_empty(), "{format:?}");
            assert_eq!(output.width, 120);
            assert_eq!(output.height, 80);
        }
    }

    #[test]
    fn quality_affects_lossy_size() {
        let source = noise(256, 256);
        let low = encode(
            &source,
            &EncodeOptions {
                format: OutputFormat::Webp,
                quality: 20,
                ico_sizes: Vec::new(),
                background: [255; 4],
            },
        )
        .unwrap();
        let high = encode(
            &source,
            &EncodeOptions {
                format: OutputFormat::Webp,
                quality: 92,
                ico_sizes: Vec::new(),
                background: [255; 4],
            },
        )
        .unwrap();
        assert!(low.bytes.len() < high.bytes.len());
    }

    #[test]
    fn jxl_preserves_alpha() {
        let mut source = RgbaImage::new(8, 8);
        source.put_pixel(2, 2, Rgba([255, 0, 0, 128]));
        let output = encode(
            &source,
            &EncodeOptions {
                format: OutputFormat::Jxl,
                quality: 82,
                ico_sizes: Vec::new(),
                background: [255; 4],
            },
        )
        .unwrap();
        assert_eq!(output.preview.get_pixel(2, 2).0[3], 128);
    }

    #[test]
    fn png_preserves_alpha() {
        let mut source = RgbaImage::new(8, 8);
        source.put_pixel(2, 2, Rgba([255, 0, 0, 128]));
        let output = encode(
            &source,
            &EncodeOptions {
                format: OutputFormat::Png,
                quality: 82,
                ico_sizes: Vec::new(),
                background: [255; 4],
            },
        )
        .unwrap();
        let decoded = image::load_from_memory(&output.bytes).unwrap().to_rgba8();
        assert_eq!(decoded.get_pixel(2, 2).0[3], 128);
    }
    fn tiff_rgb_pixels(width: u32, height: u32) -> Vec<u8> {
        let mut pixels = Vec::with_capacity((width * height * 3) as usize);
        for _ in 0..height {
            for x in 0..width {
                if x < width / 2 {
                    pixels.extend_from_slice(&[220, 24, 24]);
                } else {
                    pixels.extend_from_slice(&[24, 24, 220]);
                }
            }
        }
        pixels
    }

    fn tiff_rgb_bytes(compression: Compression, predictor: Predictor) -> Vec<u8> {
        let pixels = tiff_rgb_pixels(32, 20);
        let mut bytes = Vec::new();
        {
            let mut encoder = TiffEncoder::new(Cursor::new(&mut bytes))
                .unwrap()
                .with_compression(compression)
                .with_predictor(predictor);
            encoder
                .write_image::<colortype::RGB8>(32, 20, &pixels)
                .unwrap();
        }
        bytes
    }

    fn tiff_orientation_bytes() -> Vec<u8> {
        let width = 32;
        let height = 20;
        let pixels = tiff_rgb_pixels(width, height);
        let mut bytes = Vec::new();
        {
            let mut encoder = TiffEncoder::new(Cursor::new(&mut bytes)).unwrap();
            let mut directory = encoder.image_directory().unwrap();
            let offset = directory.write_data(pixels.as_slice()).unwrap();
            directory.write_tag(Tag::ImageWidth, width).unwrap();
            directory.write_tag(Tag::ImageLength, height).unwrap();
            directory
                .write_tag(Tag::BitsPerSample, &[8u16, 8, 8][..])
                .unwrap();
            directory.write_tag(Tag::Compression, 1u16).unwrap();
            directory
                .write_tag(Tag::PhotometricInterpretation, 2u16)
                .unwrap();
            directory
                .write_tag(Tag::StripOffsets, u32::try_from(offset).unwrap())
                .unwrap();
            directory.write_tag(Tag::SamplesPerPixel, 3u16).unwrap();
            directory.write_tag(Tag::RowsPerStrip, height).unwrap();
            directory
                .write_tag(Tag::StripByteCounts, u32::try_from(pixels.len()).unwrap())
                .unwrap();
            directory.write_tag(Tag::PlanarConfiguration, 1u16).unwrap();
            directory.write_tag(Tag::Orientation, 6u16).unwrap();
            directory.finish().unwrap();
        }
        bytes
    }

    #[test]
    fn loads_tiff_file_and_detects_format() {
        let path = std::env::temp_dir().join(format!(
            "hunk-converter-{}-{}.tiff",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::write(
            &path,
            tiff_rgb_bytes(Compression::Uncompressed, Predictor::None),
        )
        .unwrap();
        let loaded = load_image_file(&path).unwrap();
        let _ = std::fs::remove_file(&path);
        assert_eq!(loaded.format, "TIFF");
        assert_eq!(loaded.pixels.dimensions(), (32, 20));
        assert_red(loaded.pixels.get_pixel(4, 10).0);
        assert_blue(loaded.pixels.get_pixel(28, 10).0);
    }

    #[test]
    fn tiff_compressions_decode() {
        let cases = [
            ("sin compresión", Compression::Uncompressed, Predictor::None),
            ("LZW", Compression::Lzw, Predictor::Horizontal),
            (
                "Deflate",
                Compression::Deflate(DeflateLevel::default()),
                Predictor::Horizontal,
            ),
            ("PackBits", Compression::Packbits, Predictor::None),
        ];
        for (name, compression, predictor) in cases {
            let image =
                decode_raster(&tiff_rgb_bytes(compression, predictor), ImageFormat::Tiff).unwrap();
            assert_eq!(image.dimensions(), (32, 20), "{name}");
            assert_red(image.get_pixel(4, 10).0);
            assert_blue(image.get_pixel(28, 10).0);
        }
    }

    #[test]
    fn tiff_grayscale_converts_to_rgba8() {
        let pixels = vec![128u8; 16 * 8];
        let mut bytes = Vec::new();
        {
            let mut encoder = TiffEncoder::new(Cursor::new(&mut bytes)).unwrap();
            encoder
                .write_image::<colortype::Gray8>(16, 8, &pixels)
                .unwrap();
        }
        let image = decode_raster(&bytes, ImageFormat::Tiff).unwrap();
        assert_eq!(image.get_pixel(0, 0).0, [128, 128, 128, 255]);
    }

    #[test]
    fn tiff_rgba_preserves_alpha() {
        let mut pixels = vec![255u8; 8 * 8 * 4];
        pixels[..4].copy_from_slice(&[255, 0, 0, 128]);
        let mut bytes = Vec::new();
        {
            let mut encoder = TiffEncoder::new(Cursor::new(&mut bytes)).unwrap();
            encoder
                .write_image::<colortype::RGBA8>(8, 8, &pixels)
                .unwrap();
        }
        let image = decode_raster(&bytes, ImageFormat::Tiff).unwrap();
        assert_eq!(image.get_pixel(0, 0).0, [255, 0, 0, 128]);
    }

    #[test]
    fn tiff_applies_orientation() {
        let image = decode_raster(&tiff_orientation_bytes(), ImageFormat::Tiff).unwrap();
        assert_eq!(image.dimensions(), (20, 32));
        assert_red(image.get_pixel(5, 5).0);
        assert_blue(image.get_pixel(5, 26).0);
    }

    #[test]
    fn tiff_loads_first_page_only() {
        let first = tiff_rgb_pixels(32, 20);
        let second = vec![0u8; 8 * 8 * 3];
        let mut bytes = Vec::new();
        {
            let mut encoder = TiffEncoder::new(Cursor::new(&mut bytes)).unwrap();
            encoder
                .write_image::<colortype::RGB8>(32, 20, &first)
                .unwrap();
            encoder
                .write_image::<colortype::RGB8>(8, 8, &second)
                .unwrap();
        }
        let image = decode_raster(&bytes, ImageFormat::Tiff).unwrap();
        assert_eq!(image.dimensions(), (32, 20));
        assert_red(image.get_pixel(4, 10).0);
    }

    #[test]
    fn tiff_16_bit_is_rejected() {
        let pixels = vec![0u16; 8 * 8 * 4];
        let mut bytes = Vec::new();
        {
            let mut encoder = TiffEncoder::new(Cursor::new(&mut bytes)).unwrap();
            encoder
                .write_image::<colortype::RGBA16>(8, 8, &pixels)
                .unwrap();
        }
        assert!(matches!(
            decode_raster(&bytes, ImageFormat::Tiff),
            Err(LoadError::UnsupportedTiff)
        ));
    }

    fn jpeg_luma_sampling(bytes: &[u8]) -> Option<(u8, u8)> {
        let mut index = 2;
        while index + 9 < bytes.len() {
            if bytes[index] != 0xFF {
                index += 1;
                continue;
            }
            let marker = bytes[index + 1];
            if matches!(marker, 0xC0..=0xC3) {
                let component = index + 10;
                return Some((bytes[component + 1] >> 4, bytes[component + 1] & 0x0F));
            }
            if matches!(marker, 0xD8 | 0xD9 | 0x01) || (0xD0..=0xD7).contains(&marker) {
                index += 2;
                continue;
            }
            let length = usize::from(u16::from_be_bytes([bytes[index + 2], bytes[index + 3]]));
            index += 2 + length;
        }
        None
    }

    #[test]
    fn jpeg_is_progressive_and_uses_auto_subsampling() {
        let source = noise(64, 48);
        let encode_at = |quality| {
            encode(
                &source,
                &EncodeOptions {
                    format: OutputFormat::Jpg,
                    quality,
                    ico_sizes: Vec::new(),
                    background: [255; 4],
                },
            )
            .unwrap()
            .bytes
        };

        for (quality, expected) in [(75, (2, 2)), (82, (2, 1)), (90, (1, 1))] {
            let bytes = encode_at(quality);
            assert!(
                bytes.windows(2).any(|window| window == [0xFF, 0xC2]),
                "q{quality} no es progresivo"
            );
            assert_eq!(jpeg_luma_sampling(&bytes), Some(expected), "q{quality}");
        }
    }

    #[test]
    fn png_preserves_all_pixels() {
        let mut source = noise(32, 24);
        source.put_pixel(3, 4, Rgba([12, 34, 56, 78]));
        source.put_pixel(7, 8, Rgba([90, 12, 34, 0]));
        let output = encode(
            &source,
            &EncodeOptions {
                format: OutputFormat::Png,
                quality: 82,
                ico_sizes: Vec::new(),
                background: [255; 4],
            },
        )
        .unwrap();
        let decoded = image::load_from_memory(&output.bytes).unwrap().to_rgba8();
        assert_eq!(decoded.dimensions(), source.dimensions());
        assert!(decoded.pixels().eq(source.pixels()));
    }

    fn jpeg_with_orientation(orientation: u8) -> Vec<u8> {
        let source = RgbImage::from_fn(32, 20, |x, _| {
            if x < 16 {
                Rgb([220, 24, 24])
            } else {
                Rgb([24, 24, 220])
            }
        });
        let mut jpeg = Vec::new();
        JpegEncoder::new_with_quality(Cursor::new(&mut jpeg), 100)
            .write_image(source.as_raw(), 32, 20, ExtendedColorType::Rgb8)
            .unwrap();
        let exif = exif_orientation(orientation);
        let mut with_exif = Vec::with_capacity(jpeg.len() + exif.len());
        with_exif.extend_from_slice(&jpeg[..2]);
        with_exif.extend_from_slice(&exif);
        with_exif.extend_from_slice(&jpeg[2..]);
        with_exif
    }

    fn exif_orientation(orientation: u8) -> Vec<u8> {
        let mut tiff = Vec::new();
        tiff.extend_from_slice(b"II*\0");
        tiff.extend_from_slice(&8u32.to_le_bytes());
        tiff.extend_from_slice(&1u16.to_le_bytes());
        tiff.extend_from_slice(&0x0112u16.to_le_bytes());
        tiff.extend_from_slice(&3u16.to_le_bytes());
        tiff.extend_from_slice(&1u32.to_le_bytes());
        let mut value = [0u8; 4];
        value[..2].copy_from_slice(&(u16::from(orientation)).to_le_bytes());
        tiff.extend_from_slice(&value);
        tiff.extend_from_slice(&0u32.to_le_bytes());

        let mut payload = Vec::new();
        payload.extend_from_slice(b"Exif\0\0");
        payload.extend_from_slice(&tiff);
        let mut app1 = Vec::new();
        app1.extend_from_slice(&[0xFF, 0xE1]);
        app1.extend_from_slice(&((payload.len() + 2) as u16).to_be_bytes());
        app1.extend_from_slice(&payload);
        app1
    }

    fn decoded_orientation(orientation: u8) -> RgbaImage {
        decode_raster(&jpeg_with_orientation(orientation), ImageFormat::Jpeg).unwrap()
    }

    fn assert_red(pixel: [u8; 4]) {
        assert!(
            pixel[0] > 180 && pixel[1] < 80 && pixel[2] < 80,
            "se esperaba rojo: {pixel:?}"
        );
    }

    fn assert_blue(pixel: [u8; 4]) {
        assert!(
            pixel[2] > 180 && pixel[0] < 80 && pixel[1] < 80,
            "se esperaba azul: {pixel:?}"
        );
    }

    #[test]
    fn applies_exif_orientation_one() {
        let image = decoded_orientation(1);
        assert_eq!(image.dimensions(), (32, 20));
        assert_red(image.get_pixel(4, 10).0);
        assert_blue(image.get_pixel(28, 10).0);
    }

    #[test]
    fn applies_exif_orientation_three() {
        let image = decoded_orientation(3);
        assert_eq!(image.dimensions(), (32, 20));
        assert_blue(image.get_pixel(4, 10).0);
        assert_red(image.get_pixel(28, 10).0);
    }

    #[test]
    fn applies_exif_orientation_six() {
        let image = decoded_orientation(6);
        assert_eq!(image.dimensions(), (20, 32));
        assert_red(image.get_pixel(5, 5).0);
        assert_blue(image.get_pixel(5, 26).0);
    }

    #[test]
    fn applies_exif_orientation_eight() {
        let image = decoded_orientation(8);
        assert_eq!(image.dimensions(), (20, 32));
        assert_blue(image.get_pixel(5, 5).0);
        assert_red(image.get_pixel(5, 26).0);
    }
}
