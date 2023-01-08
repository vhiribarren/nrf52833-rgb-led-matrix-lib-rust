/*
MIT License

Copyright (c) 2022, 2023 Vincent Hiribarren

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

#[derive(Clone, Copy, PartialEq)]
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
pub struct Canvas<const WIDTH: usize = 64, const HEIGHT: usize = 32>([[Color; WIDTH]; HEIGHT]);

impl Canvas<64, 32> {
    pub fn with_64x32() -> Self {
        Canvas::<64, 32>::new()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for Canvas<WIDTH, HEIGHT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Canvas<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        Canvas([[Color::BLACK; WIDTH]; HEIGHT])
    }
    pub fn with_background_color(color: Color) -> Self {
        Canvas([[color; WIDTH]; HEIGHT])
    }
    pub fn clear_with_color(&mut self, color: Color) -> &mut Canvas<WIDTH, HEIGHT> {
        for line in 0..HEIGHT {
            for col in 0..WIDTH {
                self.0[line][col] = color;
            }
        }
        self
    }
    pub fn draw_rectangle(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: Color,
    ) -> &mut Canvas<WIDTH, HEIGHT> {
        let y_max = min(y + height, HEIGHT);
        let x_max = min(x + width, WIDTH);
        for y_pos in y..y_max {
            for x_pos in x..x_max {
                self.0[y_pos][x_pos] = color;
            }
        }
        self
    }
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) -> &mut Canvas<WIDTH, HEIGHT> {
        if x < WIDTH && y < HEIGHT {
            self.0[y][x] = color;
        }
        self
    }
    pub fn draw_stencil<const W: usize, const H: usize>(
        &mut self,
        x: usize,
        y: usize,
        model: &[[u8; W]; H],
        color: Color,
    ) -> &mut Canvas<WIDTH, HEIGHT> {
        let y_max = min(y + H, HEIGHT);
        let x_max = min(x + W, WIDTH);
        for (model_y_pos, canvas_y_pos) in (y..y_max).enumerate() {
            for (model_x_pos, canvas_x_pos) in (x..x_max).enumerate() {
                match model[model_y_pos][model_x_pos] {
                    val if val == 0 => continue,
                    _ => self.0[canvas_y_pos][canvas_x_pos] = color,
                }
            }
        }
        self
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> AsRef<[[Color; WIDTH]; HEIGHT]>
    for Canvas<WIDTH, HEIGHT>
{
    fn as_ref(&self) -> &[[Color; WIDTH]; HEIGHT] {
        &self.0
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> AsMut<[[Color; WIDTH]; HEIGHT]>
    for Canvas<WIDTH, HEIGHT>
{
    fn as_mut(&mut self) -> &mut [[Color; WIDTH]; HEIGHT] {
        &mut self.0
    }
}
