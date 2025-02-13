#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::ptr::copy;

// for re-export
pub use embedded_graphics;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{OriginDimensions, Size};
use embedded_graphics::pixelcolor::{IntoStorage, Rgb888, RgbColor};
use embedded_graphics::prelude::Point;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::Pixel;
use uefi::proto::console::gop::{FrameBuffer, ModeInfo};

pub use crate::error::UefiDisplayError;

pub mod error;

#[derive(Debug)]
pub struct UefiDisplay {
    frame_buffer: *mut u8,
    double_buffer: *mut u8,
    stride: u32,
    size: (u32, u32),
    // width * height * 4 (4 = red + green + blue + reserved)
    buffer_size: u64,
}

impl UefiDisplay {
    pub fn new(
        mut frame_buffer: FrameBuffer,
        mode_info: ModeInfo,
    ) -> Result<Self, UefiDisplayError> {
        let mut display = Self {
            frame_buffer: frame_buffer.as_mut_ptr(),
            double_buffer: Vec::with_capacity(
                mode_info.resolution().0 * mode_info.resolution().1 * 4,
            )
            .as_mut_ptr(),
            stride: mode_info.stride() as u32,
            size: (
                mode_info.resolution().0 as u32,
                mode_info.resolution().1 as u32,
            ),
            buffer_size: (mode_info.resolution().0 * mode_info.resolution().1 * 4) as u64,
        };

        match display.fill_entire(Rgb888::BLACK) {
            Ok(_) => Ok(display),
            Err(e) => Err(e),
        }
    }

    /// # Safety
    ///
    /// In [UefiDisplay::new] the screen gets set to a default color (Black).
    /// But [UefiDisplay::new_unsafe] doesn't do that,
    /// this may lead to a corrupted screen with a default color
    /// (Tests show that the screen may be initialized with a grayish color).
    /// This can easily be fixed by just using
    /// [UefiDisplay::new].
    pub unsafe fn new_unsafe(mut frame_buffer: FrameBuffer, mode_info: ModeInfo) -> Self {
        Self {
            frame_buffer: frame_buffer.as_mut_ptr(),
            double_buffer: Vec::with_capacity(
                mode_info.resolution().0 * mode_info.resolution().1 * 4,
            )
            .as_mut_ptr(),
            stride: mode_info.stride() as u32,
            size: (
                mode_info.resolution().0 as u32,
                mode_info.resolution().1 as u32,
            ),
            buffer_size: (mode_info.resolution().0 * mode_info.resolution().1 * 4) as u64,
        }
    }

    /// Fills the entire screen with a given color.
    pub fn fill_entire(&mut self, color: Rgb888) -> Result<(), UefiDisplayError> {
        self.fill_solid(
            &Rectangle {
                top_left: Point { x: 0, y: 0 },
                size: Size {
                    width: self.size.0,
                    height: self.size.1,
                },
            },
            color,
        )
    }

    /// Copies the framebuffer into the uefi framebuffer.
    /// This function is needed to draw everything.
    pub fn flush(&mut self) {
        unsafe {
            copy(
                self.double_buffer,
                self.frame_buffer,
                self.buffer_size as usize,
            )
        }
    }
}

impl OriginDimensions for UefiDisplay {
    fn size(&self) -> Size {
        Size::from(self.size)
    }
}

impl DrawTarget for UefiDisplay {
    type Color = Rgb888;
    type Error = UefiDisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let pixels = pixels.into_iter();

        for Pixel(point, color) in pixels {
            let bytes: u32 = color.into_storage();
            let stride: u64 = self.stride as u64;
            let (x, y): (u64, u64) = (point.x as u64, point.y as u64);

            let index: u64 = y
                .overflowing_mul(stride)
                .0
                .overflowing_add(x)
                .0
                .overflowing_mul(4)
                .0;

            unsafe { (self.double_buffer.add(index as usize) as *mut u32).write_volatile(bytes) };
        }

        Ok(())
    }
}
