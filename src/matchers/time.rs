use std::time::{Duration, SystemTime};

use crate::core::SimpleMatch;

use super::Mismatch;

/// The matcher for [`approx_eq_time`].
///
/// [`approx_eq_time`]: crate::approx_eq_time
#[derive(Debug)]
pub struct ApproxEqTimeMatcher {
    expected: SystemTime,
    threshold: Duration,
}

impl ApproxEqTimeMatcher {
    /// Create a new [`ApproxEqTimeMatcher`] from the expected time and threshold duration.
    pub fn new(expected: SystemTime, threshold: Duration) -> Self {
        Self {
            expected,
            threshold,
        }
    }
}

impl SimpleMatch<SystemTime> for ApproxEqTimeMatcher {
    type Fail = Mismatch<SystemTime, SystemTime>;

    fn matches(&mut self, actual: &SystemTime) -> crate::Result<bool> {
        match actual.cmp(&self.expected) {
            std::cmp::Ordering::Less => {
                Ok(self.expected.duration_since(*actual)? <= self.threshold)
            }
            std::cmp::Ordering::Equal => Ok(true),
            std::cmp::Ordering::Greater => {
                Ok(actual.duration_since(self.expected)? <= self.threshold)
            }
        }
    }

    fn fail(self, actual: SystemTime) -> Self::Fail {
        Mismatch {
            expected: self.expected,
            actual,
        }
    }
}
