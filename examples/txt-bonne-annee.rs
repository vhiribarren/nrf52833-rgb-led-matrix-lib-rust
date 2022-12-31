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

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;

use microbit::hal::gpio;
use microbit_led_matrix::canvas::{Canvas, Color};
use microbit_led_matrix::fonts::font5x7;
use microbit_led_matrix::ledmatrix::{LedMatrix, LedMatrixPins64x32};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = microbit::Peripherals::take().unwrap();
    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let p1 = gpio::p1::Parts::new(peripherals.P1);

    let mut m = LedMatrix::new(LedMatrixPins64x32 {
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

    let mut canvas = Canvas::<64, 32>::new();

    canvas.draw_stencil(0, 8, &font5x7::B, Color::RED);
    canvas.draw_stencil(6, 8, &font5x7::O, Color::RED);
    canvas.draw_stencil(12, 8, &font5x7::N, Color::RED);
    canvas.draw_stencil(18, 8, &font5x7::N, Color::RED);
    canvas.draw_stencil(24, 8, &font5x7::E, Color::RED);

    canvas.draw_stencil(0, 16, &font5x7::A, Color::BLUE);
    canvas.draw_stencil(6, 16, &font5x7::N, Color::BLUE);
    canvas.draw_stencil(12, 16, &font5x7::N, Color::BLUE);
    canvas.draw_stencil(18, 16, &font5x7::E, Color::BLUE);
    canvas.draw_stencil(24, 16, &font5x7::E, Color::BLUE);

    canvas.draw_stencil(34, 12, &font5x7::N_2, Color::WHITE);
    canvas.draw_stencil(40, 12, &font5x7::N_0, Color::WHITE);
    canvas.draw_stencil(46, 12, &font5x7::N_2, Color::WHITE);
    canvas.draw_stencil(52, 12, &font5x7::N_3, Color::WHITE);

    loop {
        m.draw_canvas(&canvas);
    }
}
