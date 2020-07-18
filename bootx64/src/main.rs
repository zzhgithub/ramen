#![no_std]
#![feature(lang_items, start)]
#![no_main]

#[macro_use]
extern crate log;

#[macro_use]
extern crate alloc;

extern crate uefi;
extern crate uefi_services;

mod fs;
mod gop;

use uefi::prelude::{Boot, Handle, Status, SystemTable};
use uefi::ResultExt;

fn reset_console(system_table: &SystemTable<Boot>) -> () {
    system_table
        .stdout()
        .reset(false)
        .expect_success("Failed to reset stdout");
}

/// Initialize uefi-rs services. This includes initialization of GlobalAlloc, which enables us to
/// use Collections defined in alloc module, such as Vec and LinkedList.
fn initialize_uefi_utilities(system_table: &SystemTable<Boot>) -> () {
    uefi_services::init(&system_table).expect_success("Failed to initialize_uefi_utilities");
}

fn initialize(system_table: &SystemTable<Boot>) -> () {
    initialize_uefi_utilities(&system_table);
    reset_console(&system_table);
    info!("Hello World!");
}

#[start]
#[no_mangle]
pub fn efi_main(image: Handle, system_table: SystemTable<Boot>) -> Status {
    initialize(&system_table);
    gop::init(&system_table);
    info!("GOP set.");
    fs::place_binary_files(&system_table);
    loop {}
}
