use std::io::Cursor;
use std::path::{Path, PathBuf};

use image::{DynamicImage, Rgba, RgbaImage};
use tiff::encoder::{TiffEncoder, colortype};

use hunk_converter::codec::{self, EncodeOptions, OutputFormat};
use hunk_converter::domain::LoadError;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 48;

fn source() -> RgbaImage {
    RgbaImage::from_fn(WIDTH, HEIGHT, |x, y| {
        Rgba([
            (x * 4) as u8,
            (y * 5) as u8,
            ((x + y) * 2) as u8,
            if (x + y) % 7 == 0 { 128 } else { 255 },
        ])
    })
}

fn options(format: OutputFormat) -> EncodeOptions {
    EncodeOptions {
        format,
        quality: 82,
        ico_sizes: Vec::new(),
        background: [255, 255, 255, 255],
    }
}

fn temp_dir(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "hunk-converter-test-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&path).unwrap();
    path
}

fn tiff_rgba(source: &RgbaImage) -> Vec<u8> {
    let mut bytes = Vec::new();
    {
        let mut encoder = TiffEncoder::new(Cursor::new(&mut bytes)).unwrap();
        encoder
            .write_image::<colortype::RGBA8>(source.width(), source.height(), source.as_raw())
            .unwrap();
    }
    bytes
}

#[test]
fn loads_all_supported_input_formats() {
    let directory = temp_dir("inputs");
    let source = source();
    let png = directory.join("entrada.png");
    let jpg = directory.join("entrada.jpg");
    let webp = directory.join("entrada.webp");
    let bmp = directory.join("entrada.bmp");
    let tga = directory.join("entrada.tga");
    let tiff = directory.join("entrada.tiff");
    let svg = directory.join("entrada.svg");

    DynamicImage::ImageRgba8(source.clone()).save(&png).unwrap();
    DynamicImage::ImageRgba8(source.clone()).save(&bmp).unwrap();
    DynamicImage::ImageRgba8(source.clone()).save(&tga).unwrap();
    std::fs::write(
        &jpg,
        codec::encode(&source, &options(OutputFormat::Jpg))
            .unwrap()
            .bytes,
    )
    .unwrap();
    std::fs::write(
        &webp,
        codec::encode(&source, &options(OutputFormat::Webp))
            .unwrap()
            .bytes,
    )
    .unwrap();
    std::fs::write(&tiff, tiff_rgba(&source)).unwrap();
    std::fs::write(
        &svg,
        format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{HEIGHT}"><rect width="{WIDTH}" height="{HEIGHT}" fill="#ff0000"/></svg>"##
        ),
    )
    .unwrap();

    let cases: [(&Path, &str); 7] = [
        (&png, "PNG"),
        (&jpg, "JPG"),
        (&webp, "WEBP"),
        (&bmp, "BMP"),
        (&tga, "TGA"),
        (&tiff, "TIFF"),
        (&svg, "SVG"),
    ];
    for (path, expected_format) in cases {
        let loaded = codec::load_image_file(path)
            .unwrap_or_else(|error| panic!("no se pudo cargar {}: {error}", path.display()));
        assert_eq!(loaded.format, expected_format, "{}", path.display());
        assert_eq!(
            loaded.pixels.dimensions(),
            (WIDTH, HEIGHT),
            "{}",
            path.display()
        );
    }

    for path in [&png, &webp, &tiff] {
        let loaded = codec::load_image_file(path).unwrap();
        assert_eq!(loaded.pixels.get_pixel(0, 0).0[3], 128);
    }

    std::fs::remove_dir_all(directory).unwrap();
}

#[test]
fn rejects_images_over_the_side_limit() {
    let directory = temp_dir("side-limit");
    let path = directory.join("demasiado-ancha.png");
    RgbaImage::new(16_385, 1).save(&path).unwrap();

    assert!(matches!(
        codec::load_image_file(&path),
        Err(LoadError::DimensionsLimit)
    ));
    std::fs::remove_dir_all(directory).unwrap();
}

#[test]
fn rejects_files_over_the_byte_limit_before_reading() {
    let directory = temp_dir("byte-limit");
    let path = directory.join("demasiado-grande.png");
    let file = std::fs::File::create(&path).unwrap();
    file.set_len(200 * 1024 * 1024 + 1).unwrap();

    assert!(matches!(
        codec::load_image_file(&path),
        Err(LoadError::TooLarge { .. })
    ));
    std::fs::remove_dir_all(directory).unwrap();
}

#[test]
fn rejects_corrupt_images_with_a_controlled_error() {
    let directory = temp_dir("corrupt");
    let path = directory.join("corrupta.png");
    std::fs::write(&path, b"esto no es una imagen").unwrap();

    assert!(codec::load_image_file(&path).is_err());
    std::fs::remove_dir_all(directory).unwrap();
}
