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

use core::{cmp::min, ops::Mul};

use crate::fonts::Font;

pub type Stencil5x7 = Stencil<5, 7>;
pub type Stencil8x16 = Stencil<8, 16>;

pub struct Stencil<const W: usize, const H: usize>(pub [[u8; W]; H]);

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "logging", derive(Debug))]
pub struct Color([u8; 3]);

impl Color {
    pub const WHITE: Color = Color::new(255, 255, 255);
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const BLUE: Color = Color::new(0, 0, 255);
    pub const RED: Color = Color::new(255, 0, 0);
    pub const YELLOW: Color = Color::new(255, 255, 0);
    pub const GREEN: Color = Color::new(0, 255, 0);
    pub const CYAN: Color = Color::new(0, 255, 255);
    pub const MAGENTA: Color = Color::new(255, 0, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color([r, g, b])
    }

    pub fn r(&self) -> u8 {
        self.0[0]
    }

    pub fn g(&self) -> u8 {
        self.0[1]
    }

    pub fn b(&self) -> u8 {
        self.0[2]
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color::new(
            (rhs * self.r() as f32) as u8,
            (rhs * self.g() as f32) as u8,
            (rhs * self.b() as f32) as u8,
        )
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(
            (self * rhs.r() as f32) as u8,
            (self * rhs.g() as f32) as u8,
            (self * rhs.b() as f32) as u8,
        )
    }
}

impl IntoIterator for Color {
    type Item = u8;

    type IntoIter = core::array::IntoIter<u8, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct TextOptions {
    pub interspace: usize,
    pub color: Color,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            interspace: 1,
            color: Color::WHITE,
        }
    }
}

#[derive(Default)]
pub enum BlendMode {
    #[default]
    TransparentBlack,
    Replace,
}

#[derive(Clone)]
pub struct Canvas<const WIDTH: usize, const HEIGHT: usize>(pub(crate) [[Color; WIDTH]; HEIGHT]);

impl Canvas<64, 32> {
    pub const fn with_64x32() -> Self {
        Canvas::<64, 32>::new()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for Canvas<WIDTH, HEIGHT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Canvas<WIDTH, HEIGHT> {
    pub const fn new() -> Self {
        Canvas([[Color::BLACK; WIDTH]; HEIGHT])
    }
    pub fn width(&self) -> usize {
        WIDTH
    }
    pub fn height(&self) -> usize {
        HEIGHT
    }
    pub fn with_background_color(color: Color) -> Self {
        Canvas([[color; WIDTH]; HEIGHT])
    }
    pub fn clear_with_color(&mut self, color: Color) -> &mut Self {
        for line in 0..HEIGHT {
            for col in 0..WIDTH {
                self.0[line][col] = color;
            }
        }
        self
    }
    pub fn clear(&mut self) -> &mut Self {
        self.clear_with_color(Color::BLACK)
    }
    pub fn draw_rectangle(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: Color,
    ) -> &mut Self {
        let y_max = min(y + height, HEIGHT);
        let x_max = min(x + width, WIDTH);
        for y_pos in y..y_max {
            for x_pos in x..x_max {
                self.0[y_pos][x_pos] = color;
            }
        }
        self
    }
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) -> &mut Self {
        if x < WIDTH && y < HEIGHT {
            self.0[y][x] = color;
        }
        self
    }
    pub fn draw_canvas<const W: usize, const H: usize>(
        &mut self,
        x: usize,
        y: usize,
        canvas: &Canvas<W, H>,
        blend_mode: BlendMode,
    ) -> &mut Self {
        let canvas_array = canvas.as_ref();
        let y_max = min(y + H, HEIGHT);
        let x_max = min(x + W, WIDTH);
        for (model_y_pos, canvas_y_pos) in (y..y_max).enumerate() {
            for (model_x_pos, canvas_x_pos) in (x..x_max).enumerate() {
                let point_color = canvas_array[model_y_pos][model_x_pos];
                match blend_mode {
                    BlendMode::TransparentBlack => match point_color {
                        val if val == Color::BLACK => continue,
                        _ => self.0[canvas_y_pos][canvas_x_pos] = point_color,
                    },
                    BlendMode::Replace => self.0[canvas_y_pos][canvas_x_pos] = point_color,
                }
            }
        }
        self
    }
    pub fn draw_stencil<const W: usize, const H: usize>(
        &mut self,
        x: usize,
        y: usize,
        model: &Stencil<W, H>,
        color: Color,
    ) -> &mut Self {
        let y_max = min(y + H, HEIGHT);
        let x_max = min(x + W, WIDTH);
        for (model_y_pos, canvas_y_pos) in (y..y_max).enumerate() {
            for (model_x_pos, canvas_x_pos) in (x..x_max).enumerate() {
                match model.0[model_y_pos][model_x_pos] {
                    val if val == 0 => continue,
                    _ => self.0[canvas_y_pos][canvas_x_pos] = color,
                }
            }
        }
        self
    }
    pub fn draw_text<const W: usize, const H: usize>(
        &mut self,
        x: usize,
        y: usize,
        text: &str,
        font: impl Font<W, H>,
        opts: TextOptions,
    ) -> &mut Self {
        for (idx, c) in text.chars().enumerate() {
            let stencil = font.stencil_for(c);
            let stencil_width = stencil.0[0].len();
            self.draw_stencil(
                x + idx * (stencil_width + opts.interspace),
                y,
                stencil,
                opts.color,
            );
        }
        self
    }

    pub fn draw_number<const W: usize, const H: usize>(
        &mut self,
        x: usize,
        y: usize,
        number: u32,
        font: impl Font<W, H>,
        opts: TextOptions,
    ) -> &mut Self {
        let mut digit_nb = 1;
        let mut remain = number;
        while remain > 9 {
            digit_nb += 1;
            remain = remain / 10;
        }
        let digit_nb = digit_nb;
        let mut to_draw = number;
        for idx in 0..digit_nb {
            let divisor = 10_u32.pow(digit_nb - idx - 1);
            let extracted_digit = to_draw / divisor;
            let extracted_digit_char = char::from_digit(extracted_digit, 10).unwrap();
            to_draw = to_draw - extracted_digit * divisor;
            let stencil = font.stencil_for(extracted_digit_char);
            let stencil_width = stencil.0[0].len();
            self.draw_stencil(
                x + (idx as usize) * (stencil_width + opts.interspace),
                y,
                stencil,
                opts.color,
            );
        }
        self
    }

    pub fn draw_char<const W: usize, const H: usize>(
        &mut self,
        x: usize,
        y: usize,
        c: char,
        color: Color,
        font: impl Font<W, H>,
    ) -> &mut Self {
        let stencil = font.stencil_for(c);
        self.draw_stencil(x, y, stencil, color);
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
