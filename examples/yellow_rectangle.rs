#![no_main]
#![no_std]

extern crate alloc;

use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::prelude::Size;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle, StyledDrawable};
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;

use uefi_graphics2::UefiDisplay;

#[entry]
fn main(_image_handle: Handle, mut boot_system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut boot_system_table).unwrap();

    // Disable the watchdog timer
    boot_system_table
        .boot_services()
        .set_watchdog_timer(0, 0x10000, None)
        .unwrap();

    let boot_services = boot_system_table.boot_services();

    // Get gop
    let gop_handle = boot_services
        .get_handle_for_protocol::<GraphicsOutput>()
        .unwrap();
    let mut gop = boot_services
        .open_protocol_exclusive::<GraphicsOutput>(gop_handle)
        .unwrap();

    // Create UefiDisplay
    let mode = gop.current_mode_info();
    let mut display = UefiDisplay::new(gop.frame_buffer(), mode).unwrap();

    // Create a new rectangle
    let rectangle = Rectangle::new(Point { x: 30, y: 100 }, Size { width: 300, height: 150 });

    // Draw the text on the display
    rectangle.draw_styled(&mut PrimitiveStyle::with_fill(Rgb888::YELLOW), &mut display).unwrap();

    // Flush everything
    display.flush();

    // wait 10000000 microseconds (10 seconds)
    boot_services.stall(10_000_000);

    Status::SUCCESS
}
