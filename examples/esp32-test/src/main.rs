use mousefood::embedded_graphics::prelude::{Dimensions, DrawTarget};
use mousefood::{prelude::*, FlushError};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::*;
use ratatui::{Frame, Terminal};
use ssd1306::mode::DisplayConfig;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let p = esp_idf_svc::hal::prelude::Peripherals::take().unwrap();

    let i2c = p.i2c0;
    let sda = p.pins.gpio21;
    let scl = p.pins.gpio22;
    let config = esp_idf_svc::hal::i2c::config::Config::new();

    let i2c = match esp_idf_svc::hal::i2c::I2cDriver::new(i2c, sda, scl, &config) {
        Ok(i2c) => i2c,
        Err(e) => {
            log::error!("failed to crate i2c driver {}", e);
            panic!();
        }
    };

    let interface = ssd1306::I2CDisplayInterface::new(i2c);
    type MyDisplay = ssd1306::Ssd1306<
        ssd1306::prelude::I2CInterface<esp_idf_svc::hal::i2c::I2cDriver<'static>>,
        ssd1306::size::DisplaySize128x64,
        ssd1306::mode::BufferedGraphicsMode<ssd1306::size::DisplaySize128x64>,
    >;

    let mut display = MySuperDisplay {
        display: ssd1306::Ssd1306::new(
            interface,
            ssd1306::size::DisplaySize128x64,
            ssd1306::prelude::DisplayRotation::Rotate0,
        )
        .into_buffered_graphics_mode(),
    };
    display.display.init().unwrap();

    struct MySuperDisplay {
        display: MyDisplay,
    }

    impl mousefood::Flush for MySuperDisplay {
        fn flush(&mut self) -> Result<(), FlushError> {
            self.display.flush().map_err(|e| FlushError)
        }
    }

    impl DrawTarget for MySuperDisplay {
        type Color = mousefood::embedded_graphics::pixelcolor::BinaryColor;
        type Error = display_interface::DisplayError;

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

    let backend = EmbeddedBackend::new(&mut display, EmbeddedBackendConfig::default());
    let mut terminal = Terminal::new(backend).unwrap();
    loop {
        match terminal.draw(|f| draw(f, "Ratatui on embedded devices!")) {
            Ok(_completed_frame) => log::info!("completed mousefood demo"),
            Err(e) => log::error!("failed to draw mousefood demo {}", e),
        };
        std::thread::sleep(std::time::Duration::from_millis(1500));
        match terminal.draw(|f| draw(f, "Give food to mouse!")) {
            Ok(_completed_frame) => log::info!("completed mousefood demo 2"),
            Err(e) => log::error!("failed to draw mousefood demo 2 {}", e),
        };
        std::thread::sleep(std::time::Duration::from_millis(1500));
        match terminal.draw(|f| draw(f, "I like traits and trains")) {
            Ok(_completed_frame) => log::info!("completed mousefood demo 3"),
            Err(e) => log::error!("failed to draw mousefood demo 3 {}", e),
        };
        std::thread::sleep(std::time::Duration::from_millis(1500));
    }
}

fn draw(frame: &mut Frame, text: &str) {
    let paragraph = Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true });
    let bordered_block = Block::bordered()
        .border_style(Style::new().yellow())
        .title("Mousefood");
    frame.render_widget(paragraph.block(bordered_block), frame.area());
}
