use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use microbit::{
    hal::{
        rtc::{Instance, Rtc},
        Clocks,
    },
    pac::{CLOCK, RTC0},
};

use crate::{log, readonly_cell::DynamicReadOnlyCell};

const DRAW_CYCLE_LOG_PERIOD_MS: u32 = 1_000;

static TIMER_SOURCE: DynamicReadOnlyCell<RTCTimerSource<RTC0>> = DynamicReadOnlyCell::new();

type DrawCycleMetric<'a> =
    Mutex<RefCell<Option<AverageFrequencyMeasure<'a, RTCTimerSource<RTC0>, fn(u32)>>>>;
pub(crate) static DRAW_CYCLE_METRICS: DrawCycleMetric = Mutex::new(RefCell::new(None));

#[allow(unused_variables)]
fn log_image_freq(freq: u32) {
    log!("Image drawing frequency: {} Hz", freq);
}

pub fn init_global_time_source(
    clock: CLOCK,
    rtc_periph: RTC0,
) -> &'static DynamicReadOnlyCell<RTCTimerSource<RTC0>> {
    if TIMER_SOURCE.try_get_ref().is_some() {
        return &TIMER_SOURCE;
    }
    let clocks = Clocks::new(clock);
    clocks.start_lfclk();
    let rtc_hal = Rtc::new(rtc_periph, 0).unwrap();
    let timer_source = RTCTimerSource::new(rtc_hal);
    TIMER_SOURCE.set(timer_source);
    &TIMER_SOURCE
}

pub fn init_debug_metrics(timer_source: &'static DynamicReadOnlyCell<RTCTimerSource<RTC0>>) {
    cortex_m::interrupt::free(|cs| {
        let mut draw_cycle_measure_borrow = DRAW_CYCLE_METRICS.borrow(cs).borrow_mut();
        let avg_freq_measure = AverageFrequencyMeasure::new(
            timer_source,
            DRAW_CYCLE_LOG_PERIOD_MS,
            log_image_freq as fn(u32),
        );
        draw_cycle_measure_borrow.replace(avg_freq_measure);
    });
}

pub trait TimerSource {
    fn current_value(&self) -> u32;
    fn frequency(&self) -> u32;
}

pub struct RTCTimerSource<I> {
    source: Rtc<I>,
}

/// Do not forget that the low frequency clock must be started before somewhere in the code.
impl<I: Instance> RTCTimerSource<I> {
    pub fn new(source: Rtc<I>) -> Self {
        source.enable_counter();
        Self { source }
    }
}

impl<I: Instance> TimerSource for RTCTimerSource<I> {
    fn current_value(&self) -> u32 {
        self.source.get_counter()
    }

    fn frequency(&self) -> u32 {
        microbit::hal::clocks::LFCLK_FREQ
    }
}

pub struct AverageFrequencyMeasure<'a, T, F> {
    timer_source: &'a DynamicReadOnlyCell<T>,
    action: F,
    timer_delta_max: u32,
    counter_val: u32,
    timer_last_measured: u32,
}

impl<'a, T: TimerSource, F: Fn(u32)> AverageFrequencyMeasure<'a, T, F> {
    pub fn new(timer_source: &'a DynamicReadOnlyCell<T>, log_period_ms: u32, action: F) -> Self {
        let frequency = timer_source.get_ref().frequency();
        AverageFrequencyMeasure {
            timer_source,
            action,
            timer_delta_max: frequency * log_period_ms / 1000,
            timer_last_measured: 0,
            counter_val: 0,
        }
    }

    pub fn inc_period(&mut self) {
        let current_timer_value = self.timer_source.get_ref().current_value();
        if self.counter_val == 0 {
            self.timer_last_measured = current_timer_value;
            self.counter_val += 1;
            return;
        }
        let time_delta = if current_timer_value > self.timer_last_measured {
            current_timer_value - self.timer_last_measured
        } else {
            current_timer_value + (u32::MAX - self.timer_last_measured)
        };
        if time_delta > self.timer_delta_max {
            self.trigger_end_cycle(time_delta);
        } else {
            self.counter_val += 1;
        }
    }

    fn trigger_end_cycle(&mut self, time_delta: u32) {
        let timer_frequency = self.timer_source.get_ref().frequency();
        let avg_freq = self.counter_val * timer_frequency / time_delta;
        (self.action)(avg_freq);
        self.counter_val = 0;
    }
}
