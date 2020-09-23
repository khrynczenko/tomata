use std::rc::Rc;
use std::time::Duration;

use druid::{Data, Lens};

use crate::tomata::{Period, ZERO};
use crate::settings::Settings;

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
            current_period: Period::WorkPeriod,
            paused: true,
            period_finished: false,
        }
    }
}

impl TomataState {
    pub(crate) fn is_paused(&self) -> bool {
        self.paused
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.period_finished
    }

    pub(crate) fn pause(&mut self) {
        self.paused = true;
    }

    pub(crate) fn start(&mut self) {
        self.paused = false;
    }

    pub(crate) fn cycle_to_next_period(&mut self) {
        match self.current_period {
            Period::WorkPeriod => self.activate_short_break(),
            Period::ShortBreak => self.activate_work(),
            Period::LongBreak => self.activate_work(),
        }
    }

    pub(crate) fn activate_work(&mut self) {
        self.current_period = Period::WorkPeriod;
        self.paused = true;
        self.period_finished = false;
        self.elapsed_time = Rc::new(ZERO);
    }

    pub(crate) fn activate_short_break(&mut self) {
        self.current_period = Period::ShortBreak;
        self.paused = true;
        self.period_finished = false;
        self.elapsed_time = Rc::new(ZERO);
    }

    pub(crate) fn activate_long_break(&mut self) {
        self.current_period = Period::LongBreak;
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

    pub(crate) fn calculate_remaining_time(&self) -> Duration {
        let period_duration = self.settings.get_duration_for_period(self.current_period);
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
    fn calculate_remaining_time_returns_zero_when_elapsed_time_is_bigger_that_period() {
        let duration = Duration::from_secs(HOUR_S);
        let mut state = TomataState::default();
        state.elapsed_time = Rc::new(duration);
        let remaining_time = state.calculate_remaining_time();
        assert_eq!(remaining_time, ZERO);
    }
}
