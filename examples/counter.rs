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

use microbit::hal::prelude::*;
use microbit::hal::Delay;
use microbit_led_matrix::{fonts::Font5x7, init_scheduled_led_matrix};

const COUNTER_DELAY_MS: u32 = 1000;

#[entry]
fn main() -> ! {
    let peripherals = microbit::Peripherals::take().unwrap();
    let scheduled_led_matrix = init_scheduled_led_matrix!(peripherals);

    let core_periphs = microbit::pac::CorePeripherals::take().unwrap();
    let mut delay = Delay::new(core_periphs.SYST);
    let mut counter = 0;

    loop {
        cortex_m::interrupt::free(|cs| {
            let mut borrowed_scheduled_led_matrix = scheduled_led_matrix.borrow(cs).borrow_mut();
            let led_matrix = borrowed_scheduled_led_matrix.as_mut().unwrap();
            let canvas = led_matrix.borrow_mut_canvas();
            canvas.clear();
            canvas.draw_number(0, 0, counter, Font5x7, Default::default());
        });

        delay.delay_ms(COUNTER_DELAY_MS);
        counter += 1;
    }
}
