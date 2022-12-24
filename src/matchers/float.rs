#![cfg(feature = "float-cmp")]

use float_cmp::approx_eq;

use crate::core::SimpleMatch;

use super::Mismatch;

/// The matcher for [`approx_eq_f32`] and [`approx_eq_f64`].
///
/// [`approx_eq_f32`]: crate::approx_eq_f32
/// [`approx_eq_f64`]: crate::approx_eq_f64
#[derive(Debug)]
pub struct ApproxEqFloatMatcher<Float, Ulps> {
    expected: Float,
    ulps: Ulps,
}

impl ApproxEqFloatMatcher<f32, i32> {
    /// Create a new [`ApproxEqFloatMatcher`] from the expected value and number of ULPs.
    pub fn new(expected: f32, ulps: i32) -> Self {
        Self { expected, ulps }
    }
}

impl ApproxEqFloatMatcher<f64, i64> {
    /// Create a new [`ApproxEqFloatMatcher`] from the expected value and number of ULPs.
    pub fn new(expected: f64, ulps: i64) -> Self {
        Self { expected, ulps }
    }
}

impl SimpleMatch<f32> for ApproxEqFloatMatcher<f32, i32> {
    type Fail = Mismatch<f32, f32>;

    fn matches(&mut self, actual: &f32) -> crate::Result<bool> {
        Ok(approx_eq!(f32, *actual, self.expected, ulps = self.ulps))
    }

    fn fail(self, actual: f32) -> Self::Fail {
        Mismatch {
            expected: self.expected,
            actual,
        }
    }
}

impl SimpleMatch<f64> for ApproxEqFloatMatcher<f64, i64> {
    type Fail = Mismatch<f64, f64>;

    fn matches(&mut self, actual: &f64) -> crate::Result<bool> {
        Ok(approx_eq!(f64, *actual, self.expected, ulps = self.ulps))
    }

    fn fail(self, actual: f64) -> Self::Fail {
        Mismatch {
            expected: self.expected,
            actual,
        }
    }
}
