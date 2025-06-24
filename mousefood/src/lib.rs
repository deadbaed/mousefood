#![no_std]
#![doc = include_str!("../../README.md")]

extern crate alloc;

mod backend;
mod colors;
mod default_font;
mod display;
pub mod error;
mod framebuffer;
mod macros;
pub mod prelude;

pub use backend::{EmbeddedBackend, EmbeddedBackendConfig};
pub use display::BufferedDisplay;
pub use embedded_graphics;
pub use framebuffer::HeapBuffer;

pub type Result<T, E = error::Error> = core::result::Result<T, E>;

#[cfg(feature = "fonts")]
pub use embedded_graphics_unicodefonts as fonts;
