use std::sync::Arc;
use std::time::{Duration, Instant};

use eframe::egui::{Pos2, Rect, TextureHandle};
use image::RgbaImage;

use crate::codec::{EncodedOutput, OutputFormat};
use crate::domain::CropRect;

pub(super) struct SourceImage {
    pub name: String,
    pub stem: String,
    pub format: String,
    pub file_bytes: u64,
    pub width: u32,
    pub height: u32,
    pub original: Arc<RgbaImage>,
    pub pixels: Arc<RgbaImage>,
    pub texture: TextureHandle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CropPreset {
    Free,
    Original,
    Square,
    FourThree,
    SixteenNine,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CropHandle {
    Move,
    North,
    South,
    East,
    West,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct CropDrag {
    pub handle: CropHandle,
    pub start: CropRect,
    pub start_pointer: Pos2,
    pub start_image_rect: Rect,
}

pub(super) struct RenderedResult {
    pub texture: TextureHandle,
    pub bytes: Vec<u8>,
    pub size: u64,
    pub width: u32,
    pub height: u32,
    pub frame_count: usize,
    pub format: OutputFormat,
}

pub(crate) struct SourceInput {
    pub pixels: RgbaImage,
    pub name: String,
    pub stem: String,
    pub format: String,
    pub file_bytes: u64,
}

pub(super) struct EncodeJobResult {
    pub revision: u64,
    pub result: Result<EncodedOutput, crate::domain::EncodeError>,
    pub elapsed: Duration,
}

pub(super) struct PreprocessJobResult {
    pub revision: u64,
    pub pixels: RgbaImage,
    pub preview: RgbaImage,
}

pub(super) struct EncodeCompletion {
    pub output: EncodedOutput,
    pub elapsed: Duration,
}

pub(super) struct Toast {
    pub message: String,
    pub error: bool,
    pub until: Instant,
}

impl Toast {
    pub fn new(message: impl Into<String>, error: bool, ttl: Duration) -> Self {
        Self {
            message: message.into(),
            error,
            until: Instant::now() + ttl,
        }
    }
}
