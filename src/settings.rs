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

const TWENTY_FIVE_MINUTES: u64 = MINUTE_S * 25;
const FIVE_MINUTES: u64 = MINUTE_S * 5;
const EIGHT_MINUTES: u64 = MINUTE_S * 8;
const DEFAULT_SHORT_BREAKS_BEFORE_LONG_BREAK: usize = 3;

/// Represents all the settings for the application, these are ought be written/read
/// from a file by means of serialization/deserialization. Most of the settings
/// can be changed by the user.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Data, Lens)]
pub struct Settings {
    work_period: Rc<Duration>, // Data cannot be derive fo Duration, unless it is in Rc
    short_break_period: Rc<Duration>, // Data cannot be derive fo Duration, unless it is in Rc
    long_break_period: Rc<Duration>, // Data cannot be derive fo Duration, unless it is in Rc
    short_breaks_number: usize,
    long_breaks_are_included: bool,
    next_period_starts_automatically: bool,
    system_notifications_are_enabled: bool,
    period_ending_sound_is_enabled: bool,
    beep_volume: f64,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            work_period: Rc::new(Duration::from_secs(TWENTY_FIVE_MINUTES)),
            short_break_period: Rc::new(Duration::from_secs(FIVE_MINUTES)),
            long_break_period: Rc::new(Duration::from_secs(EIGHT_MINUTES)),
            short_breaks_number: DEFAULT_SHORT_BREAKS_BEFORE_LONG_BREAK,
            long_breaks_are_included: true,
            next_period_starts_automatically: false,
            system_notifications_are_enabled: true,
            period_ending_sound_is_enabled: true,
            beep_volume: 0.1,
        }
    }
}

impl Settings {
    #[allow(dead_code)] // used in tests
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        work_period: Duration,
        short_break_period: Duration,
        long_break_period: Duration,
        short_breaks_number: usize,
        long_breaks_are_included: bool,
        next_period_starts_automatically: bool,
        system_notifications_are_enabled: bool,
        period_ending_sound_is_enabled: bool,
        beep_volume: f64,
    ) -> Settings {
        Settings {
            work_period: Rc::new(work_period),
            short_break_period: Rc::new(short_break_period),
            long_break_period: Rc::new(long_break_period),
            short_breaks_number,
            long_breaks_are_included,
            next_period_starts_automatically,
            system_notifications_are_enabled,
            period_ending_sound_is_enabled,
            beep_volume,
        }
    }

    pub fn increase_period_duration(&mut self, period: Period, value: Duration) {
        match period {
            Period::Work => self.work_period = Rc::new(*self.work_period + value),
            Period::ShortBreak => {
                self.short_break_period = Rc::new(*self.short_break_period + value)
            }
            Period::LongBreak => self.long_break_period = Rc::new(*self.long_break_period + value),
        }
    }

    pub fn decrease_period_duration(&mut self, period: Period, value: Duration) {
        match period {
            Period::Work => {
                let current_period_duration = &self.work_period;
                if value > **current_period_duration {
                    self.work_period = Rc::new(Duration::from_secs(0));
                } else {
                    self.work_period = Rc::new(*self.work_period - value)
                }
            }
            Period::ShortBreak => {
                let current_period_duration = &self.short_break_period;
                if value > **current_period_duration {
                    self.short_break_period = Rc::new(Duration::from_secs(0));
                } else {
                    self.short_break_period = Rc::new(*self.short_break_period - value)
                }
            }
            Period::LongBreak => {
                let current_period_duration = &self.long_break_period;
                if value > **current_period_duration {
                    self.long_break_period = Rc::new(Duration::from_secs(0));
                } else {
                    self.long_break_period = Rc::new(*self.long_break_period - value)
                }
            }
        }
    }

    pub fn get_short_breaks_number(&self) -> usize {
        self.short_breaks_number
    }

    pub fn increase_short_breaks_number(&mut self, value: usize) {
        self.short_breaks_number += value;
    }

    pub fn decrease_short_breaks_number(&mut self, value: usize) {
        if value > self.short_breaks_number {
            self.short_breaks_number = 0;
            return;
        }
        self.short_breaks_number -= value;
    }

    pub fn are_long_breaks_included(&self) -> bool {
        self.long_breaks_are_included
    }

    pub fn does_next_period_start_automatically(&self) -> bool {
        self.next_period_starts_automatically
    }

    pub fn are_system_notifications_enabled(&self) -> bool {
        self.system_notifications_are_enabled
    }

    pub fn is_period_ending_sound_enabled(&self) -> bool {
        self.period_ending_sound_is_enabled
    }

    pub fn convert_period_to_duration(&self, period: Period) -> Duration {
        match period {
            Period::Work => *self.work_period,
            Period::ShortBreak => *self.short_break_period,
            Period::LongBreak => *self.long_break_period,
        }
    }

    pub fn get_beep_volume(&self) -> f32 {
        // This function returns f32 because that is what slider widgets
        // works with. I decided that making a Lens for the f64 is not
        // worth the effort.
        self.beep_volume as f32
    }
}

