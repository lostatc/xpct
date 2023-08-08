use crate::core::FormattedFailure;

/// A value that is returned by matchers that compose a list of other matchers.
///
/// This represents the formatted failure output of each element in the list.
///
/// This type is used by matchers like [`any`] and [`each`] that compose other matchers.
///
/// [`any`]: crate::any
/// [`each`]: crate::each
pub type SomeFailures = Vec<Option<FormattedFailure>>;

/// A value that is returned by matchers when the expected and actual values differ.
///
/// This is meant to be a deliberately generic value that can be reused in a number of different
/// matchers and formatted with [`MismatchFormat`].
///
/// This can be used for matchers like [`equal`] to represent two values being not equal, but it can
/// also be used for matchers like [`be_lt`] or [`be_ge`] to represent different kinds of
/// relationships.
///
/// Use this over [`Expectation`] when you want to record the "expected" value alongside the actual
/// value.
///
/// When returned by a matcher, this value just means, "here is the expected value and here is the
/// actual value." It's up to the formatter to determine how that information is presented to the
/// user.
///
/// [`MismatchFormat`]: crate::format::MismatchFormat
/// [`Expectation`]: crate::matchers::Expectation
/// [`equal`]: crate::equal
/// [`be_lt`]: crate::be_lt
/// [`be_ge`]: crate::be_ge
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mismatch<Expected, Actual> {
    /// The expected value.
    pub expected: Expected,

    /// The actual value.
    pub actual: Actual,
}

/// A value that is returned by matchers that compose other matchers by struct field.
///
/// This pairs struct field names to their formatted failure output.
///
/// This type is used by the [`match_fields`] and [`match_any_fields`] matchers.
///
/// [`match_fields`]: crate::match_fields
/// [`match_any_fields`]: crate::match_any_fields
pub type FailuresByField = Vec<(&'static str, Option<FormattedFailure>)>;

/// A value that is returned by matchers when the actual value doesn't meet some criteria.
///
/// This is meant to be a deliberately generic value that can be reused in a number of different
/// matchers and formatted with [`ExpectationFormat`].
///
/// This can be used for matchers like [`be_some`] and [`be_ok`] to represent a value not meeting
/// some criteria, such as not being `Some(_)` and not being `Ok(_)` respectively.
///
/// Use this over [`Mismatch`] when there's only one case in which the matcher could fail.
///
/// When returned by a matcher, this value just means, "here is the actual value." It's up to the
/// formatter to determine how that information is presented to the user.
///
/// [`ExpectationFormat`]: crate::format::ExpectationFormat
/// [`Mismatch`]: crate::matchers::Mismatch
/// [`be_some`]: crate::be_some
/// [`be_ok`]: crate::be_ok
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expectation<Actual> {
    /// The actual value.
    pub actual: Actual,
}
