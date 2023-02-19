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

use microbit::hal::gpio;
use microbit_led_matrix::ledmatrix::{LedMatrix, LedMatrixPins64x32};

use microbit_led_matrix::models;
use microbit_led_matrix::scheduler::ScheduledLedMatrix;
use microbit_led_matrix::timer::Timer16Mhz;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = microbit::Peripherals::take().unwrap();

    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let p1 = gpio::p1::Parts::new(peripherals.P1);

    let m = LedMatrix::new(LedMatrixPins64x32 {
        r1: p0.p0_02.into(),
        g1: p0.p0_03.into(),
        b1: p0.p0_04.into(),
        r2: p0.p0_31.into(),
        g2: p0.p0_28.into(),
        b2: p0.p0_14.into(),
        a: p1.p1_05.into(),
        b: p0.p0_11.into(),
        c: p0.p0_10.into(),
        d: p0.p0_09.into(),
        clk: p0.p0_30.into(),
        lat: p0.p0_23.into(),
        oe: p0.p0_12.into(),
    });

    let scheduled_let_matrix = ScheduledLedMatrix::take_ref(m, Timer16Mhz::new(peripherals.TIMER0));

    cortex_m::interrupt::free(|cs| {
        let mut borrowed_scheduled_led_matrix = scheduled_let_matrix.borrow(cs).borrow_mut();
        let led_matrix = borrowed_scheduled_led_matrix.as_mut().unwrap();
        let canvas = led_matrix.borrow_mut_canvas();
        canvas.draw_canvas(5, 5, &models::FRENCH_FLAG, Default::default());
        led_matrix.start_rendering_loop();
    });

    loop {
        cortex_m::asm::wfi();
    }
}
