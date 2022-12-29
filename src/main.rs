#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;

use microbit::board::Board;
use microbit::hal::gpio::Level;
use microbit::hal::prelude::*;
use microbit::hal::timer::Timer;
use microbit_led_matrix::ledmatrix::LedMatrix;
use panic_halt as _;
use rtt_target::rtt_init_print;

const DELAY: u32 = 1000;

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

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut m = LedMatrix {
        pin_r1: board.pins.p0_02.into_push_pull_output(Level::Low).into(),
        pin_g1: board.pins.p0_03.into_push_pull_output(Level::Low).into(),
        pin_b1: board.pins.p0_04.into_push_pull_output(Level::Low).into(),
        pin_r2: board
            .display_pins
            .col3
            .into_push_pull_output(Level::Low)
            .into(),
        pin_g2: board
            .display_pins
            .col1
            .into_push_pull_output(Level::Low)
            .into(),
        pin_b2: board
            .buttons
            .button_a
            .into_push_pull_output(Level::Low)
            .into(),
        pin_clk: board
            .display_pins
            .col5
            .into_push_pull_output(Level::Low)
            .into(),
        pin_lat: board
            .buttons
            .button_b
            .into_push_pull_output(Level::Low)
            .into(),
        pin_oe: board.pins.p0_12.into_push_pull_output(Level::Low).into(),
        pin_a: board
            .display_pins
            .col4
            .into_push_pull_output(Level::Low)
            .into(),
        pin_b: board
            .display_pins
            .col2
            .into_push_pull_output(Level::Low)
            .into(),
        pin_c: board.pins.p0_10.into_push_pull_output(Level::Low).into(),
        pin_d: board.pins.p0_09.into_push_pull_output(Level::Low).into(),
    };

    loop {
        m.pin_r1.set_low().unwrap();
        m.pin_g1.set_low().unwrap();
        m.pin_b1.set_low().unwrap();
        m.pin_r2.set_low().unwrap();
        m.pin_g2.set_low().unwrap();
        m.pin_b2.set_high().unwrap();
        for idx in 0..64 {
            if idx < 63 {
                m.pin_r1.set_low().unwrap();
            } else {
                m.pin_r1.set_high().unwrap();
            }
            m.pin_clk.set_high().unwrap();
            m.pin_clk.set_low().unwrap();
        }
        m.pin_oe.set_high().unwrap();
        m.pin_a.set_low().unwrap();
        m.pin_b.set_low().unwrap();
        m.pin_c.set_low().unwrap();
        m.pin_d.set_low().unwrap();
        m.pin_oe.set_low().unwrap();
        m.pin_oe.set_high().unwrap();
        m.pin_lat.set_high().unwrap();
        m.pin_lat.set_low().unwrap();
        m.pin_oe.set_low().unwrap();

        m.pin_r1.set_low().unwrap();
        m.pin_g1.set_low().unwrap();
        m.pin_b1.set_low().unwrap();
        m.pin_r2.set_low().unwrap();
        m.pin_g2.set_low().unwrap();
        m.pin_b2.set_low().unwrap();
        for idx in 0..64 {
            if idx < 2 {
                m.pin_b2.set_low().unwrap();
            } else {
                m.pin_b2.set_high().unwrap();
            }
            m.pin_clk.set_high().unwrap();
            m.pin_clk.set_low().unwrap();
        }
        m.pin_oe.set_high().unwrap();
        m.pin_a.set_low().unwrap();
        m.pin_b.set_low().unwrap();
        m.pin_c.set_low().unwrap();
        m.pin_d.set_high().unwrap();
        m.pin_lat.set_high().unwrap();
        m.pin_lat.set_low().unwrap();
        m.pin_oe.set_low().unwrap();
    }
}
