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

use nrf52833_rgb_led_matrix::canvas::{Canvas, Color};
use nrf52833_rgb_led_matrix::ledmatrix::LedMatrix;
use nrf52833_rgb_led_matrix::utils::MicrobitPinMapFor64x32;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = nrf52833_hal::pac::Peripherals::take().unwrap();
    let pins = MicrobitPinMapFor64x32::new(peripherals.P0, peripherals.P1);

    let mut m = LedMatrix::new(pins.led_matrix);

    let mut canvas = Canvas::with_64x32();

    canvas.draw_rectangle(0, 0, 21, 32, Color::BLUE);
    canvas.draw_rectangle(21, 0, 22, 32, Color::WHITE);
    canvas.draw_rectangle(43, 0, 21, 32, Color::RED);

    loop {
        m.draw_canvas(&canvas, Default::default());
    }
}
