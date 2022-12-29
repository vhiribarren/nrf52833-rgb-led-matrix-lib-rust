use microbit::hal::gpio::{Level, Output, Pin, PushPull};
use microbit::hal::prelude::*;

pub struct LedMatrix {
    pub pin_r1: Pin<Output<PushPull>>,
    pub pin_g1: Pin<Output<PushPull>>,
    pub pin_b1: Pin<Output<PushPull>>,
    pub pin_r2: Pin<Output<PushPull>>,
    pub pin_g2: Pin<Output<PushPull>>,
    pub pin_b2: Pin<Output<PushPull>>,
    pub pin_a: Pin<Output<PushPull>>,
    pub pin_b: Pin<Output<PushPull>>,
    pub pin_c: Pin<Output<PushPull>>,
    pub pin_d: Pin<Output<PushPull>>,
    pub pin_clk: Pin<Output<PushPull>>,
    pub pin_lat: Pin<Output<PushPull>>,
    pub pin_oe: Pin<Output<PushPull>>,
}
