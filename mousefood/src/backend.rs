use core::marker::PhantomData;

use crate::colors::*;
use crate::default_font;
use embedded_graphics::Drawable;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{self, Dimensions};
use embedded_graphics::mono_font::{MonoFont, MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::{PixelColor, Rgb888};
use embedded_graphics::text::Text;
use ratatui_core::backend::{Backend, ClearType};
use ratatui_core::layout;
use ratatui_core::style;

pub trait MousefoodDisplay<D, C>
where
    D: DrawTarget<Color = C> + Dimensions,
    C: PixelColor + From<TermColor>,
{
    fn get_drawable_target(&mut self) -> &mut (impl DrawTarget<Color = C> + Dimensions);
    fn flush(&mut self) -> Result<()>;
}

/// Embedded backend configuration.
pub struct EmbeddedBackendConfig {
    /// Regular font.
    pub font_regular: MonoFont<'static>,
    /// Bold font.
    pub font_bold: Option<MonoFont<'static>>,
    /// Italic font.
    pub font_italic: Option<MonoFont<'static>>,
}

impl Default for EmbeddedBackendConfig {
    fn default() -> Self {
        Self {
            font_regular: default_font::regular,
            font_bold: None,
            font_italic: None,
        }
    }
}

/// Embedded backend for Ratatui.
///
/// # Examples
///
/// ```rust,no_run
/// use mousefood::prelude::*;
///
/// let backend = EmbeddedBackend::new(&mut display, EmbeddedBackendConfig::default());
/// let mut terminal = Terminal::new(backend).unwrap();
/// ```
pub struct EmbeddedBackend<'display, D, C, M>
where
    M: MousefoodDisplay<D, C>,
    D: DrawTarget<Color = C> + 'display,
    C: PixelColor + 'display + From<TermColor>,
{
    display: &'display mut M,
    display_type: PhantomData<D>,
    font_regular: MonoFont<'static>,
    font_bold: Option<MonoFont<'static>>,
    font_italic: Option<MonoFont<'static>>,

    char_offset: geometry::Point,

    columns_rows: layout::Size,
    pixels: layout::Size,
}

impl<'display, D, C, M> EmbeddedBackend<'display, D, C, M>
where
    M: MousefoodDisplay<D, C> + Dimensions,
    D: DrawTarget<Color = C> + Dimensions + 'static,
    C: PixelColor + Into<Rgb888> + From<Rgb888> + From<TermColor> + 'static,
{
    fn init(
        display: &'display mut M,
        font_regular: MonoFont<'static>,
        font_bold: Option<MonoFont<'static>>,
        font_italic: Option<MonoFont<'static>>,
    ) -> Result<EmbeddedBackend<'display, D, C, M>> {
        let pixels = layout::Size {
            width: display.bounding_box().size.width as u16,
            height: display.bounding_box().size.height as u16,
        };

        let mut backend = Self {
            display,
            display_type: PhantomData,
            font_regular,
            font_bold,
            font_italic,
            char_offset: geometry::Point::new(0, font_regular.character_size.height as i32),
            columns_rows: layout::Size {
                height: pixels.height / font_regular.character_size.height as u16,
                width: pixels.width / font_regular.character_size.width as u16,
            },
            pixels,
        };

        // Start with a clear display to have a coherent look on unbuffered and buffered display
        backend.clear().map_err(|_| crate::error::Error::Init)?;

        Ok(backend)
    }

    /// Creates a new `EmbeddedBackend` using default fonts.
    pub fn new(
        display: &'display mut M,
        config: EmbeddedBackendConfig,
    ) -> Result<EmbeddedBackend<'display, D, C, M>> {
        Self::init(
            display,
            config.font_regular,
            config.font_bold,
            config.font_italic,
        )
    }
}

type Result<T, E = crate::error::Error> = core::result::Result<T, E>;

impl<D, C, M> Backend for EmbeddedBackend<'_, D, C, M>
where
    M: MousefoodDisplay<D, C>,
    D: DrawTarget<Color = C> + 'static,
    C: PixelColor + Into<Rgb888> + From<Rgb888> + From<TermColor> + 'static,
{
    type Error = crate::error::Error;

    fn draw<'a, I>(&mut self, content: I) -> Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a ratatui_core::buffer::Cell)>,
    {
        for (x, y, cell) in content {
            let position = geometry::Point::new(
                x as i32 * self.font_regular.character_size.width as i32,
                y as i32 * self.font_regular.character_size.height as i32,
            );

            let mut style_builder = MonoTextStyleBuilder::new()
                .font(&self.font_regular)
                .text_color(TermColor(cell.fg, TermColorType::Foreground).into())
                .background_color(TermColor(cell.bg, TermColorType::Background).into());

            for modifier in cell.modifier.iter() {
                style_builder = match modifier {
                    style::Modifier::BOLD => match &self.font_bold {
                        None => style_builder.font(&self.font_regular),
                        Some(font) => style_builder.font(font),
                    },
                    style::Modifier::DIM => style_builder, // TODO
                    style::Modifier::ITALIC => match &self.font_italic {
                        None => style_builder.font(&self.font_regular),
                        Some(font) => style_builder.font(font),
                    },
                    style::Modifier::UNDERLINED => style_builder.underline(),
                    style::Modifier::SLOW_BLINK => style_builder, // TODO
                    style::Modifier::RAPID_BLINK => style_builder, // TODO
                    style::Modifier::REVERSED => style_builder,   // TODO
                    style::Modifier::HIDDEN => style_builder,     // TODO
                    style::Modifier::CROSSED_OUT => style_builder.strikethrough(),
                    _ => style_builder,
                }
            }

            if cell.underline_color != style::Color::Reset {
                style_builder = style_builder.underline_with_color(
                    TermColor(cell.underline_color, TermColorType::Foreground).into(),
                );
            }

            Text::new(
                cell.symbol(),
                position + self.char_offset,
                style_builder.build(),
            )
            .draw(self.display.get_drawable_target())
            .map_err(|_| crate::error::Error::DrawError)?;
        }
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<()> {
        // TODO
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<()> {
        // TODO
        Ok(())
    }

    fn get_cursor_position(&mut self) -> Result<layout::Position> {
        // TODO
        Ok(layout::Position::new(0, 0))
    }

    fn set_cursor_position<P: Into<layout::Position>>(
        &mut self,
        #[allow(unused_variables)] position: P,
    ) -> Result<()> {
        // TODO
        Ok(())
    }

    fn clear(&mut self) -> Result<()> {
        self.display
            .get_drawable_target()
            .clear(crate::colors::INITIAL_COLOR.into())
            .map_err(|_| crate::error::Error::DrawError)
    }

    fn clear_region(&mut self, clear_type: ClearType) -> Result<()> {
        match clear_type {
            ClearType::All => self.clear(),
            ClearType::AfterCursor
            | ClearType::BeforeCursor
            | ClearType::CurrentLine
            | ClearType::UntilNewLine => Err(crate::error::Error::ClearTypeUnsupported(
                alloc::format!("{:?}", clear_type),
            )),
        }
    }

    fn size(&self) -> Result<layout::Size> {
        Ok(self.columns_rows)
    }

    fn window_size(&mut self) -> Result<ratatui_core::backend::WindowSize> {
        Ok(ratatui_core::backend::WindowSize {
            columns_rows: self.columns_rows,
            pixels: self.pixels,
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.display.flush()
    }
}
