#![no_std]
#![doc = include_str!("../../README.md")]

extern crate alloc;

mod backend;
mod colors;
mod default_font;
pub mod error;
mod framebuffer;
mod macros;
pub mod prelude;

pub use backend::{EmbeddedBackend, EmbeddedBackendConfig};
pub use embedded_graphics;

/// Wrapping [Result](core::result::Result) with [Error](crate::error::Error) as the failure type
pub type Result<T, E = error::Error> = core::result::Result<T, E>;

#[cfg(feature = "fonts")]
pub use embedded_graphics_unicodefonts as fonts;
