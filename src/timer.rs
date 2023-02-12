use microbit::hal::timer::Instance;

pub struct Timer16Mhz<T>(T);

impl<T: Instance> Timer16Mhz<T> {
    ///  0 -> 16 MHz, 1 -> 8 MHz, ... 4 -> 1 MHz
    const PRESCALER: u8 = 4;

    pub fn new(timer: T) -> Self {
        timer
            .as_timer0()
            .shorts
            .write(|w| w.compare0_clear().enabled().compare0_stop().enabled());
        timer
            .as_timer0()
            .prescaler
            .write(|w| unsafe { w.prescaler().bits(Self::PRESCALER) });
        timer.as_timer0().bitmode.write(|w| w.bitmode()._32bit());
        Timer16Mhz(timer)
    }

    pub fn start(&mut self, cycles: u32) {
        self.0.timer_start(cycles);
    }

    pub fn enable_interrupt(&mut self) {
        self.0
            .as_timer0()
            .intenset
            .modify(|_, w| w.compare0().set());
    }

    pub fn disable_interrupt(&self) {
        self.0
            .as_timer0()
            .intenclr
            .modify(|_, w| w.compare0().clear());
    }

    pub fn read(&self) -> u32 {
        self.0.as_timer0().tasks_capture[1].write(|w| unsafe { w.bits(1) });
        self.0.as_timer0().cc[1].read().bits()
    }

    pub fn delay_us(&self, _: u32) {
        todo!()
    }
}
