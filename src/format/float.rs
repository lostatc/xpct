#![cfg(feature = "float")]

use crate::core::Matcher;
use crate::matchers::numbers::ApproxEqFloatMatcher;

use super::MismatchFormat;

/// Succeeds when the actual `f32` value approximately equals the expected `f32` value.
///
/// To test two floating-point numbers for equality, you must specify the margin of acceptable error
/// as a number of ULPs, which stands for [Units of Least
/// Precisions](https://en.wikipedia.org/wiki/Unit_in_the_last_place) or Units in the Last Place.
///
/// # Examples
///
/// ```
/// use xpct::{expect, approx_eq_f32};
///
/// let actual: f32 = 0.15 + 0.15 + 0.15;
/// let expected: f32 = 0.1 + 0.1 + 0.25;
///
/// expect!(actual).to(approx_eq_f32(expected, 2));
/// ```
pub fn approx_eq_f32<'a>(expected: f32, ulps: i32) -> Matcher<'a, f32, f32> {
    Matcher::new(
        ApproxEqFloatMatcher::<f32, i32>::new(expected, ulps),
        MismatchFormat::new("to approximately equal", "to not approximately equal"),
    )
}

/// Succeeds when the actual `f64` value approximately equals the expected `f64` value.
///
/// See [`approx_eq_f32`] for details.
pub fn approx_eq_f64<'a>(expected: f64, ulps: i64) -> Matcher<'a, f64, f64> {
    Matcher::new(
        ApproxEqFloatMatcher::<f64, i64>::new(expected, ulps),
        MismatchFormat::new("to approximately equal", "to not approximately equal"),
    )
}

#[cfg(test)]
mod tests {
    use super::approx_eq_f32;
    use crate::expect;

    fn actual() -> f32 {
        0.15 + 0.15 + 0.15
    }

    fn expected() -> f32 {
        0.1 + 0.1 + 0.25
    }

    #[test]
    fn succeeds_when_approx_eq() {
        expect!(actual()).to(approx_eq_f32(expected(), 2));
    }

    #[test]
    fn succeeds_when_not_approx_eq() {
        expect!(actual()).to_not(approx_eq_f32(expected(), 0));
    }

    #[test]
    #[should_panic]
    fn fails_when_approx_eq() {
        expect!(actual()).to_not(approx_eq_f32(expected(), 2));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_approx_eq() {
        expect!(actual()).to(approx_eq_f32(expected(), 0));
    }
}
