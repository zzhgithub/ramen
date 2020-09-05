// SPDX-License-Identifier: GPL-3.0-or-later

use super::{RGB, Vram, font, screen};

pub const MOUSE_CURSOR_WIDTH: usize = 16;
pub const MOUSE_CURSOR_HEIGHT: usize = 16;

pub const MOUSE_GRAPHIC: [[char; MOUSE_CURSOR_WIDTH]; MOUSE_CURSOR_HEIGHT] = [
    [
        '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '0', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '0', '0', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '0', '0', '0', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '0', '0', '0', '0', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '0', '0', '0', '0', '*', '*', '*', '*', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '0', '*', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '*', '.', '*', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '*', '.', '.', '*', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '.', '.', '.', '.', '*', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '.', '.', '.', '.', '.', '*', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '.', '.', '.', '.', '.', '.', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
];

#[macro_export]
macro_rules! print_with_pos {
    ($coord:expr,$color:expr,$text:expr,$($args:expr),*) => {
        let mut screen_write =
            crate::graphics::screen::ScreenWrite::new($coord, $color);

        // To narrow the scope of `use core::fmt::Write;`, enclose sentences by curly braces.
        {
            use core::fmt::Write;
            write!(screen_write, $text, $($args,)*).unwrap();
        }
    };
}

pub struct Screen;

impl Screen {
    // TODO: Specify top left coordinate and length, rather than two coordinates.
    pub fn draw_rectangle(
        &mut self,
        color: RGB,
        top_left: Coord<isize>,
        bottom_right: Coord<isize>,
    ) {
        for y in top_left.y..=bottom_right.y {
            for x in top_left.x..=bottom_right.x {
                unsafe {
                    Vram::set_color(Coord::new(x, y), color);
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}

pub type TwoDimensionalVec<T> = Coord<T>;

impl<T> Coord<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: core::ops::Add<Output = T>> core::ops::Add for Coord<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: core::cmp::PartialOrd> Coord<T> {
    pub fn put_in(self, coord_1: Self, coord_2: Self) -> Self {
        let mut new_coord = self;

        if new_coord.x < coord_1.x {
            new_coord.x = coord_1.x;
        }
        if new_coord.x > coord_2.x {
            new_coord.x = coord_2.x;
        }

        if new_coord.y < coord_1.y {
            new_coord.y = coord_1.y;
        }
        if new_coord.y > coord_2.y {
            new_coord.y = coord_2.y;
        }

        new_coord
    }
}

pub struct ScreenWrite {
    coord: Coord<isize>,
    color: RGB,
}

impl ScreenWrite {
    pub fn new(coord: Coord<isize>, color: RGB) -> Self {
        Self { coord, color }
    }
}

impl core::fmt::Write for ScreenWrite {
    fn write_str(&mut self, s: &str) -> core::result::Result<(), core::fmt::Error> {
        print_str(&self.coord, self.color, s);
        self.coord.x += (s.len() * font::FONT_WIDTH) as isize;
        Ok(())
    }
}

pub struct MouseCursor {
    coord: Coord<isize>,
    image: [[RGB; MOUSE_CURSOR_WIDTH]; MOUSE_CURSOR_HEIGHT],
}

impl MouseCursor {
    pub fn new(
        background_color: RGB,
        image: [[char; MOUSE_CURSOR_WIDTH]; MOUSE_CURSOR_HEIGHT],
    ) -> Self {
        let mut colored_dots: [[RGB; MOUSE_CURSOR_WIDTH]; MOUSE_CURSOR_HEIGHT] =
            [[background_color; MOUSE_CURSOR_WIDTH]; MOUSE_CURSOR_WIDTH];

        for y in 0..MOUSE_CURSOR_HEIGHT {
            for x in 0..MOUSE_CURSOR_WIDTH {
                colored_dots[y][x] = match image[y][x] {
                    '*' => RGB::new(0x0000_0000),
                    '0' => RGB::new(0x00FF_FFFF),
                    _ => background_color,
                }
            }
        }

        Self {
            coord: Coord::new(0, 0),
            image: colored_dots,
        }
    }

    pub fn print_coord(&mut self, coord: Coord<isize>) {
        let mut screen = Screen;

        screen.draw_rectangle(
            RGB::new(0x0000_8484),
            Coord::new(16, 32),
            Coord::new(16 + 8 * 12 - 1, 32 + 15),
        );

        print_with_pos!(
            coord,
            RGB::new(0x00FF_FFFF),
            "({}, {})",
            self.coord.x,
            self.coord.y
        );
    }

    pub fn draw_offset(&mut self, offset: TwoDimensionalVec<isize>) {
        let new_coord = self.coord.clone() + offset;
        self.draw(new_coord)
    }

    pub fn draw(&mut self, coord: Coord<isize>) {
        self.remove_previous_cursor();

        let adjusted_coord = coord.put_in(
            Coord::new(0, 0),
            Coord::new(
                (Vram::x_len() - MOUSE_CURSOR_WIDTH - 1) as isize,
                (Vram::y_len() - MOUSE_CURSOR_HEIGHT - 1) as isize,
            ),
        );

        for y in 0..MOUSE_CURSOR_HEIGHT {
            for x in 0..MOUSE_CURSOR_WIDTH {
                unsafe {
                    Vram::set_color(
                        adjusted_coord.clone() + Coord::new(x as isize, y as isize),
                        self.image[y][x],
                    );
                }
            }
        }

        self.coord = adjusted_coord;
    }

    fn remove_previous_cursor(&self) {
        let mut screen = Screen;

        screen.draw_rectangle(
            RGB::new(0x0000_8484),
            Coord::new(self.coord.x, self.coord.y),
            Coord::new(
                self.coord.x + MOUSE_CURSOR_WIDTH as isize,
                self.coord.y + MOUSE_CURSOR_HEIGHT as isize,
            ),
        );
    }
}

#[rustfmt::skip]
pub fn draw_desktop()  {
    let x_len:isize  = Vram::x_len() as isize;
    let y_len:isize  = Vram::y_len() as isize;

    // It seems that changing the arguments as `color, coord_1, coord_2` actually makes the code
    // dirty because by doing it lots of `Coord::new(x1, x2)` appear on below.
    let draw_desktop_part = |color, x0, y0, x1, y1| {
        let mut screen:screen::Screen = Screen;
        screen.draw_rectangle(RGB::new(color), Coord::new(x0, y0), Coord::new(x1, y1));
    };

    draw_desktop_part(0x0000_8484,          0,          0, x_len -  1, y_len - 29);
    draw_desktop_part(0x00C6_C6C6,          0, y_len - 28, x_len -  1, y_len - 28);
    draw_desktop_part(0x00FF_FFFF,          0, y_len - 27, x_len -  1, y_len - 27);
    draw_desktop_part(0x00C6_C6C6,          0, y_len - 26, x_len -  1, y_len -  1);

    draw_desktop_part(0x00FF_FFFF,          3, y_len - 24,         59, y_len - 24);
    draw_desktop_part(0x00FF_FFFF,          2, y_len - 24,          2, y_len -  4);
    draw_desktop_part(0x0084_8484,          3, y_len -  4,         59, y_len -  4);
    draw_desktop_part(0x0084_8484,         59, y_len - 23,         59, y_len -  5);
    draw_desktop_part(0x0000_0000,          2, y_len -  3,         59, y_len -  3);
    draw_desktop_part(0x0000_0000,         60, y_len - 24,         60, y_len -  3);

    draw_desktop_part(0x0084_8484, x_len - 47, y_len - 24, x_len -  4, y_len - 24);
    draw_desktop_part(0x0084_8484, x_len - 47, y_len - 23, x_len - 47, y_len -  4);
    draw_desktop_part(0x00FF_FFFF, x_len - 47, y_len -  3, x_len -  4, y_len -  3);
    draw_desktop_part(0x00FF_FFFF, x_len -  3, y_len - 24, x_len -  3, y_len -  3);
}

fn print_str(coord: &Coord<isize>, color: RGB, str: &str) {
    let mut char_x_pos = coord.x;
    for c in str.chars() {
        print_char(
            Coord::new(char_x_pos, coord.y),
            color,
            font::FONTS[c as usize],
        );
        char_x_pos += font::FONT_WIDTH as isize;
    }
}

fn print_char(
    coord: Coord<isize>,
    color: RGB,
    font: [[bool; font::FONT_WIDTH]; font::FONT_HEIGHT],
) {
    for (i, line) in font.iter().enumerate().take(font::FONT_HEIGHT) {
        for (j, cell) in line.iter().enumerate().take(font::FONT_WIDTH) {
            if *cell {
                unsafe {
                    Vram::set_color(coord.clone() + Coord::new(j as isize, i as isize), color);
                }
            }
        }
    }
}
