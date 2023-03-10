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

#![no_std]

pub mod canvas;
pub mod fonts;
pub mod helpers;
pub mod ledmatrix;
pub mod metrics;
pub mod models;
pub mod readonly_cell;
pub mod scheduler;
pub mod timer;

pub type MetricsRtc = nrf52833_hal::pac::RTC2;
pub type MatrixTimer = nrf52833_hal::pac::TIMER4;
pub const MATRIX_TIMER_INTERRUPT: nrf52833_hal::pac::Interrupt =
    nrf52833_hal::pac::interrupt::TIMER4;

#[macro_export]
macro_rules! enable_interrupts {
    (  $( $interrupt_nb:path ), * ) => {
        #[allow(unsafe_code)]
        unsafe {
        $(
            nrf52833_hal::pac::NVIC::unmask($interrupt_nb);
        )*
        }
    };
}

#[macro_export]
macro_rules! log {
    ($($x:tt)*) => {
        {
            #[cfg(feature = "logging")]
            rtt_target::rprintln!($($x)*);
        }
    };
}
