use std::sync::Arc;
use std::sync::mpsc::{self, TryRecvError};
use std::time::{Duration, Instant};

use eframe::egui;

use crate::domain::apply_geometry;

use super::HunkApp;
use super::UI_PREVIEW_MAX;
use super::state::PreprocessJobResult;
use super::ui::widgets::{resized_for_ui, upload_texture};

impl HunkApp {
    pub(crate) fn rotate_quick(&mut self, ctx: &egui::Context) {
        if self.source.is_none() {
            return;
        }
        self.advanced.preprocess.rotation = (self.advanced.preprocess.rotation + 90) % 360;
        if self.advanced.preprocess.resize_enabled {
            std::mem::swap(
                &mut self.advanced.preprocess.width,
                &mut self.advanced.preprocess.height,
            );
        }
        self.request_preprocess(ctx, None);
    }

    pub(crate) fn toggle_invert(&mut self, ctx: &egui::Context) {
        if self.source.is_none() {
            return;
        }
        self.advanced.preprocess.invert = !self.advanced.preprocess.invert;
        self.request_preprocess(ctx, None);
    }

    pub(crate) fn request_preprocess(&mut self, ctx: &egui::Context, delay: Option<Duration>) {
        if self.source.is_none() {
            return;
        }
        self.preprocess_revision = self.preprocess_revision.wrapping_add(1);
        self.preprocess_pending_since = Some(Instant::now() + delay.unwrap_or(Duration::ZERO));
        self.pending_since = None;
        self.revision = self.revision.wrapping_add(1);
        self.result = None;
        self.eyedropper_active = false;
        self.sampled_color = None;
        self.sampled_position = None;
        self.sample_pending_history = false;
        self.screenshot_at = None;
        self.screenshot_requested = false;
        ctx.request_repaint();
    }

    pub(crate) fn maybe_start_preprocess(&mut self, ctx: &egui::Context) {
        if self.preprocess_worker.is_some() || self.source.is_none() {
            return;
        }
        let Some(due) = self.preprocess_pending_since else {
            return;
        };
        let now = Instant::now();
        if now < due {
            ctx.request_repaint_after(due - now);
            return;
        }
        let Some(original) = self
            .source
            .as_ref()
            .map(|source| Arc::clone(&source.original))
        else {
            return;
        };
        let options = self.advanced.preprocess.clone();
        let revision = self.preprocess_revision;
        let (sender, receiver) = mpsc::channel();
        let thread_ctx = ctx.clone();
        std::thread::spawn(move || {
            let pixels = apply_geometry(&original, &options);
            let preview = resized_for_ui(&pixels, UI_PREVIEW_MAX);
            let _ = sender.send(PreprocessJobResult {
                revision,
                pixels,
                preview,
            });
            thread_ctx.request_repaint();
        });
        self.preprocess_worker = Some(receiver);
        self.preprocess_pending_since = None;
        ctx.request_repaint();
    }

    pub(crate) fn poll_preprocess(&mut self, ctx: &egui::Context) {
        let Some(receiver) = self.preprocess_worker.as_ref() else {
            return;
        };
        match receiver.try_recv() {
            Ok(message) => {
                self.preprocess_worker = None;
                if message.revision == self.preprocess_revision
                    && self.preprocess_pending_since.is_none()
                {
                    let (width, height) = message.pixels.dimensions();
                    let texture = upload_texture(
                        ctx,
                        &format!("source-{}", self.preprocess_revision),
                        &message.preview,
                    );
                    if let Some(source) = self.source.as_mut() {
                        source.width = width;
                        source.height = height;
                        source.pixels = Arc::new(message.pixels);
                        source.texture = texture;
                    }
                    if self.actual_size {
                        let (original_width, original_height) = self
                            .source
                            .as_ref()
                            .map(|source| source.original.dimensions())
                            .unwrap_or((0, 0));
                        let (rotated_width, rotated_height) = crate::domain::rotated_size(
                            original_width,
                            original_height,
                            self.advanced.preprocess.rotation,
                        );
                        if self.advanced.preprocess.resize_enabled
                            && (
                                self.advanced.preprocess.width,
                                self.advanced.preprocess.height,
                            ) != (rotated_width, rotated_height)
                        {
                            self.actual_size = false;
                            self.zoom = super::FIT_ZOOM;
                            self.pan = egui::Vec2::ZERO;
                            self.pending_zoom = None;
                        }
                    }
                    self.result = None;
                    self.eyedropper_active = false;
                    self.sampled_color = None;
                    self.sampled_position = None;
                    self.sample_pending_history = false;
                    self.request_encode(ctx, None);
                }
                ctx.request_repaint();
            }
            Err(TryRecvError::Disconnected) => {
                self.preprocess_worker = None;
                ctx.request_repaint();
            }
            Err(TryRecvError::Empty) => {
                ctx.request_repaint_after(Duration::from_millis(16));
            }
        }
    }
}
