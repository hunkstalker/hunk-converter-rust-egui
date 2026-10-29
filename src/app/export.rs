use std::io::Write;
use std::path::{Path, PathBuf};

use crate::domain::export_name::{export_file_name, safe_stem};
use crate::domain::{AppError, ExportError};
use crate::i18n;

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

        let directory = match export_directory() {
            Ok(directory) => directory,
            Err(error) => {
                self.toast_error(&AppError::Export(error), 4);
                return;
            }
        };
        if let Err(source) = std::fs::create_dir_all(&directory) {
            self.toast_error(
                &AppError::Export(ExportError::CreateDir {
                    path: directory,
                    source,
                }),
                4,
            );
            return;
        }

        let stem = safe_stem(&source.stem);
        let extension = self.format.extension();
        match write_unique(&directory, &stem, extension, &result.bytes) {
            Ok(path) => {
                let file_name = path
                    .file_name()
                    .map(|value| value.to_string_lossy().into_owned())
                    .unwrap_or_else(|| path.display().to_string());
                self.toast(i18n::saved_export(self.language, &file_name), false, 4);
            }
            Err((path, source)) => {
                self.toast_error(&AppError::Export(ExportError::Write { path, source }), 4);
            }
        }
    }
}

fn export_directory() -> Result<PathBuf, ExportError> {
    let root = crate::settings::app_root().map_err(|_| ExportError::MissingExecutableDirectory)?;
    Ok(root.join("export"))
}

fn write_unique(
    directory: &Path,
    stem: &str,
    extension: &str,
    bytes: &[u8],
) -> Result<PathBuf, (PathBuf, std::io::Error)> {
    let mut suffix = None;
    loop {
        let candidate = directory.join(export_file_name(stem, extension, suffix));
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            Ok(mut file) => {
                if let Err(error) = file.write_all(bytes) {
                    drop(file);
                    let _ = std::fs::remove_file(&candidate);
                    return Err((candidate, error));
                }
                return Ok(candidate);
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                suffix = Some(suffix.map_or(2, |value| value + 1));
            }
            Err(error) => return Err((candidate, error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_exports_do_not_overwrite() {
        let directory = std::env::temp_dir().join(format!(
            "hunk-converter-export-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&directory).unwrap();

        let first = write_unique(&directory, "foto", "webp", b"uno").unwrap();
        let second = write_unique(&directory, "foto", "webp", b"dos").unwrap();
        let third = write_unique(&directory, "foto", "webp", b"tres").unwrap();

        assert_eq!(first.file_name().unwrap(), "foto.webp");
        assert_eq!(second.file_name().unwrap(), "foto-2.webp");
        assert_eq!(third.file_name().unwrap(), "foto-3.webp");
        assert_eq!(std::fs::read(&first).unwrap(), b"uno");
        assert_eq!(std::fs::read(&second).unwrap(), b"dos");
        assert_eq!(std::fs::read(&third).unwrap(), b"tres");
        let _ = std::fs::remove_dir_all(&directory);
    }
}
