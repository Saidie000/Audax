//! Export formats for raster and layered outputs.

mod png;
mod jpg;
mod psd;

pub use jpg::export_jpg;
pub use png::export_png;
pub use psd::export_psd;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExportError {
    #[error("unsupported format")]
    UnsupportedFormat,
    #[error("io error: {0}")]
    Io(String),
}
