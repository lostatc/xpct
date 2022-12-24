use std::time::{Duration, SystemTime};

use crate::core::Matcher;
use crate::matchers::ApproxEqTimeMatcher;

use super::MismatchFormat;

/// Succeeds when the actual time approximately equals the expected time.
///
/// This accepts a `threshold` duration and succeeds when the expected time is within that threshold
/// duration of the actual time.
///
/// # Examples
///
/// ```
/// use std::time::{SystemTime, Duration};
/// use xpct::{expect, approx_eq_time};
///
/// let actual = SystemTime::now();
/// let expected = SystemTime::now();
///
/// expect!(actual).to(approx_eq_time(expected, Duration::from_millis(1)));
/// ```
pub fn approx_eq_time<'a>(
    expected: SystemTime,
    threshold: Duration,
) -> Matcher<'a, SystemTime, SystemTime> {
    Matcher::simple(
        ApproxEqTimeMatcher::new(expected, threshold),
        MismatchFormat::new("to approximately equal", "to not approximately equal"),
    )
}

#[cfg(test)]
mod tests {
    use crate::{approx_eq_time, expect};
    use std::time::{Duration, SystemTime};

    fn threshold() -> Duration {
        Duration::from_millis(1)
    }

    fn actual() -> SystemTime {
        SystemTime::UNIX_EPOCH
    }

    fn expected() -> SystemTime {
        SystemTime::UNIX_EPOCH + threshold()
    }

    fn not_expected() -> SystemTime {
        SystemTime::UNIX_EPOCH + (threshold() * 2)
    }

    #[test]
    fn succeeds_when_approx_eq() {
        expect!(actual()).to(approx_eq_time(expected(), threshold()));
    }

    #[test]
    fn succeeds_when_not_approx_eq() {
        expect!(actual()).to_not(approx_eq_time(not_expected(), threshold()));
    }

    #[test]
    #[should_panic]
    fn fails_when_approx_eq() {
        expect!(actual()).to_not(approx_eq_time(expected(), threshold()));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_approx_eq() {
        expect!(actual()).to(approx_eq_time(not_expected(), threshold()));
    }
}
