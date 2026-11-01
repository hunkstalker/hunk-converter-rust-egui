use crate::domain::CropRect;

use super::HunkApp;
use super::state::CropPreset;

impl HunkApp {
    pub(crate) fn toggle_crop(&mut self) {
        if self.crop_edit.is_some() {
            self.cancel_crop();
        } else {
            self.start_crop();
        }
    }

    pub(crate) fn start_crop(&mut self) {
        if self.format == crate::codec::OutputFormat::Ico {
            return;
        }
        self.eyedropper_active = false;
        self.sample_pending_history = false;
        self.crop_drag = None;
        self.crop_preset = CropPreset::Free;
        self.crop_edit = Some(self.crop.unwrap_or(CropRect::FULL));
    }

    pub(crate) fn cancel_crop(&mut self) {
        self.crop_edit = None;
        self.crop_drag = None;
    }

    pub(crate) fn apply_crop(&mut self) -> bool {
        let Some(draft) = self.crop_edit.take() else {
            return false;
        };
        self.crop_drag = None;
        self.crop_preset = CropPreset::Free;
        let next = if draft.is_full() {
            None
        } else {
            Some(draft.clamped())
        };
        let changed = next != self.crop;
        self.crop = next;
        changed
    }

    pub(crate) fn remove_crop(&mut self) -> bool {
        self.crop_edit = None;
        self.crop_drag = None;
        self.crop_preset = CropPreset::Free;
        self.crop.take().is_some()
    }

    pub(crate) fn set_crop_preset(&mut self, preset: CropPreset) {
        let Some(source) = self.source.as_ref() else {
            return;
        };
        let aspect = source.width as f32 / source.height.max(1) as f32;
        let Some(draft) = self.crop_edit.as_mut() else {
            return;
        };
        self.crop_preset = preset;
        if preset == CropPreset::Free {
            return;
        }
        let ratio = match preset {
            CropPreset::Free => return,
            CropPreset::Original => aspect,
            CropPreset::Square => 1.0,
            CropPreset::FourThree => 4.0 / 3.0,
            CropPreset::SixteenNine => 16.0 / 9.0,
        };
        *draft = CropRect::from_aspect(ratio, aspect);
    }

    pub(crate) fn crop_output_dimensions(
        &self,
        source_width: u32,
        source_height: u32,
    ) -> (u32, u32) {
        self.crop_edit
            .or(self.crop)
            .map(|crop| crop.output_dimensions(source_width, source_height))
            .unwrap_or((source_width, source_height))
    }

    pub(crate) fn clear_crop(&mut self) {
        self.crop = None;
        self.crop_edit = None;
        self.crop_drag = None;
        self.crop_preset = CropPreset::Free;
        self.view_content = None;
    }
}
