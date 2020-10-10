// SPDX-License-Identifier: GPL-3.0-or-later

pub mod font;

#[macro_use]
pub mod screen;

use {
    common::{constant::VRAM_ADDR, kernelboot},
    conquer_once::spin::{Lazy, OnceCell},
    core::{
        fmt,
        ops::{Index, IndexMut},
        ptr,
    },
    rgb::RGB8,
    vek::Vec2,
    x86_64::VirtAddr,
};

static VRAM: Lazy<OnceCell<Vram>> = Lazy::new(OnceCell::uninit);

#[derive(Clone)]
pub struct Vram {
    bits_per_pixel: i32,
    resolution: Vec2<i32>,
    ptr: VirtAddr,
}

impl Vram {
    pub fn init(boot_info: &kernelboot::Info) {
        VRAM.try_init_once(|| Self::new(boot_info)).unwrap();
    }

    pub fn resolution() -> &'static Vec2<i32> {
        &Vram::get().resolution
    }

    pub fn display() -> impl core::fmt::Display {
        Self::get()
    }

    pub fn bpp() -> i32 {
        Vram::get().bits_per_pixel
    }

    pub fn ptr() -> VirtAddr {
        Vram::get().ptr
    }

    pub fn set_color(coord: Vec2<i32>, rgb: RGB8) {
        let vram = Self::get();

        if coord.cmplt(&Vec2::zero()).iter().any(|x| *x)
            || coord
                .cmpgt(&(vram.resolution - Vec2::one()))
                .iter()
                .any(|x| *x)
        {
            panic!("Tried to draw out of screen: {}", coord);
        }

        let offset_from_base = (coord.y * Vram::resolution().x + coord.x) * vram.bits_per_pixel / 8;

        unsafe {
            let ptr = vram.ptr.as_mut_ptr::<u8>().offset(offset_from_base as _);

            ptr::write(ptr.offset(0), rgb.b);
            ptr::write(ptr.offset(1), rgb.g);
            ptr::write(ptr.offset(2), rgb.r);
        }
    }

    fn new(boot_info: &kernelboot::Info) -> Self {
        let vram = boot_info.vram();

        let (x_len, y_len) = vram.resolution();
        let resolution = Vec2::new(x_len, y_len);

        Self {
            bits_per_pixel: vram.bpp(),
            resolution,
            ptr: VRAM_ADDR,
        }
    }

    fn get() -> &'static Vram {
        VRAM.try_get().expect("VRAM not initialized")
    }
}

impl fmt::Display for Vram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}bpp Resolution: {}x{}",
            self.bits_per_pixel, self.resolution.x, self.resolution.y
        )
    }
}
