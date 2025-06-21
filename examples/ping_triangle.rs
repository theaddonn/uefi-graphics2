#![no_main]
#![no_std]

extern crate alloc;

use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{Rgb888, WebColors};
use embedded_graphics::primitives::{PrimitiveStyle, StyledDrawable, Triangle};
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

    // Create a new triangle
    let triangle = Triangle::new(
        Point { x: 30, y: 100 },
        Point { x: 230, y: 130 },
        Point { x: 110, y: 300 },
    );

    // Draw the text on the display
    triangle
        .draw_styled(
            &mut PrimitiveStyle::with_fill(Rgb888::CSS_PINK),
            &mut display,
        )
        .unwrap();

    // Flush everything
    display.flush();

    // wait 10000000 microseconds (10 seconds)
    boot_services.stall(10_000_000);

    Status::SUCCESS
}
