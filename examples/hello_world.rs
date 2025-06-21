#![no_main]
#![no_std]

extern crate alloc;

use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;

use uefi_graphics2::UefiDisplay;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    // Disable the watchdog timer
    boot::set_watchdog_timer(0, 0x10000, None).unwrap();

    // Get gop
    let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop = boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle).unwrap();

    // Create UefiDisplay
    let mode = gop.current_mode_info();
    let mut display = UefiDisplay::new(gop.frame_buffer(), mode).unwrap();

    // Create a new character style
    let style = MonoTextStyle::new(&FONT_6X10, Rgb888::WHITE);

    // Create a new text
    let text = Text::new("Hello World!", Point { x: 30, y: 100 }, style);

    // Draw the text on the display
    text.draw(&mut display).unwrap();

    // Flush everything
    display.flush();

    // wait 10000000 microseconds (10 seconds)
    boot::stall(10_000_000);

    Status::SUCCESS
}
