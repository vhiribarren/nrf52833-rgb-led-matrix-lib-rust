/*
MIT License

Copyright (c) 2022 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */

use core::cmp::min;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const WHITE: Color = Color { r: 1, g: 1, b: 1 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 1 };
    pub const RED: Color = Color { r: 1, g: 0, b: 0 };
    pub const YELLOW: Color = Color { r: 1, g: 1, b: 0 };
    pub const GREEN: Color = Color { r: 0, g: 1, b: 0 };
    pub const CYAN: Color = Color { r: 0, g: 1, b: 1 };
    pub const MAGENTA: Color = Color { r: 1, g: 0, b: 1 };
}

#[derive(Clone)]
pub struct Canvas<const WIDTH: usize = 64, const HEIGHT: usize = 32>(
    pub(crate) [[Color; WIDTH]; HEIGHT],
);

impl<const WIDTH: usize, const HEIGHT: usize> Default for Canvas<WIDTH, HEIGHT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Canvas<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        Canvas([[Color::BLACK; WIDTH]; HEIGHT])
    }
    pub fn draw_rectangle(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: Color,
    ) {
        let y_max = min(y + height, HEIGHT);
        let x_max = min(x + width, WIDTH);
        for y_pos in y..y_max {
            for x_pos in x..x_max {
                self.0[y_pos][x_pos] = color;
            }
        }
    }
}
