//! Mousefood `Error` enum.

#[derive(thiserror::Error, Debug)]
/// Flushing to the display failed.
#[error("flushing to DrawTarget failed: {0}")]
pub struct FlushError(pub alloc::boxed::Box<dyn core::error::Error>);

/// Represents backend error.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Drawing to the display failed.
    #[error("drawing to DrawTarget failed")]
    DrawError,
    /// Selected [`ClearType`](ratatui_core::backend::ClearType) is not supported by Mousefood.
    #[error("ClearType::{0} is not supported by Mousefood")]
    ClearTypeUnsupported(alloc::string::String),
    /// Flushing to the display failed.
    #[error(transparent)]
    Flush(#[from] FlushError),
    /// Failed to write initial pixels to initialize the display.
    #[error("could not draw reset pixels to DrawTarget, initialization failed")]
    Init,
}
