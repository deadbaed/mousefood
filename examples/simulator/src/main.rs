//! # Simulator
//!
//! Run mousefood apps on your computer inside a simulator! Uses [embedded-graphics-simulator](https://crates.io/crates/embedded-graphics-simulator).
//!
//! ## Requirements
//!
//! This app requires [SDL2](https://wiki.libsdl.org/SDL2/Installation) to be installed.
//!
//! If you use [nix](https://nixos.org) you can run `nix-shell -p SDL2`
//! before running the application.
//!
//! ## Run
//!
//! To start this demo, simply run:
//!
//! ```shell
//! cargo run -p simulator
//! ```
//!
//! A window will open with the simulator running.

use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};
use mousefood::embedded_graphics::geometry;
use mousefood::embedded_graphics::prelude::{Dimensions, DrawTarget};
use mousefood::error::Error;
use mousefood::prelude::*;
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{Frame, Terminal, style::*};

struct MySuperDisplay {
    simulator_window: Window,
    display: SimulatorDisplay<Bgr565>,
}

impl mousefood::Flush for MySuperDisplay {
    fn flush(&mut self) -> Result<(), mousefood::FlushError> {
        self.simulator_window.update(&self.display);
        if self
            .simulator_window
            .events()
            .any(|e| e == SimulatorEvent::Quit)
        {
            return Err(mousefood::FlushError);
            // panic!("simulator window closed");
        }
        Ok(())
    }
}

impl DrawTarget for MySuperDisplay {
    type Color = Bgr565;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = mousefood::embedded_graphics::Pixel<Self::Color>>,
    {
        self.display.draw_iter(pixels)
    }
}

impl Dimensions for MySuperDisplay {
    fn bounding_box(&self) -> mousefood::embedded_graphics::primitives::Rectangle {
        self.display.bounding_box()
    }
}

fn main() -> Result<(), Error> {
    let mut display = MySuperDisplay {
        display: SimulatorDisplay::<Bgr565>::new(geometry::Size::new(128, 64)),
        simulator_window: Window::new(
            "mousefood simulator",
            &OutputSettings {
                scale: 4,
                max_fps: 30,
                ..Default::default()
            },
        ),
    };

    let backend = EmbeddedBackend::new(&mut display, EmbeddedBackendConfig::default());

    // Start ratatui with our simulator backend
    println!("starting ratatui");
    let mut terminal = Terminal::new(backend)?;

    // Run an infinite loop, where widgets will be rendered
    loop {
        terminal.draw(draw)?;
    }
}

fn draw(frame: &mut Frame) {
    let text = "Ratatui on embedded devices!";
    let paragraph = Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true });
    let bordered_block = Block::bordered()
        .border_style(Style::new().yellow())
        .title("Mousefood");
    frame.render_widget(paragraph.block(bordered_block), frame.area());
    println!("drew widget");
}
