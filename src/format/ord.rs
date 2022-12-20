use std::fmt;

use crate::core::Matcher;
use crate::matchers::{Inequality, OrdMatcher};

use super::MismatchFormat;

/// Succeeds when the actual value is greater than the expected value.
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
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
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
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
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
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
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
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
