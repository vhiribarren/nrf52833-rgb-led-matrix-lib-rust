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

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m::prelude::*;
use cortex_m_rt::entry;

use microbit::hal::pac::interrupt;
use microbit::hal::timer::Timer;
use microbit::hal::{gpio, Delay};
use microbit::pac::{TIMER0, TIMER1};
use microbit_led_matrix::canvas::{Canvas, Color};
use microbit_led_matrix::ledmatrix::{LedMatrix, LedMatrixPins64x32, ScheduledLedMatrix};

#[cfg(not(feature = "logging"))]
use panic_halt as _;

#[cfg(feature = "logging")]
use panic_rtt_target as _;

#[cfg(feature = "logging")]
use rtt_target::{rprintln, rtt_init_print};

/*
If there are not regular switch between two elements of A, B, C or D, the panel shutdown.
A minimal block like this one is needed to allow the system working:
   loop {
        pin_a.set_low().unwrap();
        pin_a.set_high().unwrap();
   }

Correct order is:
    for _ in range(64):
        select colors
        clock it, H then L
    set OE to H
    select A, B, C, D
    set latch to H
    set latch to L
    set OE to L
*/

const CANVAS_SWITCH_DELAY_MICROSEC: u32 = 2_000_000;

macro_rules! enable_interrupts {
    (  $( $interrupt_nb:path ), * ) => {
        #[allow(unsafe_code)]
        unsafe {
        $(
            microbit::pac::NVIC::unmask($interrupt_nb);
        )*
        }
    };
}

#[entry]
fn main() -> ! {
    #[cfg(feature = "logging")]
    {
        rtt_init_print!();
        rprintln!("Logging active");
    }

    enable_interrupts!(interrupt::TIMER0, interrupt::TIMER1);

    let peripherals = microbit::Peripherals::take().unwrap();

    let timer = Timer::new(peripherals.TIMER0);
    let mut delay = Delay::new(cortex_m::Peripherals::take().unwrap().SYST);
    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let p1 = gpio::p1::Parts::new(peripherals.P1);

    let m = LedMatrix::new(
        LedMatrixPins64x32 {
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
        },
        timer,
    );

    let mut canvas_1 = Canvas::with_64x32();
    canvas_1.draw_text(1, 1, "HELLO", Color::RED);
    let mut canvas_2 = Canvas::with_64x32();
    canvas_2.draw_text(1, 1, "WORLD", Color::RED);

    let scheduled_let_matrix =
        ScheduledLedMatrix::<TIMER0, 4, 64, 32>::new(m, Timer::new(peripherals.TIMER1));

    cortex_m::interrupt::free(|cs| {
        let mut borrowed_scheduled_led_matrix = scheduled_let_matrix.borrow(cs).borrow_mut();
        let led_matrix = borrowed_scheduled_led_matrix.as_mut().unwrap();
        led_matrix.start_rendering_loop();
        led_matrix.copy_canvas(&canvas_1);
    });

    let next_canvas = &mut canvas_2;

    #[cfg(feature = "logging")]
    rprintln!("Start loop!");
    loop {
        delay.delay_us(CANVAS_SWITCH_DELAY_MICROSEC);
        #[cfg(feature = "logging")]
        rprintln!("Switch!");
        cortex_m::interrupt::free(|cs| {
            let mut borrowed_scheduled_led_matrix = scheduled_let_matrix.borrow(cs).borrow_mut();
            let led_matrix = borrowed_scheduled_led_matrix.as_mut().unwrap();
            led_matrix.swap_canvas(next_canvas);
        });
    }
}
