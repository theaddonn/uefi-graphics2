# uefi-graphics2

[![Crates.io Version](https://img.shields.io/crates/v/uefi-graphics2)](https://crates.io/crates/uefi-graphics2)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/uefi-graphics2)](https://crates.io/crates/uefi-graphics2)
[![Crates.io License](https://img.shields.io/crates/l/uefi-graphics2)](https://github.com/theaddonn/uefi-graphics2/blob/main/LICENSE)

A fast embedded-graphics display driver for UEFI environments,
using the [`embedded-graphics`](https://crates.io/crates/embedded-graphics) crate as its base.

Supports:

- [X] Double buffering
- [X] Display resizing
- [X] An extensive draw/render library using the [`embedded-graphics`](https://crates.io/crates/embedded-graphics) crate

### Why are there 2 other crates for this job?

[`uefi-graphics`](https://crates.io/crates/uefi-graphics) and [`uefi-graphics-driver`](https://crates.io/crates/uefi-graphics-driver)
are two crates providing similar functionality,
sadly both seem to either lack some of the necessary functionality or are completely unmaintained

## Example

Here is a simple example with using the [`uefi`](https://crates.io/crates/uefi) crate on version `0.30.0`:

```rust
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
```

More example can be found in the example
directory [`uefi-graphics2/examples`](https://github.com/theaddonn/uefi-graphics2/tree/main/examples).

## Contributing and Updating

If any dependencies are outdated, bugs occur or features are requested,
please notify me and create an issue.

Contributing is appreciated as well, feel free to create a pull request.

If you like this project dont forget to leave a star on github!

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=theaddonn/uefi-graphics2&type=Date)](https://star-history.com/#theaddonn/uefi-graphics2&Date)
