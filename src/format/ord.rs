use std::fmt;

use crate::core::Matcher;
use crate::matchers::{Inequality, OrdMatcher};

use super::MismatchFormat;

/// Succeeds when the actual value is greater than the expected value.
pub fn be_gt<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialOrd<Expected> + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::simple(
        OrdMatcher::new(expected, Inequality::Greater),
        MismatchFormat::new("to be greater than", "to not be greater than"),
    )
}

/// Succeeds when the actual value is greater than or equal to the expected value.
pub fn be_ge<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialOrd<Expected> + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::simple(
        OrdMatcher::new(expected, Inequality::GreaterOrEqual),
        MismatchFormat::new(
            "to be greater than or equal to",
            "to not be greater than or equal to",
        ),
    )
}

/// Succeeds when the actual value is less than the expected value.
pub fn be_lt<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialOrd<Expected> + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::simple(
        OrdMatcher::new(expected, Inequality::Less),
        MismatchFormat::new("to be less than", "to not be less than"),
    )
}

/// Succeeds when the actual value is less than or equal to the expected value.
pub fn be_le<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialOrd<Expected> + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::simple(
        OrdMatcher::new(expected, Inequality::LessOrEqual),
        MismatchFormat::new(
            "to be less than or equal to",
            "to not be less than or equal to",
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::{be_ge, be_gt, be_le, be_lt};
    use crate::expect;

    #[test]
    fn succeeds_when_gt() {
        expect!(1).to(be_gt(0));
    }

    #[test]
    fn succeeds_when_not_gt() {
        expect!(1).to_not(be_gt(1));
    }

    #[test]
    #[should_panic]
    fn fails_when_gt() {
        expect!(1).to_not(be_gt(0));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_gt() {
        expect!(1).to(be_gt(1));
    }

    #[test]
    fn succeeds_when_ge() {
        expect!(1).to(be_ge(1));
    }

    #[test]
    fn succeeds_when_not_ge() {
        expect!(1).to_not(be_ge(2));
    }

    #[test]
    #[should_panic]
    fn fails_when_ge() {
        expect!(1).to_not(be_ge(1));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_ge() {
        expect!(1).to(be_ge(2));
    }

    #[test]
    fn succeeds_when_lt() {
        expect!(1).to(be_lt(2));
    }

    #[test]
    fn succeeds_when_not_lt() {
        expect!(1).to_not(be_lt(1));
    }

    #[test]
    #[should_panic]
    fn fails_when_lt() {
        expect!(1).to_not(be_lt(2));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_lt() {
        expect!(1).to(be_lt(1));
    }

    #[test]
    fn succeeds_when_le() {
        expect!(1).to(be_le(1));
    }

    #[test]
    fn succeeds_when_not_le() {
        expect!(1).to_not(be_le(0));
    }

    #[test]
    #[should_panic]
    fn fails_when_le() {
        expect!(1).to_not(be_le(1));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_le() {
        expect!(1).to(be_le(0));
    }
}
