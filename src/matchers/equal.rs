use crate::core::SimpleMatch;

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

/// The matcher for [`equal`].
///
/// [`equal`]: crate::equal
#[derive(Debug)]
pub struct EqualMatcher<Expected> {
    expected: Expected,
}

impl<Expected> EqualMatcher<Expected> {
    /// Create a new [`EqualMatcher`] from the expected value.
    pub fn new(expected: Expected) -> Self {
        Self { expected }
    }
}

impl<Expected, Actual> SimpleMatch<Actual> for EqualMatcher<Expected>
where
    Actual: PartialEq<Expected> + Eq,
{
    type Fail = Mismatch<Expected, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(actual == &self.expected)
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            actual,
            expected: self.expected,
        }
    }
}
