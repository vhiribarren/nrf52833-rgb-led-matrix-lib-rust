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

use nrf52833_hal::gpio::{self, Disconnected, Pin};

use nrf52833_hal::pac::{P0, P1};

use crate::ledmatrix::LedMatrixPins64x32;

/// Helper to launch examples without repeating to much code.
///
/// It uses a macro, because of issues with partial move if I try to transfer
/// the Peripherals variable using functions.
#[macro_export]
macro_rules! init_scheduled_led_matrix_system {
    ($peripherals:ident) => {{
        use $crate::ledmatrix::LedMatrix;
        use $crate::scheduler::ScheduledLedMatrix;
        use $crate::timer::Timer16Mhz;
        use $crate::utils::MicrobitPinMapFor64x32;

        #[cfg(feature = "logging")]
        {
            use $crate::metrics::*;
            let timer_source = init_global_time_source($peripherals.CLOCK, $peripherals.RTC2);
            init_debug_metrics(timer_source);
        }

        let pins = MicrobitPinMapFor64x32::new($peripherals.P0, $peripherals.P1);
        let led_matrix = LedMatrix::new(pins.led_matrix);
        let scheduled_led_matrix =
            ScheduledLedMatrix::take_ref(led_matrix, Timer16Mhz::new($peripherals.TIMER4));

        ::cortex_m::interrupt::free(|cs| {
            let mut borrowed_scheduled_led_matrix = scheduled_led_matrix.borrow(cs).borrow_mut();
            let led_matrix = borrowed_scheduled_led_matrix.as_mut().unwrap();
            led_matrix.start_rendering_loop();
        });

        scheduled_led_matrix
    }};
}

#[macro_export]
macro_rules! register_panic_handler_with_logging {
    () => {
        #[cfg(not(feature = "logging"))]
        use panic_halt as _;

        #[cfg(feature = "logging")]
        {
            use panic_rtt_target as _;
            rtt_target::rtt_init_print!();
            $crate::log!("Logging active");
        }
    };
}

pub struct MicrobitPinMapFor64x32 {
    pub led_matrix: LedMatrixPins64x32<Disconnected>,
    pub button_a: Pin<Disconnected>,
    pub button_b: Pin<Disconnected>,
}

/// P0, P1, P2 -> rgb1
/// P7, P8, P9 -> rgb2
/// P12, P13, P14, P15 -> abcd
/// P16, P19, P20 -> clk, oe, l
///
/// Microbit Buttons are on P5 & P11
impl MicrobitPinMapFor64x32 {
    pub fn new(p0: P0, p1: P1) -> Self {
        let p0_parts = gpio::p0::Parts::new(p0);
        let p1_parts = gpio::p1::Parts::new(p1);
        let led_matrix = LedMatrixPins64x32 {
            r1: p0_parts.p0_02.into(),
            g1: p0_parts.p0_03.into(),
            b1: p0_parts.p0_04.into(),
            r2: p0_parts.p0_11.into(),
            g2: p0_parts.p0_10.into(),
            b2: p0_parts.p0_09.into(),
            a: p0_parts.p0_12.into(),
            b: p0_parts.p0_17.into(),
            c: p0_parts.p0_01.into(),
            d: p0_parts.p0_13.into(),
            clk: p1_parts.p1_02.into(),
            oe: p0_parts.p0_26.into(),
            lat: p1_parts.p1_00.into(),
        };
        let button_a = p0_parts.p0_14.into();
        let button_b = p0_parts.p0_23.into();
        Self {
            led_matrix,
            button_a,
            button_b,
        }
    }
}