pub fn load_settings_from_file(path: impl AsRef<Path>) -> Option<Settings> {
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

pub fn save_settings_to_file(settings: &Settings, path: impl AsRef<Path>) -> io::Result<()> {
    let create_result = File::create(path)?;
    let buffer = BufWriter::new(create_result);
    serde_json::to_writer_pretty(buffer, settings).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increasing_work_period_duration() {
        let mut settings = Settings::default();
        let pre_change = *settings.work_period;
        settings.increase_period_duration(Period::Work, Duration::from_secs(10));
        let post_change = *settings.work_period;
        assert_eq!(pre_change + Duration::from_secs(10), post_change);
    }

    #[test]
    fn decreasing_work_period_duration() {
        let mut settings = Settings::default();
        let pre_change = *settings.work_period;
        settings.decrease_period_duration(Period::Work, Duration::from_secs(10));
        let post_change = *settings.work_period;
        assert_eq!(pre_change - Duration::from_secs(10), post_change);
    }

    #[test]
    fn increasing_short_break_period_duration() {
        let mut settings = Settings::default();
        let pre_change = *settings.short_break_period;
        settings.increase_period_duration(Period::ShortBreak, Duration::from_secs(10));
        let post_change = *settings.short_break_period;
        assert_eq!(pre_change + Duration::from_secs(10), post_change);
    }

    #[test]
    fn decreasing_short_break_period_duration() {
        let mut settings = Settings::default();
        let pre_change = *settings.short_break_period;
        settings.decrease_period_duration(Period::ShortBreak, Duration::from_secs(10));
        let post_change = *settings.short_break_period;
        assert_eq!(pre_change - Duration::from_secs(10), post_change);
    }

    #[test]
    fn increasing_long_break_period_duration() {
        let mut settings = Settings::default();
        let pre_change = *settings.long_break_period;
        settings.increase_period_duration(Period::LongBreak, Duration::from_secs(10));
        let post_change = *settings.long_break_period;
        assert_eq!(pre_change + Duration::from_secs(10), post_change);
    }

    #[test]
    fn decreasing_long_break_period_duration() {
        let mut settings = Settings::default();
        let pre_change = *settings.long_break_period;
        settings.decrease_period_duration(Period::LongBreak, Duration::from_secs(10));
        let post_change = *settings.long_break_period;
        assert_eq!(pre_change - Duration::from_secs(10), post_change);
    }

    #[test]
    fn decreasing_period_duration_below_zero() {
        let mut settings = Settings::default();
        settings.long_break_period = Rc::new(Duration::from_secs(1));
        settings.decrease_period_duration(Period::LongBreak, Duration::from_secs(10));
        let post_change = *settings.long_break_period;
        assert_eq!(Duration::from_secs(0), post_change);
    }

    #[test]
    fn getting_short_breaks_number() {
        let mut settings = Settings::default();
        settings.short_breaks_number = 2;
        assert_eq!(2, settings.get_short_breaks_number());
    }

    #[test]
    fn increasing_short_breaks_number() {
        let mut settings = Settings::default();
        let pre_change = settings.short_breaks_number;
        settings.increase_short_breaks_number(1);
        assert_eq!(pre_change + 1, settings.short_breaks_number);
        settings.increase_short_breaks_number(1);
        assert_eq!(pre_change + 2, settings.short_breaks_number);
    }

    #[test]
    fn descreasing_short_breaks_number() {
        let mut settings = Settings::default();
        settings.short_breaks_number = 1;
        settings.decrease_short_breaks_number(1);
        assert_eq!(0, settings.short_breaks_number);
        settings.decrease_short_breaks_number(1);
        assert_eq!(0, settings.short_breaks_number);
    }

    #[test]
    fn checking_if_long_breaks_are_active() {
        let settings = Settings::default();
        let actual = settings.long_breaks_are_included;
        assert_eq!(actual, settings.are_long_breaks_included());
    }

    #[test]
    fn checking_if_period_ending_sound_is_enabled() {
        let settings = Settings::default();
        let actual = settings.period_ending_sound_is_enabled;
        assert_eq!(actual, settings.is_period_ending_sound_enabled());
    }
}
