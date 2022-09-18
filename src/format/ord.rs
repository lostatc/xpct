use std::fmt;

use crate::core::Matcher;
use crate::matchers::{Inequality, OrdMatcher};

use super::MismatchFormat;

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
