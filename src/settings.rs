use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;
use std::u64;

use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

use crate::tomata::{Period, MINUTE_S};

pub const TWENTY_FIVE_MINUTES: u64 = MINUTE_S * 25;
pub const FIVE_MINUTES: u64 = MINUTE_S * 5;
pub const EIGHT_MINUTES: u64 = MINUTE_S * 8;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Data, Lens)]
pub struct Settings {
    pub work_period: Rc<Duration>,
    pub short_break_period: Rc<Duration>,
    pub long_break_period: Rc<Duration>,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            work_period: Rc::new(Duration::from_secs(TWENTY_FIVE_MINUTES)),
            short_break_period: Rc::new(Duration::from_secs(FIVE_MINUTES)),
            long_break_period: Rc::new(Duration::from_secs(EIGHT_MINUTES)),
        }
    }
}

impl Settings {
    pub fn get_duration_for_period(&self, period: Period) -> Duration {
        match period {
            Period::Work => *self.work_period,
            Period::ShortBreak => *self.short_break_period,
            Period::LongBreak => *self.long_break_period,
        }
    }

    pub fn from_file(path: impl AsRef<Path>) -> Option<Settings> {
        let open_result = File::open(path);
        if open_result.is_err() {
            return None;
        }

        let reader = BufReader::new(open_result.unwrap());
        let deserialize_result = serde_json::from_reader(reader);
        if deserialize_result.is_err() {
            return None;
        }
        Some(deserialize_result.unwrap())
    }

    pub fn to_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let create_result = File::create(path)?;
        let buffer = BufWriter::new(create_result);
        serde_json::to_writer_pretty(buffer, &self).unwrap();
        Ok(())
    }
}
