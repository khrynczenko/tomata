//! The state of the application reperesented by [`TomataState`]
//! acts as a model for the application. It is used by the widgets
//! to present significant data such as remaining time etc.
use std::rc::Rc;
use std::time::Duration;

use druid::{Data, Lens};
use notify_rust::Notification;

use crate::settings::Settings;
use crate::sound::BEEPER;
use crate::tomata::{Period, ZERO};

#[derive(Debug, Clone, Data, Lens)]
pub struct TomataState {
    settings: Settings,
    elapsed_time: Rc<Duration>, // Data cannot be derived for Duration, unless it is in Rc
    current_period: Period,
    stopwatch_is_paused: bool,
    period_is_finished: bool,
    short_breaks_finished: usize,
}

impl Default for TomataState {
    fn default() -> TomataState {
        let elapsed_time = Rc::new(ZERO);
        let settings = Settings::default();
        TomataState {
            settings,
            elapsed_time,
            current_period: Period::Work,
            stopwatch_is_paused: true,
            period_is_finished: false,
            short_breaks_finished: 0,
        }
    }
}

impl TomataState {
    pub fn new(settings: Settings) -> TomataState {
        TomataState {
            settings,
            ..Default::default()
        }
    }

    pub fn beep(&self) {
        let volume = self.settings.get_beep_volume();
        std::thread::spawn(move || {
            BEEPER.get().unwrap().beep(volume).unwrap();
        });
    }

    pub fn is_stopwatch_paused(&self) -> bool {
        self.stopwatch_is_paused
    }

    pub fn is_period_finished(&self) -> bool {
        self.period_is_finished
    }

    pub fn start_stopwatch(&mut self) {
        self.stopwatch_is_paused = false;
    }

    pub fn pause_stopwatch(&mut self) {
        self.stopwatch_is_paused = true;
    }

    pub fn reset_stopwatch(&mut self) {
        self.activate_period(self.current_period);
    }

    pub fn cycle_to_next_period(&mut self) {
        match self.current_period {
            Period::Work => {
                if self.is_long_break_next() {
                    self.activate_period(Period::LongBreak);
                } else if self.settings.get_short_breaks_number() > 0 {
                    self.activate_period(Period::ShortBreak);
                } else {
                    self.activate_period(Period::Work);
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
        self.period_is_finished = false;
        self.elapsed_time = Rc::new(ZERO);
        if self.settings.does_next_period_start_automatically() {
            self.stopwatch_is_paused = false;
        } else {
            self.stopwatch_is_paused = true;
        }

        if self.settings.are_system_notifications_enabled() {
            Notification::from(period).show().unwrap();
        }
    }

    pub fn increase_elapsed_time(&mut self, value: Duration) {
        if self.is_period_finishing() && self.settings.is_period_ending_sound_enabled() {
            self.beep();
        }

        self.elapsed_time = Rc::new(*self.elapsed_time + value);
        let period_duration = self
            .settings
            .convert_period_to_duration(self.current_period);
        if period_duration <= *self.elapsed_time {
            self.period_is_finished = true;
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

    fn is_period_finishing(&self) -> bool {
        self.calculate_remaining_time() <= Duration::from_secs(5)
    }

    fn is_long_break_next(&self) -> bool {
        self.short_breaks_finished == self.settings.get_short_breaks_number()
            && self.settings.are_long_breaks_included()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tomata::HOUR_S;

    fn make_default_test_state() -> TomataState {
        // normal settings but with system notifications are disabled
        let settings = Settings::new(
            Duration::from_secs(1),
            Duration::from_secs(1),
            Duration::from_secs(1),
            2,
            true,
            true,
            false, // during tests we don't want the system notifications
            // running
            false, // during tests we don't want the beep sound effect
            0.1,
        );
        TomataState::new(settings)
    }

    #[test]
    fn remaining_time_is_zero_when_elapsed_time_is_bigger_than_period_time() {
        let duration = Duration::from_secs(HOUR_S);
        let mut state = make_default_test_state();
        state.elapsed_time = Rc::new(duration);
        let remaining_time = state.calculate_remaining_time();
        assert_eq!(remaining_time, ZERO);
    }

    #[test]
    fn increasing_elapsed_time() {
        let duration = Duration::from_secs(HOUR_S);
        let mut state = make_default_test_state();
        state.increase_elapsed_time(duration.clone());
        assert_eq!(*state.elapsed_time, duration);
    }

    #[test]
    fn activating_work_period() {
        let mut state = make_default_test_state();
        state.activate_period(Period::Work);
        assert_eq!(state.current_period, Period::Work);
        assert_eq!(
            state.stopwatch_is_paused,
            !state.settings.does_next_period_start_automatically()
        );
        assert_eq!(*state.elapsed_time, ZERO);
    }

    #[test]
    fn activating_short_break() {
        let mut state = make_default_test_state();
        state.activate_period(Period::ShortBreak);
        assert_eq!(state.current_period, Period::ShortBreak);
        assert_eq!(
            state.stopwatch_is_paused,
            !state.settings.does_next_period_start_automatically()
        );
        assert_eq!(*state.elapsed_time, ZERO);
    }

    #[test]
    fn activating_long_break() {
        let mut state = make_default_test_state();
        state.activate_period(Period::LongBreak);
        assert_eq!(state.current_period, Period::LongBreak);
        assert_eq!(
            state.stopwatch_is_paused,
            !state.settings.does_next_period_start_automatically()
        );
        assert_eq!(*state.elapsed_time, ZERO);
    }

    #[test]
    fn cycling_over_all_periods() {
        let mut state = make_default_test_state();
        let short_breaks_number = state.settings.get_short_breaks_number();
        for _ in 0..short_breaks_number {
            assert_eq!(state.current_period, Period::Work);
            state.cycle_to_next_period();
            assert_eq!(state.current_period, Period::ShortBreak);
            state.cycle_to_next_period();
        }
        assert_eq!(state.current_period, Period::Work);
        if state.settings.are_long_breaks_included() {
            state.cycle_to_next_period();
            assert_eq!(state.current_period, Period::LongBreak);
            state.cycle_to_next_period();
            assert_eq!(state.current_period, Period::Work);
        }
    }

    #[test]
    fn checking_if_period_is_finishing() {
        let state = make_default_test_state();
        assert!(state.is_period_finishing());
    }
}
