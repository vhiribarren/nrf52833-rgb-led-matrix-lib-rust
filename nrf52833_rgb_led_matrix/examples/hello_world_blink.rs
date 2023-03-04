/*
MIT License

Copyright (c) 2023 Vincent Hiribarren

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

use cortex_m::prelude::*;
use cortex_m_rt::entry;

use nrf52833_hal::Delay;
use nrf52833_rgb_led_matrix::canvas::{Canvas, Color, TextOptions};
use nrf52833_rgb_led_matrix::fonts::Font5x7;
use nrf52833_rgb_led_matrix::{
    init_scheduled_led_matrix_system, register_panic_handler_with_logging,
};

const CANVAS_SWITCH_DELAY_MICROSEC: u32 = 2_000_000;

#[entry]
fn main() -> ! {
    register_panic_handler_with_logging!();
    let peripherals = nrf52833_hal::pac::Peripherals::take().unwrap();
    let core_periphs = nrf52833_hal::pac::CorePeripherals::take().unwrap();

    let scheduled_led_matrix = init_scheduled_led_matrix_system!(peripherals);

    let mut delay = Delay::new(core_periphs.SYST);

    let mut next_canvas = Canvas::with_64x32();

    cortex_m::interrupt::free(|cs| {
        let mut borrowed_scheduled_led_matrix = scheduled_led_matrix.borrow(cs).borrow_mut();
        let led_matrix = borrowed_scheduled_led_matrix.as_mut().unwrap();
        next_canvas.draw_text(
            1,
            1,
            "HELLO",
            Font5x7,
            TextOptions {
                color: Color::RED,
                ..Default::default()
            },
        );
        led_matrix.swap_canvas(&mut next_canvas);
        next_canvas.draw_text(
            1,
            1,
            "WORLD",
            Font5x7,
            TextOptions {
                color: Color::BLUE,
                ..Default::default()
            },
        );
    });

    loop {
        delay.delay_us(CANVAS_SWITCH_DELAY_MICROSEC);
        cortex_m::interrupt::free(|cs| {
            let mut borrowed_scheduled_led_matrix = scheduled_led_matrix.borrow(cs).borrow_mut();
            let led_matrix = borrowed_scheduled_led_matrix.as_mut().unwrap();
            led_matrix.swap_canvas(&mut next_canvas);
        });
    }
}
