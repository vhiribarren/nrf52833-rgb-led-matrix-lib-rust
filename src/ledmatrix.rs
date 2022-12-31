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

use crate::canvas::Canvas;
use microbit::hal::gpio::{Level, Output, Pin, PushPull};
use microbit::hal::prelude::*;

pub struct LedMatrixPins64x32<MODE> {
    pub r1: Pin<MODE>,
    pub g1: Pin<MODE>,
    pub b1: Pin<MODE>,
    pub r2: Pin<MODE>,
    pub g2: Pin<MODE>,
    pub b2: Pin<MODE>,
    pub a: Pin<MODE>,
    pub b: Pin<MODE>,
    pub c: Pin<MODE>,
    pub d: Pin<MODE>,
    pub clk: Pin<MODE>,
    pub lat: Pin<MODE>,
    pub oe: Pin<MODE>,
}

pub struct LedMatrix<const LINES: usize = 4, const WIDTH: usize = 64, const HEIGHT: usize = 32> {
    pub pin_r1: Pin<Output<PushPull>>,
    pub pin_g1: Pin<Output<PushPull>>,
    pub pin_b1: Pin<Output<PushPull>>,
    pub pin_r2: Pin<Output<PushPull>>,
    pub pin_g2: Pin<Output<PushPull>>,
    pub pin_b2: Pin<Output<PushPull>>,
    pin_clk: Pin<Output<PushPull>>,
    pin_lat: Pin<Output<PushPull>>,
    pin_oe: Pin<Output<PushPull>>,
    line_ctrl: [Pin<Output<PushPull>>; LINES],
}

impl LedMatrix {
    pub fn new<MODE>(pins: LedMatrixPins64x32<MODE>) -> LedMatrix {
        LedMatrix {
            pin_r1: pins.r1.into_push_pull_output(Level::Low),
            pin_g1: pins.g1.into_push_pull_output(Level::Low),
            pin_b1: pins.b1.into_push_pull_output(Level::Low),
            pin_r2: pins.r2.into_push_pull_output(Level::Low),
            pin_g2: pins.g2.into_push_pull_output(Level::Low),
            pin_b2: pins.b2.into_push_pull_output(Level::Low),
            pin_clk: pins.clk.into_push_pull_output(Level::Low),
            pin_lat: pins.lat.into_push_pull_output(Level::Low),
            pin_oe: pins.oe.into_push_pull_output(Level::Low),
            line_ctrl: [
                pins.a.into_push_pull_output(Level::Low),
                pins.b.into_push_pull_output(Level::Low),
                pins.c.into_push_pull_output(Level::Low),
                pins.d.into_push_pull_output(Level::Low),
            ],
        }
    }
}

impl<const LINES: usize, const WIDTH: usize, const HEIGHT: usize> LedMatrix<LINES, WIDTH, HEIGHT> {
    pub fn draw_canvas(&mut self, canvas: &Canvas<WIDTH, HEIGHT>) {
        let half_height = HEIGHT / 2;
        let raw_canvas = canvas.as_ref();
        for line_index in 0..half_height {
            for col_index in 0..WIDTH {
                let color_down = &raw_canvas[line_index][col_index];
                let color_up = &raw_canvas[line_index + half_height][col_index];
                if color_down.r > 0 {
                    self.pin_r1.set_high().unwrap();
                } else {
                    self.pin_r1.set_low().unwrap();
                }
                if color_up.r > 0 {
                    self.pin_r2.set_high().unwrap();
                } else {
                    self.pin_r2.set_low().unwrap();
                }

                if color_down.g > 0 {
                    self.pin_g1.set_high().unwrap();
                } else {
                    self.pin_g1.set_low().unwrap();
                }
                if color_up.g > 0 {
                    self.pin_g2.set_high().unwrap();
                } else {
                    self.pin_g2.set_low().unwrap();
                }

                if color_down.b > 0 {
                    self.pin_b1.set_high().unwrap();
                } else {
                    self.pin_b1.set_low().unwrap();
                }
                if color_up.b > 0 {
                    self.pin_b2.set_high().unwrap();
                } else {
                    self.pin_b2.set_low().unwrap();
                }
                self.clock_color();
            }
            self.latch_to_line(line_index as u8);
        }
    }

    pub fn latch_to_line(&mut self, line: u8) {
        self.pin_oe.set_high().unwrap();
        let mline = line % 2_u8.pow(LINES as u32);
        for pin_idx in 0..self.line_ctrl.len() {
            let enable_pin = (mline & (1 << pin_idx)) != 0;
            self.line_ctrl[pin_idx]
                .set_state(PinState::from(enable_pin))
                .unwrap();
        }
        self.pin_lat.set_high().unwrap();
        self.pin_lat.set_low().unwrap();
        self.pin_oe.set_low().unwrap();
    }
    pub fn clock_color(&mut self) {
        self.pin_clk.set_high().unwrap();
        self.pin_clk.set_low().unwrap();
    }
}
