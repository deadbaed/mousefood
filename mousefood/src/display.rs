use crate::error::Result;
use crate::prelude::display::*;
use embedded_graphics::prelude::PixelColor;

pub trait DisplayTarget<D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + From<TermColor>,
{
    fn draw_target(&mut self) -> &mut impl DrawTarget<Color = C>;
    fn flush(&mut self) -> Result<()>;
}
