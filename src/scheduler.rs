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
use crate::ledmatrix::{ColorBitPosition, LedMatrix};
use crate::timer::Timer16Mhz;
use crate::{enable_interrupts, log, MatrixTimer, MATRIX_TIMER_INTERRUPT};
use cortex_m::interrupt::Mutex;

use microbit::hal::pac::interrupt;

static SCHEDULED_LED_MATRIX: Mutex<RefCell<Option<ScheduledLedMatrix<4, 64, 32>>>> =
    Mutex::new(RefCell::new(None));

const BCM_CYCLES_NB: u8 = 2; // min is 1
const BCM_BASE_PERIOD_MICROSEC: u32 = 1;

#[interrupt]
fn TIMER0() {
    static mut CYCLE_STEP: u8 = 0;
    static mut LINE_STEP: usize = 0;
    cortex_m::interrupt::free(|cs| {
        let mut borrowed_led_matrix = SCHEDULED_LED_MATRIX.borrow(cs).borrow_mut();
        let schedule_led_matrix = borrowed_led_matrix.as_mut().unwrap();
        schedule_led_matrix.ack_interrupt();
        schedule_led_matrix.display_line(
            *LINE_STEP,
            ColorBitPosition(*CYCLE_STEP + ColorBitPosition::MSB_POSITION - BCM_CYCLES_NB + 1),
        );
        /* TODO: Bug somewhere, but I would like to test the result
        if *LINE_STEP >= schedule_led_matrix.half_height() {
            *LINE_STEP = 0;
            *CYCLE_STEP = (*CYCLE_STEP + 1_u8) % BCM_CYCLES_NB;
        }
        else {
            *LINE_STEP += 1;
        }
        */

        let next_int_delay = BCM_BASE_PERIOD_MICROSEC * 2_u32.pow(*CYCLE_STEP as u32);
        schedule_led_matrix.schedule_next_interrupt(next_int_delay);

        if *CYCLE_STEP >= BCM_CYCLES_NB - 1 {
            *CYCLE_STEP = 0;
            *LINE_STEP = (*LINE_STEP + 1) % schedule_led_matrix.half_height();
        } else {
            *CYCLE_STEP += 1;
        }
    });
}

pub struct ScheduledLedMatrix<
    const LINECTRL_PIN_COUNT: usize = 4,
    const WIDTH: usize = 64,
    const HEIGHT: usize = 32,
> {
    front_canvas: Canvas<WIDTH, HEIGHT>,
    led_matrix: LedMatrix<LINECTRL_PIN_COUNT, WIDTH, HEIGHT>,
    timer: Timer16Mhz<MatrixTimer>,
}

impl ScheduledLedMatrix<4, 64, 32> {
    pub fn take_ref(
        led_matrix: LedMatrix<4, 64, 32>,
        timer: Timer16Mhz<MatrixTimer>,
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
    pub fn half_height(&self) -> usize {
        HEIGHT / 2
    }

    // fn start_rendering_loop(self) -> Self<started>
    pub fn start_rendering_loop(&mut self) {
        log!("Start rendering loop");
        self.schedule_next_interrupt(BCM_BASE_PERIOD_MICROSEC);
    }

    pub fn swap_canvas(&mut self, canvas: &mut Canvas<WIDTH, HEIGHT>) {
        core::mem::swap(&mut self.front_canvas, canvas);
    }

    pub fn copy_canvas(&mut self, canvas: &Canvas<WIDTH, HEIGHT>) {
        self.front_canvas = canvas.clone();
    }

    pub fn borrow_mut_canvas(&mut self) -> &mut Canvas<WIDTH, HEIGHT> {
        &mut self.front_canvas
    }

    pub fn ack_interrupt(&mut self) {
        self.timer.disable_interrupt();
    }

    // fn refresh_display(&mut self) {
    //     self.led_matrix
    //         .draw_canvas_with_delay_buffer(&self.front_canvas, Some(&mut self.timer));
    // }

    fn display_line(&mut self, line: usize, bit_position: ColorBitPosition) {
        self.led_matrix
            .draw_canvas_line(&self.front_canvas, line, bit_position);
    }

    fn schedule_next_interrupt(&mut self, delay_microsec: u32) {
        self.timer.start(delay_microsec);
        self.timer.enable_interrupt();
    }
}
