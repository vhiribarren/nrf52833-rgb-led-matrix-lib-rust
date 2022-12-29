use microbit::hal::gpio::{Level, Output, Pin, PushPull};
use microbit::hal::prelude::*;

pub struct LedMatrixPins64x32 {
    pub r1: Pin<Output<PushPull>>,
    pub g1: Pin<Output<PushPull>>,
    pub b1: Pin<Output<PushPull>>,
    pub r2: Pin<Output<PushPull>>,
    pub g2: Pin<Output<PushPull>>,
    pub b2: Pin<Output<PushPull>>,
    pub a: Pin<Output<PushPull>>,
    pub b: Pin<Output<PushPull>>,
    pub c: Pin<Output<PushPull>>,
    pub d: Pin<Output<PushPull>>,
    pub clk: Pin<Output<PushPull>>,
    pub lat: Pin<Output<PushPull>>,
    pub oe: Pin<Output<PushPull>>,
}

pub struct LedMatrix<const N: usize = 4> {
    pub pin_r1: Pin<Output<PushPull>>,
    pub pin_g1: Pin<Output<PushPull>>,
    pub pin_b1: Pin<Output<PushPull>>,
    pub pin_r2: Pin<Output<PushPull>>,
    pub pin_g2: Pin<Output<PushPull>>,
    pub pin_b2: Pin<Output<PushPull>>,
    pub pin_clk: Pin<Output<PushPull>>,
    pin_lat: Pin<Output<PushPull>>,
    pin_oe: Pin<Output<PushPull>>,
    line_ctrl: [Pin<Output<PushPull>>; N],
}

impl LedMatrix {
    pub fn new(pins: LedMatrixPins64x32) -> LedMatrix {
        LedMatrix {
            pin_r1: pins.r1.into_push_pull_output(Level::Low),
            pin_g1: pins.g1.into_push_pull_output(Level::Low),
            pin_b1: pins.b1.into_push_pull_output(Level::Low),
            pin_r2: pins.r2.into_push_pull_output(Level::Low),
            pin_g2: pins.g2.into_push_pull_output(Level::Low),
            pin_b2: pins.b2.into_push_pull_output(Level::Low),
            pin_clk: pins.clk.into_push_pull_output(Level::Low),
            pin_lat: pins.lat.into_push_pull_output(Level::Low),
            pin_oe: pins.oe.into_push_pull_output(Level::Low),
            line_ctrl: [
                pins.a.into_push_pull_output(Level::Low),
                pins.b.into_push_pull_output(Level::Low),
                pins.c.into_push_pull_output(Level::Low),
                pins.d.into_push_pull_output(Level::Low),
            ],
        }
    }
}

impl<const N: usize> LedMatrix<N> {
    pub fn draw_canvas() {}
    pub fn latch_to_line(&mut self, line: u8) {
        self.pin_oe.set_high().unwrap();
        let mline = line % 2_u8.pow(N as u32);
        for pin_idx in 0..self.line_ctrl.len() {
            let enable_pin = (mline & (1 << pin_idx)) != 0;
            self.line_ctrl[pin_idx]
                .set_state(PinState::from(enable_pin))
                .unwrap();
        }
        self.pin_lat.set_high().unwrap();
        self.pin_lat.set_low().unwrap();
        self.pin_oe.set_low().unwrap();
    }
}
