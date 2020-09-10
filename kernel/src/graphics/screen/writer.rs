// SPDX-License-Identifier: GPL-3.0-or-later

use super::{font, Vram};
use core::convert::TryFrom;
use rgb::RGB8;
use vek::Vec2;

pub struct Writer {
    coord: Vec2<isize>,
    color: RGB8,
}

impl Writer {
    pub const fn new(coord: Vec2<isize>, color: RGB8) -> Self {
        Self { coord, color }
    }

    fn print_str(&mut self, str: &str) {
        for c in str.chars() {
            if c == '\n' {
                self.coord.x = 0;
                self.coord.y += isize::try_from(font::FONT_HEIGHT).unwrap();
                continue;
            }

            print_char(&self.coord, self.color, font::FONTS[c as usize]);
            self.coord.x += isize::try_from(font::FONT_WIDTH).unwrap();

            if self.coord.x + isize::try_from(font::FONT_WIDTH).unwrap()
                >= isize::try_from(Vram::resolution().x).unwrap()
            {
                self.coord.x = 0;
                self.coord.y += isize::try_from(font::FONT_HEIGHT).unwrap();
            }
        }
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.print_str(s);
        Ok(())
    }
}

fn print_char(
    coord: &Vec2<isize>,
    color: RGB8,
    font: [[bool; font::FONT_WIDTH]; font::FONT_HEIGHT],
) {
    for (i, line) in font.iter().enumerate().take(font::FONT_HEIGHT) {
        for (j, cell) in line.iter().enumerate().take(font::FONT_WIDTH) {
            if *cell {
                unsafe {
                    Vram::set_color(
                        &(coord.clone()
                            + Vec2::new(isize::try_from(j).unwrap(), isize::try_from(i).unwrap())),
                        color,
                    );
                }
            }
        }
    }
}
