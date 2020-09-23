use std::time::Duration;

use druid::Data;

pub const SECOND_S: u64 = 1;
pub const MINUTE_S: u64 = SECOND_S * 60;
pub const HOUR_S: u64 = MINUTE_S * 60;

pub static ZERO: Duration = Duration::from_secs(0);

#[derive(Debug, Copy, Clone, PartialEq, Data)]
pub enum Period {
    WorkPeriod,
    ShortBreak,
    LongBreak,
}

pub(crate) fn duration_to_string(duration: &Duration) -> String {
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
