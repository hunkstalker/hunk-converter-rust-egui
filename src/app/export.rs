use std::path::PathBuf;

use crate::domain::export_name::{export_file_name, safe_stem};
use crate::domain::{AppError, ExportError};
use crate::i18n::{self, K};

use super::HunkApp;

impl HunkApp {
    pub(crate) fn export(&mut self) {
        if !self.can_export() {
            return;
        }
        let Some(result) = self.result.as_ref() else {
            return;
        };
        let Some(source) = self.source.as_ref() else {
            return;
        };

        let stem = safe_stem(&source.stem);
        let extension = self.format.extension();
        let default_name = export_file_name(&stem, extension, None);
        let mut dialog = rfd::FileDialog::new()
            .set_title(self.t(K::SaveImageAs))
            .set_file_name(&default_name)
            .add_filter(self.format.label(), &[extension]);
        if let Ok(root) = crate::settings::app_root() {
            let export_directory = root.join("export");
            if export_directory.is_dir() {
                dialog = dialog.set_directory(export_directory);
            }
        }
        let Some(path) = dialog.save_file() else {
            return;
        };
        let path = ensure_extension(path, extension);
        if let Err(source) = std::fs::write(&path, &result.bytes) {
            self.toast_error(&AppError::Export(ExportError::Write { path, source }), 4);
            return;
        }
        let file_name = path
            .file_name()
            .map(|value| value.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.display().to_string());
        self.toast(i18n::saved_export(self.language, &file_name), false, 4);
    }
}

fn ensure_extension(mut path: PathBuf, extension: &str) -> PathBuf {
    if path.extension().is_none() {
        path.set_extension(extension);
    }
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_extension_only_when_missing() {
        assert_eq!(
            ensure_extension(PathBuf::from("/tmp/foto"), "webp"),
            PathBuf::from("/tmp/foto.webp")
        );
        assert_eq!(
            ensure_extension(PathBuf::from("/tmp/foto.png"), "webp"),
            PathBuf::from("/tmp/foto.png")
        );
    }
}
