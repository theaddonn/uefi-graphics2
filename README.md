# Uefi-graphics2

![Crates.io Version](https://img.shields.io/crates/v/uefi-graphics2)
![Crates.io Total Downloads](https://img.shields.io/crates/d/uefi-graphics2)
![Crates.io License](https://img.shields.io/crates/l/uefi-graphics2)

A blazingly fast embedded-graphics display driver for UEFI environments,
using the [`embedded-graphics`](https://crates.io/crates/embedded-graphics) crate
as its base.

Supports:

- [X] Double buffering
- [X] Display resizing
- [X] An extensive draw/render library using the
  [`embedded-graphics`](https://crates.io/crates/embedded-graphics) crate

### Why are there 2 other crates for this job?

[`uefi-graphics`](https://crates.io/crates/uefi-graphics) and
[`uefi-graphics-driver`](https://crates.io/crates/uefi-graphics-driver)
are 2 crates providing similar purpose,
sadly both seem to either lack functionality or are unmaintained

## Example

Here is a simple example with utilising the [`uefi`](https://crates.io/crates/uefi) crate on version `0.28.0`:

```rust
#![no_main]
#![no_std]

extern crate alloc;

use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;

use uefi_graphics2::embedded_graphics::pixelcolor::{Rgb888, RgbColor};
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
    let gop_handle = boot_services.get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop = boot_services.open_protocol_exclusive::<GraphicsOutput>(gop_handle).unwrap();

    // Create UefiDisplay
    let mode = gop.current_mode_info();
    let mut display = UefiDisplay::new(gop.frame_buffer(), mode);

    // Tint the entire screen cyan
    display.fill_entire(Rgb888::CYAN).unwrap();

    // Draw everything
    display.flush();

    // wait 10000000 microseconds (10 seconds)
    boot_services.stall(10_000_000);

    Status::SUCCESS
}
```

## Contributing and Updating

If any dependencies are outdated, bugs occur or features are requested,
please notify me and create an issue.

Contributing is appreciated as well, feel free to create a pull request.
