use std::fs::File;
use std::io::BufWriter;
use std::io::Result as IOResult;
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use druid::{Data, Lens};
use serde_json;

use crate::settings::Settings;
use crate::tomata::{Period, ZERO};

#[derive(Debug, Clone, Data, Lens)]
pub(crate) struct TomataState {
    settings: Settings,
    elapsed_time: Rc<Duration>,
    current_period: Period,
    paused: bool,
    period_finished: bool,
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
        }
    }
}

impl TomataState {
    pub fn new(settings: Settings) -> TomataState {
        let mut state = TomataState::default();
        state.settings = settings;
        state
    }
    pub(crate) fn is_paused(&self) -> bool {
        self.paused
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.period_finished
    }

    pub(crate) fn start(&mut self) {
        self.paused = false;
    }

    pub(crate) fn pause(&mut self) {
        self.paused = true;
    }

    pub(crate) fn reset(&mut self) {
        self.activate_period(self.current_period);
    }

    pub(crate) fn get_settings(&self) -> &Settings {
        &self.settings
    }

    pub(crate) fn cycle_to_next_period(&mut self) {
        match self.current_period {
            Period::Work => self.activate_period(Period::Work),
            Period::ShortBreak => self.activate_period(Period::ShortBreak),
            Period::LongBreak => self.activate_period(Period::LongBreak),
        }
    }

    pub fn activate_period(&mut self, period: Period) {
        self.current_period = period;
        self.paused = true;
        self.period_finished = false;
        self.elapsed_time = Rc::new(ZERO);
    }

    pub(crate) fn increase_elapsed_time(&mut self, value: Duration) {
        self.elapsed_time = Rc::new(*self.elapsed_time + value);
        let period_duration = self.settings.get_duration_for_period(self.current_period);
        if period_duration <= *self.elapsed_time {
            self.period_finished = true;
        }
    }

    pub(crate) fn increase_period_duration(&mut self, period: Period, value: Duration) {
        match period {
            Period::Work => self.settings.work_period = Rc::new(*self.settings.work_period + value),
            Period::ShortBreak => {
                self.settings.short_break_period =
                    Rc::new(*self.settings.short_break_period + value)
            }
            Period::LongBreak => {
                self.settings.long_break_period = Rc::new(*self.settings.long_break_period + value)
            }
        }
    }

    pub(crate) fn decrease_period_duration(&mut self, period: Period, value: Duration) {
        match period {
            Period::Work => {
                let current_period_duration = &self.settings.work_period;
                if value > **current_period_duration {
                    self.settings.work_period = Rc::new(Duration::from_secs(0));
                    return;
                } else {
                    self.settings.work_period = Rc::new(*self.settings.work_period - value)
                }
            }
            Period::ShortBreak => {
                let current_period_duration = &self.settings.short_break_period;
                if value > **current_period_duration {
                    self.settings.short_break_period = Rc::new(Duration::from_secs(0));
                    return;
                } else {
                    self.settings.short_break_period =
                        Rc::new(*self.settings.short_break_period - value)
                }
            }
            Period::LongBreak => {
                let current_period_duration = &self.settings.long_break_period;
                if value > **current_period_duration {
                    self.settings.long_break_period = Rc::new(Duration::from_secs(0));
                    return;
                } else {
                    self.settings.long_break_period =
                        Rc::new(*self.settings.long_break_period - value)
                }
            }
        }
    }

    pub(crate) fn calculate_remaining_time(&self) -> Duration {
        let period_duration = self.settings.get_duration_for_period(self.current_period);
        if period_duration <= *self.elapsed_time {
            return ZERO;
        }
        period_duration - *self.elapsed_time
    }

    pub(crate) fn serialize_settings(&self, path: impl AsRef<Path>) -> IOResult<()> {
        let file = File::create(path)?;
        let buffer = BufWriter::new(file);
        serde_json::to_writer_pretty(buffer, &self.settings).unwrap();
        Ok(())
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
}
