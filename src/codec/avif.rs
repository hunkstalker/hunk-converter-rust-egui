use std::io::Cursor;

use image::RgbaImage;

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
