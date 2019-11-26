use crate::asm;
use crate::graphics;

const PIC0_ICW1: i32 = 0x0020;
const PIC0_OCW2: i32 = 0x0020;
const PIC0_IMR: i32 = 0x0021;
const PIC0_ICW2: i32 = 0x0021;
const PIC0_ICW3: i32 = 0x0021;
const PIC0_ICW4: i32 = 0x0021;
const PIC1_ICW1: i32 = 0x00a0;
const _PIC1_OCW2: i32 = 0x00a0;
const PIC1_IMR: i32 = 0x00a1;
const PIC1_ICW2: i32 = 0x00a1;
const PIC1_ICW3: i32 = 0x00a1;
const PIC1_ICW4: i32 = 0x00a1;

const PORT_KEYDATA: i32 = 0x0060;

// See P.128.
pub fn init_pic() -> () {
    asm::out8(PIC0_IMR, 0xff);
    asm::out8(PIC1_IMR, 0xff);

    asm::out8(PIC0_ICW1, 0x11);
    asm::out8(PIC0_ICW2, 0x20);
    asm::out8(PIC0_ICW3, 1 << 2);
    asm::out8(PIC0_ICW4, 0x01);

    asm::out8(PIC1_ICW1, 0x11);
    asm::out8(PIC1_ICW2, 0x28);
    asm::out8(PIC1_ICW3, 2);
    asm::out8(PIC1_ICW4, 0x01);

    asm::out8(PIC0_IMR, 0xfb);
    asm::out8(PIC1_IMR, 0xff);
}

pub fn enable_pic1_keyboard_mouse() -> () {
    asm::out8(PIC0_IMR, 0xf9);
    asm::out8(PIC1_IMR, 0xef);
}

pub fn interrupt_handler_21() -> () {
    asm::out8(PIC0_OCW2, 0x61);

    use crate::print_with_pos;
    graphics::screen::draw_rectangle(
        &graphics::Vram::new(),
        graphics::Vram::new().x_len as isize,
        graphics::ColorIndex::Rgb008484,
        graphics::screen::Coord::new(0, 16),
        graphics::screen::Coord::new(15, 31),
    );

    print_with_pos!(
        graphics::screen::Coord::new(0, 16),
        graphics::ColorIndex::RgbFFFFFF,
        "{}",
        asm::in8(PORT_KEYDATA)
    );

    loop {
        asm::hlt()
    }
}

pub fn interrupt_handler_2c() -> () {
    use crate::print_with_pos;
    graphics::screen::draw_rectangle(
        &graphics::Vram::new(),
        graphics::Vram::new().x_len as isize,
        graphics::ColorIndex::Rgb000000,
        graphics::screen::Coord::new(0, 0),
        graphics::screen::Coord::new(32 * 8 - 1, 15),
    );

    print_with_pos!(
        graphics::screen::Coord::new(0, 0),
        graphics::ColorIndex::RgbFFFFFF,
        "INT 2C (IRQ-12) : PS/2 mouse",
    );

    loop {
        asm::hlt();
    }
}
