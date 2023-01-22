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

use core::cell::RefCell;

use crate::canvas::Canvas;
use crate::ledmatrix::LedMatrix;
use crate::{enable_interrupts, log, MatrixTimer, MATRIX_TIMER_INTERRUPT};
use cortex_m::interrupt::Mutex;
use microbit::hal::{prelude::*, Timer};

use microbit::hal::pac::interrupt;

const MAX_DRAW_DELAY_MICROSEC: u32 = 10_000;

static SCHEDULED_LED_MATRIX: Mutex<RefCell<Option<ScheduledLedMatrix<4, 64, 32>>>> =
    Mutex::new(RefCell::new(None));

#[interrupt]
fn TIMER0() {
    cortex_m::interrupt::free(|cs| {
        let mut borrowed_led_matrix = SCHEDULED_LED_MATRIX.borrow(cs).borrow_mut();
        let schedule_led_matrix = borrowed_led_matrix.as_mut().unwrap();
        schedule_led_matrix.ack_interrupt();
        schedule_led_matrix.refresh_display();
        schedule_led_matrix.schedule_next_interrupt();
    });
}

pub struct ScheduledLedMatrix<
    const LINECTRL_PIN_COUNT: usize = 4,
    const WIDTH: usize = 64,
    const HEIGHT: usize = 32,
> {
    front_canvas: Canvas<WIDTH, HEIGHT>,
    led_matrix: LedMatrix<LINECTRL_PIN_COUNT, WIDTH, HEIGHT>,
    timer: Timer<MatrixTimer>,
}

impl ScheduledLedMatrix<4, 64, 32> {
    pub fn take_ref(
        led_matrix: LedMatrix<4, 64, 32>,
        timer: Timer<MatrixTimer>,
    ) -> &'static Mutex<RefCell<Option<ScheduledLedMatrix<4, 64, 32>>>> {
        cortex_m::interrupt::free(|cs| {
            let borrowed_scheduled_matrix = SCHEDULED_LED_MATRIX.borrow(cs);
            if borrowed_scheduled_matrix.borrow().is_some() {
                return;
            }
            enable_interrupts!(MATRIX_TIMER_INTERRUPT);
            let scheduled_let_matrix = ScheduledLedMatrix {
                led_matrix,
                front_canvas: Default::default(),
                timer,
            };
            borrowed_scheduled_matrix.replace(Some(scheduled_let_matrix));
        });
        &SCHEDULED_LED_MATRIX
    }
}

impl<const LINECTRL_PIN_COUNT: usize, const WIDTH: usize, const HEIGHT: usize>
    ScheduledLedMatrix<LINECTRL_PIN_COUNT, WIDTH, HEIGHT>
{
    // fn start_rendering_loop(self) -> Self<started>
    pub fn start_rendering_loop(&mut self) {
        log!("Start rendering loop");
        self.schedule_next_interrupt();
    }

    pub fn swap_canvas(&mut self, canvas: &mut Canvas<WIDTH, HEIGHT>) {
        core::mem::swap(&mut self.front_canvas, canvas);
    }

    pub fn copy_canvas(&mut self, canvas: &Canvas<WIDTH, HEIGHT>) {
        self.front_canvas = canvas.clone();
    }

    pub fn ack_interrupt(&mut self) {
        self.timer.disable_interrupt();
    }

    fn refresh_display(&mut self) {
        self.led_matrix
            .draw_canvas_with_delay_buffer(&self.front_canvas, Some(&mut self.timer));
    }

    fn schedule_next_interrupt(&mut self) {
        self.timer.start(MAX_DRAW_DELAY_MICROSEC);
        self.timer.enable_interrupt();
    }
}
