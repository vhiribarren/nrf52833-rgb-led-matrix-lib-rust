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
    canvas::{Color, TextOptions},
    fonts::Font5x7,
    init_scheduled_led_matrix, models,
};

#[entry]
fn main() -> ! {
    let peripherals = nrf52833_hal::pac::Peripherals::take().unwrap();
    let scheduled_led_matrix = init_scheduled_led_matrix!(peripherals);

    cortex_m::interrupt::free(|cs| {
        let mut borrowed_scheduled_led_matrix = scheduled_led_matrix.borrow(cs).borrow_mut();
        let led_matrix = borrowed_scheduled_led_matrix.as_mut().unwrap();
        let canvas = led_matrix.borrow_mut_canvas();
        canvas.draw_canvas(0, 1, &models::BIRTHDAY_CAKE, Default::default());
        canvas.draw_text(
            34,
            4,
            "BON",
            Font5x7,
            TextOptions {
                color: Color::RED,
                ..Default::default()
            },
        );
        canvas.draw_text(
            28,
            12,
            "ANNIV",
            Font5x7,
            TextOptions {
                color: Color::GREEN,
                ..Default::default()
            },
        );
        canvas.draw_text(
            22,
            20,
            "AURELIA",
            Font5x7,
            TextOptions {
                color: Color::BLUE,
                ..Default::default()
            },
        );
    });

    loop {
        cortex_m::asm::wfi();
    }
}
