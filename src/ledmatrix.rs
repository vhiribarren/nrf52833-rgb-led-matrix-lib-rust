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

use crate::canvas::Canvas;
use crate::MatrixTimer;
use microbit::hal::gpio::{Level, Output, Pin, PushPull};
use microbit::hal::timer::Instance;
use microbit::hal::{prelude::*, Timer};

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

const RGB_COUNT: usize = 3;

pub struct LedMatrix<
    const LINECTRL_PIN_COUNT: usize = 4,
    const WIDTH: usize = 64,
    const HEIGHT: usize = 32,
> {
    top_colors: [Pin<Output<PushPull>>; RGB_COUNT],
    bottom_colors: [Pin<Output<PushPull>>; RGB_COUNT],
    pin_clk: Pin<Output<PushPull>>,
    pin_lat: Pin<Output<PushPull>>,
    pin_oe: Pin<Output<PushPull>>,
    line_ctrl: [Pin<Output<PushPull>>; LINECTRL_PIN_COUNT],
}

impl LedMatrix<4, 64, 32> {
    pub fn new<MODE>(pins: LedMatrixPins64x32<MODE>) -> Self {
        LedMatrix {
            top_colors: [
                pins.r1.into_push_pull_output(Level::Low),
                pins.g1.into_push_pull_output(Level::Low),
                pins.b1.into_push_pull_output(Level::Low),
            ],
            bottom_colors: [
                pins.r2.into_push_pull_output(Level::Low),
                pins.g2.into_push_pull_output(Level::Low),
                pins.b2.into_push_pull_output(Level::Low),
            ],
            line_ctrl: [
                pins.a.into_push_pull_output(Level::Low),
                pins.b.into_push_pull_output(Level::Low),
                pins.c.into_push_pull_output(Level::Low),
                pins.d.into_push_pull_output(Level::Low),
            ],
            pin_clk: pins.clk.into_push_pull_output(Level::Low),
            pin_lat: pins.lat.into_push_pull_output(Level::Low),
            pin_oe: pins.oe.into_push_pull_output(Level::High),
        }
    }
}

impl<const LINECTRL_PIN_COUNT: usize, const WIDTH: usize, const HEIGHT: usize>
    LedMatrix<LINECTRL_PIN_COUNT, WIDTH, HEIGHT>
{
    pub fn draw_canvas_line(&mut self, canvas: &Canvas<WIDTH, HEIGHT>, line: usize) {
        let half_height = HEIGHT / 2;
        let raw_canvas = canvas.as_ref();
        let line_index = line;
        for col_index in 0..WIDTH {
            let color_top = &raw_canvas[line_index][col_index];
            let color_bottom = &raw_canvas[line_index + half_height][col_index];
            let color_chain = color_top.into_iter().chain(color_bottom.into_iter());
            let pin_chain = self
                .top_colors
                .iter_mut()
                .chain(self.bottom_colors.iter_mut());
            for (pin, color) in pin_chain.zip(color_chain) {
                if color > 0 {
                    pin.set_high().unwrap();
                } else {
                    pin.set_low().unwrap();
                }
            }
            self.clock_color();
        }
        self.latch_to_line(line);
    }

    pub fn draw_canvas(&mut self, canvas: &Canvas<WIDTH, HEIGHT>) {
        // Here, the usage of the TIMER0 is completely fake, it is just to have the right type when using None
        // Is it possible to have something less far-fetched?
        // Implmenting a dummy struct to reference its type seems not possible since microbit::hal::timer::Instance is a sealed trait.
        self.draw_canvas_with_delay_buffer(canvas, None::<&mut Timer<MatrixTimer>>);
    }

    pub fn draw_canvas_with_delay_buffer<TIMER>(
        &mut self,
        canvas: &Canvas<WIDTH, HEIGHT>,
        mut timer: Option<&mut Timer<TIMER>>,
    ) where
        TIMER: Instance,
    {
        let half_height = HEIGHT / 2;
        let mut line_time_avg = 0_f32;
        for line_index in 0..half_height {
            if let Some(unwrapped_timer) = &mut timer {
                unwrapped_timer.start(u32::MAX);
            }
            self.draw_canvas_line(canvas, line_index);
            if let Some(unwrapped_timer) = &mut timer {
                let counter_delta = unwrapped_timer.read();
                line_time_avg = line_time_avg * (line_index as f32 / (line_index + 1) as f32)
                    + counter_delta as f32 / ((line_index + 1) as f32);
            }
        }
        // Wait one line cycle, and simulate a end of latch_to_line
        if let Some(unwrapped_timer) = &mut timer {
            unwrapped_timer.delay_us(line_time_avg as u32);
            self.pin_oe.set_high().unwrap();
        }

        // TODO: should print that only every n times, and should avoid the cfg marco
        //log!("Mean value {}", line_time_avg as u32);
    }

    fn latch_to_line(&mut self, line: usize) {
        self.pin_oe.set_high().unwrap();
        let mline = line % 2_usize.pow(LINECTRL_PIN_COUNT as u32);
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

    fn clock_color(&mut self) {
        self.pin_clk.set_high().unwrap();
        self.pin_clk.set_low().unwrap();
    }
}
