use std::rc::Rc;
use std::time::Duration;
use std::u64;

use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

use crate::tomata::{Period, MINUTE_S};

pub const TWENTY_FIVE_MINUTES: u64 = MINUTE_S * 25;
pub const FIVE_MINUTES: u64 = MINUTE_S * 5;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Data, Lens)]
pub struct Settings {
    pub work_period: Rc<Duration>,
    pub short_break_period: Rc<Duration>,
    pub long_break_period: Rc<Duration>,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            work_period: Rc::new(Duration::from_secs(10)),
            short_break_period: Rc::new(Duration::from_secs(5)),
            long_break_period: Rc::new(Duration::from_secs(8)),
        }
    }
}

impl Settings {
    pub fn get_duration_for_period(&self, period: Period) -> Duration {
        match period {
            Period::WorkPeriod => *self.work_period,
            Period::ShortBreak => *self.short_break_period,
            Period::LongBreak => *self.long_break_period,
        }
    }
}
