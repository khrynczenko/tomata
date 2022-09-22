use std::time::Duration;

use druid::Data;
use notify_rust::Notification;

pub const APPLICATION_NAME: &str = "tomata";

pub const WINDOW_SIZE_PX: (f64, f64) = if cfg!(windows) {
    (520., 480.)
} else {
    (520., 460.)
};

pub const SECOND_S: u64 = 1;
pub const MINUTE_S: u64 = SECOND_S * 60;
pub const HOUR_S: u64 = MINUTE_S * 60;

pub static ZERO: Duration = Duration::from_secs(0);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Data)]
pub enum Period {
    Work,
    ShortBreak,
    LongBreak,
}

impl From<Period> for Notification {
    fn from(period: Period) -> Notification {
        match period {
            Period::Work => Notification::new()
                .appname("tomata")
                .summary("Work period.")
                .body("Concentrate on the work you ought to.")
                .clone(),
            Period::ShortBreak => Notification::new()
                .appname("tomata")
                .summary("Short break.")
                .body("Stretch out, calm your mind, look into distance.")
                .clone(),
            Period::LongBreak => Notification::new()
                .appname("tomata")
                .summary("Long break.")
                .body("Take a walk, make a coffee, watch something interesting.")
                .clone(),
        }
    }
}

pub fn duration_to_string(duration: &Duration) -> String {
    let seconds = duration.as_secs();
    format!(
        "{:0>2}:{:0>2}:{:0>2}",
        seconds / HOUR_S,
        (seconds % HOUR_S) / MINUTE_S,
        seconds % MINUTE_S
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_converts_to_string() {
        const ONE_HOUR_THIRTY_MINUTE_TEN_SECONDS_IN_SECONDS: u64 =
            HOUR_S + MINUTE_S * 30 + SECOND_S * 10;
        let duration = Duration::from_secs(ONE_HOUR_THIRTY_MINUTE_TEN_SECONDS_IN_SECONDS);
        let as_string = duration_to_string(&duration);
        assert_eq!(as_string, "01:30:10");
    }
}
