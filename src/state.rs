use std::rc::Rc;
use std::time::Duration;

use druid::{Data, Lens};

use crate::settings::Settings;
use crate::tomata::{Period, ZERO};

#[derive(Debug, Clone, Data, Lens)]
pub struct TomataState {
    settings: Settings,
    elapsed_time: Rc<Duration>,
    current_period: Period,
    paused: bool,
    period_finished: bool,
    short_breaks_finished: usize,
}

impl Default for TomataState {
    fn default() -> TomataState {
        let elapsed_time = Rc::new(ZERO);
        let settings = Settings::default();
        TomataState {
            settings: settings.clone(),
            elapsed_time: elapsed_time.clone(),
            current_period: Period::Work,
            paused: true,
            period_finished: false,
            short_breaks_finished: 0,
        }
    }
}

impl TomataState {
    pub fn new(settings: Settings) -> TomataState {
        let mut state = TomataState::default();
        state.settings = settings;
        state
    }
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn is_finished(&self) -> bool {
        self.period_finished
    }

    pub fn start(&mut self) {
        self.paused = false;
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn reset(&mut self) {
        self.activate_period(self.current_period);
    }

    pub fn cycle_to_next_period(&mut self) {
        match self.current_period {
            Period::Work => {
                if self.short_breaks_finished == self.settings.short_breaks_number {
                    self.activate_period(Period::LongBreak);
                } else {
                    self.activate_period(Period::ShortBreak);
                }
            }
            Period::ShortBreak => {
                self.short_breaks_finished += 1;
                self.activate_period(Period::Work);
            }
            Period::LongBreak => {
                self.short_breaks_finished = 0;
                self.activate_period(Period::Work);
            }
        }
    }

    pub fn activate_period(&mut self, period: Period) {
        self.current_period = period;
        self.paused = true;
        self.period_finished = false;
        self.elapsed_time = Rc::new(ZERO);
    }

    pub fn increase_elapsed_time(&mut self, value: Duration) {
        self.elapsed_time = Rc::new(*self.elapsed_time + value);
        let period_duration = self
            .settings
            .convert_period_to_duration(self.current_period);
        if period_duration <= *self.elapsed_time {
            self.period_finished = true;
        }
    }

    pub fn calculate_remaining_time(&self) -> Duration {
        let period_duration = self
            .settings
            .convert_period_to_duration(self.current_period);
        if period_duration <= *self.elapsed_time {
            return ZERO;
        }
        period_duration - *self.elapsed_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tomata::HOUR_S;

    #[test]
    fn remaining_time_is_zero_when_elapsed_time_is_bigger_than_period_time() {
        let duration = Duration::from_secs(HOUR_S);
        let mut state = TomataState::default();
        state.elapsed_time = Rc::new(duration);
        let remaining_time = state.calculate_remaining_time();
        assert_eq!(remaining_time, ZERO);
    }

    #[test]
    fn increasing_elapsed_time() {
        let duration = Duration::from_secs(HOUR_S);
        let mut state = TomataState::default();
        state.increase_elapsed_time(duration.clone());
        assert_eq!(*state.elapsed_time, duration);
    }

    #[test]
    fn activating_work_period() {
        let mut state = TomataState::default();
        state.activate_period(Period::Work);
        assert_eq!(state.current_period, Period::Work);
        assert_eq!(state.paused, true);
        assert_eq!(*state.elapsed_time, ZERO);
    }

    #[test]
    fn activating_short_break() {
        let mut state = TomataState::default();
        state.activate_period(Period::ShortBreak);
        assert_eq!(state.current_period, Period::ShortBreak);
        assert_eq!(state.paused, true);
        assert_eq!(*state.elapsed_time, ZERO);
    }

    #[test]
    fn activating_long_break() {
        let mut state = TomataState::default();
        state.activate_period(Period::LongBreak);
        assert_eq!(state.current_period, Period::LongBreak);
        assert_eq!(state.paused, true);
        assert_eq!(*state.elapsed_time, ZERO);
    }

    #[test]
    fn cycling_over_all_periods() {
        let mut state = TomataState::default();
        let short_breaks_number = state.settings.short_breaks_number;
        for _ in 0..short_breaks_number {
            assert_eq!(state.current_period, Period::Work);
            state.cycle_to_next_period();
            assert_eq!(state.current_period, Period::ShortBreak);
            state.cycle_to_next_period();
        }
        assert_eq!(state.current_period, Period::Work);
        state.cycle_to_next_period();
        assert_eq!(state.current_period, Period::LongBreak);
        state.cycle_to_next_period();
        assert_eq!(state.current_period, Period::Work);
    }
}
