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

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use nrf52833_rgb_led_matrix::{
    canvas::Color, init_scheduled_led_matrix_system, register_panic_handler_with_logging,
};

#[entry]
fn main() -> ! {
    register_panic_handler_with_logging!();
    let peripherals = nrf52833_hal::pac::Peripherals::take().unwrap();
    let scheduled_led_matrix = init_scheduled_led_matrix_system!(peripherals);

    cortex_m::interrupt::free(|cs| {
        let mut borrowed_scheduled_led_matrix = scheduled_led_matrix.borrow(cs).borrow_mut();
        let led_matrix = borrowed_scheduled_led_matrix.as_mut().unwrap();
        let canvas = led_matrix.borrow_mut_canvas();
        let w = canvas.width();
        let h = canvas.height();
        let canvas_array = canvas.as_mut();

        let color_segment = w / 6;
        let ramp_color = |pos: usize| -> u8 { (255 * pos / color_segment) as u8 };
        for y in 0..h {
            for x in 0..w {
                canvas_array[y][x] = match x {
                    x if x <= color_segment => Color::new(255, ramp_color(x), 0),
                    x if x <= 2 * color_segment => {
                        Color::new(ramp_color(2 * color_segment - x), 255, 0)
                    }
                    x if x <= 3 * color_segment => {
                        Color::new(0, 255, ramp_color(x - 2 * color_segment))
                    }
                    x if x <= 4 * color_segment => {
                        Color::new(0, ramp_color(4 * color_segment - x), 255)
                    }
                    x if x <= 5 * color_segment => {
                        Color::new(ramp_color(x - 4 * color_segment), 0, 255)
                    }
                    x if x <= 6 * color_segment => {
                        Color::new(255, 0, ramp_color(6 * color_segment - x))
                    }
                    _ => Color::new(255, 0, 0),
                };
            }
        }
    });

    loop {
        cortex_m::asm::wfi();
    }
}
