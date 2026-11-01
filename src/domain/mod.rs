pub mod color;
pub mod error;
pub mod export_name;
pub mod format;
pub mod history;
pub mod preprocess;

pub use color::{color_codes, flatten_rgba, hex_rgba, parse_hex_rgba};
pub use error::{
    AppError, ClipboardError, ConfigError, EncodeError, ExportError, IcoError, LoadError,
};
pub use format::{FormatSelection, OutputFormat, format_bytes};
pub use history::{HISTORY_LIMIT, HistoryEntry, clear_unlocked, remember_color, toggle_lock};
pub use preprocess::{
    PreprocessOptions, apply_geometry, apply_palette, linked_height, linked_width,
    parse_dimension_expression, rotated_size,
};
