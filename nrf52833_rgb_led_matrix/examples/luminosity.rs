/*
MIT License

Copyright (c) 2026 Vincent Hiribarren

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
        let inv_w = 1.0 / (w - 1) as f32;
        let inv_h = 1.0 / (h - 1) as f32;
        for y in 0..h {
            for x in 0..w {
                let coeff = (255.0 * (y as f32 * inv_h) * (x as f32 * inv_w)) as u8;
                canvas.draw_pixel(x, y, Color::new(coeff, coeff, coeff));
            }
        }
    });
    loop {
        cortex_m::asm::wfi();
    }
}
