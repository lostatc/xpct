use std::cmp::Ordering;
use std::fmt;

use crate::core::Matcher;
use crate::matchers::ord::{BeSortedByMatcher, BeSortedMatcher, Inequality, OrdMatcher, SortOrder};

use super::{ExpectationFormat, MismatchFormat};

/// Succeeds when the actual value is greater than the expected value.
///
/// # Examples
///
/// ```
/// use xpct::{expect, be_gt};
///
/// expect!(57).to(be_gt(41));
/// ```
pub fn be_gt<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialOrd<Expected> + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::new(
        OrdMatcher::new(expected, Inequality::Greater),
        MismatchFormat::new("to be greater than", "to not be greater than"),
    )
}

/// Succeeds when the actual value is greater than or equal to the expected value.
///
/// # Examples
///
/// ```
/// use xpct::{expect, be_ge};
///
/// expect!(57).to(be_ge(41));
/// expect!(57).to(be_ge(57));
/// ```
pub fn be_ge<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialOrd<Expected> + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::new(
        OrdMatcher::new(expected, Inequality::GreaterOrEqual),
        MismatchFormat::new(
            "to be greater than or equal to",
            "to not be greater than or equal to",
        ),
    )
}

/// Succeeds when the actual value is less than the expected value.
///
/// # Examples
///
/// ```
/// use xpct::{expect, be_lt};
///
/// expect!(41).to(be_lt(57));
/// ```
pub fn be_lt<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialOrd<Expected> + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::new(
        OrdMatcher::new(expected, Inequality::Less),
        MismatchFormat::new("to be less than", "to not be less than"),
    )
}

/// Succeeds when the actual value is less than or equal to the expected value.
///
/// # Examples
///
/// ```
/// use xpct::{expect, be_le};
///
/// expect!(41).to(be_le(57));
/// expect!(41).to(be_le(41));
/// ```
pub fn be_le<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialOrd<Expected> + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::new(
        OrdMatcher::new(expected, Inequality::LessOrEqual),
        MismatchFormat::new(
            "to be less than or equal to",
            "to not be less than or equal to",
        ),
    )
}

/// Succeeds when the actual value is sorted in ascending order.
///
/// # Examples
///
/// ```
/// use xpct::{expect, be_sorted_asc};
///
/// expect!(vec!["a", "b", "c"]).to(be_sorted_asc());
/// ```
pub fn be_sorted_asc<'a, T, Actual>() -> Matcher<'a, Actual, Actual>
where
    T: Ord + 'a,
    Actual: fmt::Debug + AsRef<[T]> + 'a,
{
    Matcher::new(
        BeSortedMatcher::new(SortOrder::Asc),
        ExpectationFormat::new(
            "to be sorted in ascending order",
            "to not be sorted in ascending order",
        ),
    )
}

/// Succeeds when the actual value is sorted in descending order.
///
/// # Examples
///
/// ```
/// use xpct::{expect, be_sorted_desc};
///
/// expect!(vec!["c", "b", "a"]).to(be_sorted_desc());
/// ```
pub fn be_sorted_desc<'a, T, Actual>() -> Matcher<'a, Actual, Actual>
where
    T: Ord + 'a,
    Actual: fmt::Debug + AsRef<[T]> + 'a,
{
    Matcher::new(
        BeSortedMatcher::new(SortOrder::Desc),
        ExpectationFormat::new(
            "to be sorted in descending order",
            "to not be sorted in descending order",
        ),
    )
}

/// Succeeds when the actual value is sorted according to the given predicate.
///
/// # Examples
///
/// ```
/// use xpct::{be_sorted_by, expect};
///
/// expect!(vec!["a", "B", "c"]).to(be_sorted_by::<&str, _>(|a, b| {
///     a.to_lowercase().cmp(&b.to_lowercase())
/// }));
/// ```
pub fn be_sorted_by<'a, T, Actual>(
    predicate: impl Fn(&T, &T) -> Ordering + 'a,
) -> Matcher<'a, Actual, Actual>
where
    T: Ord + 'a,
    Actual: fmt::Debug + AsRef<[T]> + 'a,
{
    Matcher::new(
        BeSortedByMatcher::new(predicate),
        ExpectationFormat::new(
            "to be sorted by the given predicate",
            "to not be sorted by the given predicate",
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::{be_ge, be_gt, be_le, be_lt, be_sorted_asc, be_sorted_by, be_sorted_desc};
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

    #[test]
    fn succeeds_when_sorted_asc() {
        expect!(["a", "b", "c"]).to(be_sorted_asc());
    }

    #[test]
    fn succeeds_when_not_sorted_asc() {
        expect!(["a", "c", "b"]).to_not(be_sorted_asc());
    }

    #[test]
    #[should_panic]
    fn fails_when_sorted_asc() {
        expect!(["a", "b", "c"]).to_not(be_sorted_asc());
    }

    #[test]
    #[should_panic]
    fn fails_when_not_sorted_asc() {
        expect!(["a", "c", "b"]).to(be_sorted_asc());
    }

    #[test]
    fn succeeds_when_sorted_desc() {
        expect!(["c", "b", "a"]).to(be_sorted_desc());
    }

    #[test]
    fn succeeds_when_not_sorted_desc() {
        expect!(["c", "a", "b"]).to_not(be_sorted_desc());
    }

    #[test]
    #[should_panic]
    fn fails_when_sorted_desc() {
        expect!(["c", "b", "a"]).to_not(be_sorted_desc());
    }

    #[test]
    #[should_panic]
    fn fails_when_not_sorted_desc() {
        expect!(["c", "a", "b"]).to(be_sorted_desc());
    }

    #[test]
    fn succeeds_when_sorted_by() {
        expect!(["a", "B", "c"]).to(be_sorted_by::<&str, _>(|a, b| {
            a.to_lowercase().cmp(&b.to_lowercase())
        }));
    }

    #[test]
    fn succeeds_when_not_sorted_by() {
        expect!(["c", "B", "a"]).to_not(be_sorted_by::<&str, _>(|a, b| {
            a.to_lowercase().cmp(&b.to_lowercase())
        }));
    }

    #[test]
    #[should_panic]
    fn fails_when_sorted_by() {
        expect!(["a", "B", "c"]).to_not(be_sorted_by::<&str, _>(|a, b| {
            a.to_lowercase().cmp(&b.to_lowercase())
        }));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_sorted_by() {
        expect!(["c", "B", "a"]).to(be_sorted_by::<&str, _>(|a, b| {
            a.to_lowercase().cmp(&b.to_lowercase())
        }));
    }
}
