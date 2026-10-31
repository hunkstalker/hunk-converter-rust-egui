#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FormatSelection {
    #[default]
    Auto,
    Fixed(OutputFormat),
}

impl FormatSelection {
    pub fn code(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Fixed(format) => format.extension(),
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        let code = code.trim().to_ascii_lowercase();
        if code == "auto" {
            Some(Self::Auto)
        } else {
            OutputFormat::from_code(&code).map(Self::Fixed)
        }
    }

    pub fn resolve(self, input: Option<&str>) -> OutputFormat {
        match self {
            Self::Fixed(format) => format,
            Self::Auto => input
                .and_then(OutputFormat::from_input_label)
                .unwrap_or(OutputFormat::Webp),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Webp,
    Jpg,
    Png,
    Ico,
    Avif,
    Jxl,
}

impl OutputFormat {
    pub const ALL: [Self; 5] = [Self::Webp, Self::Jpg, Self::Png, Self::Avif, Self::Ico];

    pub fn label(self) -> &'static str {
        match self {
            Self::Webp => "WebP",
            Self::Jpg => "JPG",
            Self::Png => "PNG",
            Self::Ico => "ICO",
            Self::Avif => "AVIF",
            Self::Jxl => "JPEG XL",
        }
    }

    pub fn badge(self) -> &'static str {
        match self {
            Self::Webp => "WEBP",
            Self::Jpg => "JPG",
            Self::Png => "PNG",
            Self::Ico => "ICO",
            Self::Avif => "AVIF",
            Self::Jxl => "JXL",
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Webp => "webp",
            Self::Jpg => "jpg",
            Self::Png => "png",
            Self::Ico => "ico",
            Self::Avif => "avif",
            Self::Jxl => "jxl",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code.trim().to_ascii_lowercase().as_str() {
            "webp" => Some(Self::Webp),
            "jpg" | "jpeg" => Some(Self::Jpg),
            "png" => Some(Self::Png),
            "ico" => Some(Self::Ico),
            "avif" => Some(Self::Avif),
            _ => None,
        }
    }

    pub fn from_input_label(label: &str) -> Option<Self> {
        match label.trim().to_ascii_uppercase().as_str() {
            "PNG" => Some(Self::Png),
            "JPG" | "JPEG" => Some(Self::Jpg),
            "WEBP" => Some(Self::Webp),
            "AVIF" => Some(Self::Avif),
            "JXL" | "JPEG XL" => Some(Self::Jxl),
            _ => None,
        }
    }

    pub fn is_lossy(self) -> bool {
        matches!(self, Self::Webp | Self::Jpg | Self::Avif)
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / KB).replace('.', ",")
    } else {
        format!("{:.2} MB", bytes as f64 / MB).replace('.', ",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_expose_a_single_policy_source() {
        assert_eq!(OutputFormat::Webp.extension(), "webp");
        assert!(OutputFormat::Webp.is_lossy());
        assert!(!OutputFormat::Png.is_lossy());
        assert!(!OutputFormat::Jxl.is_lossy());
        assert_eq!(OutputFormat::Jxl.badge(), "JXL");
    }

    #[test]
    fn format_selection_round_trips_and_resolves_inputs() {
        assert_eq!(FormatSelection::Auto.code(), "auto");
        assert_eq!(
            FormatSelection::from_code("auto"),
            Some(FormatSelection::Auto)
        );
        assert_eq!(
            FormatSelection::from_code("jpeg"),
            Some(FormatSelection::Fixed(OutputFormat::Jpg))
        );
        assert_eq!(FormatSelection::from_code("nope"), None);
        assert_eq!(FormatSelection::from_code("jxl"), None);
        assert!(!OutputFormat::ALL.contains(&OutputFormat::Jxl));
        assert_eq!(
            OutputFormat::from_input_label("PNG"),
            Some(OutputFormat::Png)
        );
        assert_eq!(
            OutputFormat::from_input_label("jpeg"),
            Some(OutputFormat::Jpg)
        );
        assert_eq!(
            OutputFormat::from_input_label("WEBP"),
            Some(OutputFormat::Webp)
        );
        assert_eq!(OutputFormat::from_input_label("BMP"), None);
        assert_eq!(OutputFormat::from_input_label("SVG"), None);
        assert_eq!(
            FormatSelection::Auto.resolve(Some("PNG")),
            OutputFormat::Png
        );
        assert_eq!(
            FormatSelection::Auto.resolve(Some("JPG")),
            OutputFormat::Jpg
        );
        assert_eq!(
            FormatSelection::Auto.resolve(Some("BMP")),
            OutputFormat::Webp
        );
        assert_eq!(
            FormatSelection::Auto.resolve(Some("SVG")),
            OutputFormat::Webp
        );
        assert_eq!(
            FormatSelection::Fixed(OutputFormat::Avif).resolve(Some("PNG")),
            OutputFormat::Avif
        );
    }

    #[test]
    fn formats_human_readable_sizes() {
        assert_eq!(format_bytes(900), "900 B");
        assert_eq!(format_bytes(1536), "1,5 KB");
        assert_eq!(format_bytes(2 * 1024 * 1024), "2,00 MB");
    }
}
