// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{constant::INIT_RSP, mem, mem::reserved, vram};
use core::ptr;
use uefi::table::boot;
use x86_64::VirtAddr;

#[repr(C)]
pub struct Info {
    entry_addr: VirtAddr,
    vram_info: vram::Info,
    mem_map: mem::Map,
    reserved: reserved::Map,
}

impl Info {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        entry_addr: VirtAddr,
        vram_info: vram::Info,
        mem_map: mem::Map,
        reserved: reserved::Map,
    ) -> Self {
        Self {
            entry_addr,
            vram_info,
            mem_map,
            reserved,
        }
    }

    #[must_use]
    pub fn entry_addr(&self) -> VirtAddr {
        self.entry_addr
    }

    #[must_use]
    pub fn vram(&self) -> vram::Info {
        self.vram_info
    }

    pub fn set(self) {
        unsafe {
            ptr::write(INIT_RSP.as_mut_ptr() as _, self);
        }
    }

    #[must_use]
    pub fn get() -> Self {
        unsafe { ptr::read(INIT_RSP.as_mut_ptr() as _) }
    }

    #[must_use]
    pub fn mem_map(&mut self) -> &mut [boot::MemoryDescriptor] {
        self.mem_map.as_slice()
    }

    #[must_use]
    pub fn reserved(&self) -> &reserved::Map {
        &self.reserved
    }
}
