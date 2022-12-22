use std::borrow::Cow;

use crate::core::SimpleMatch;

use super::Mismatch;

/// The matcher for [`contain_substr`].
///
/// [`contain_substr`]: crate::contain_substr
#[derive(Debug)]
pub struct ContainSubstrMatcher<'a> {
    substr: Cow<'a, str>,
}

impl<'a> ContainSubstrMatcher<'a> {
    /// Create a new [`ContainSubstrMatcher`] from the expected substring.
    pub fn new(substr: impl Into<Cow<'a, str>>) -> Self {
        Self {
            substr: substr.into(),
        }
    }
}

impl<'a, Actual> SimpleMatch<Actual> for ContainSubstrMatcher<'a>
where
    Actual: AsRef<str>,
{
    type Fail = Mismatch<Cow<'a, str>, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(actual.as_ref().contains(self.substr.as_ref()))
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            expected: self.substr,
            actual,
        }
    }
}

/// The matcher for [`have_prefix`].
///
/// [`have_prefix`]: crate::have_prefix
#[derive(Debug)]
pub struct HavePrefixMatcher<'a> {
    prefix: Cow<'a, str>,
}

impl<'a> HavePrefixMatcher<'a> {
    /// Create a new [`HavePrefixMatcher`] from the expected string prefix.
    pub fn new(prefix: impl Into<Cow<'a, str>>) -> Self {
        Self {
            prefix: prefix.into(),
        }
    }
}

impl<'a, Actual> SimpleMatch<Actual> for HavePrefixMatcher<'a>
where
    Actual: AsRef<str>,
{
    type Fail = Mismatch<Cow<'a, str>, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(actual.as_ref().starts_with(self.prefix.as_ref()))
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            expected: self.prefix,
            actual,
        }
    }
}

/// The matcher for [`have_suffix`].
///
/// [`have_suffix`]: crate::have_suffix
#[derive(Debug)]
pub struct HaveSuffixMatcher<'a> {
    suffix: Cow<'a, str>,
}

impl<'a> HaveSuffixMatcher<'a> {
    /// Create a new [`HaveSuffixMatcher`] from the expected string prefix.
    pub fn new(suffix: impl Into<Cow<'a, str>>) -> Self {
        Self {
            suffix: suffix.into(),
        }
    }
}

impl<'a, Actual> SimpleMatch<Actual> for HaveSuffixMatcher<'a>
where
    Actual: AsRef<str>,
{
    type Fail = Mismatch<Cow<'a, str>, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok(actual.as_ref().ends_with(self.suffix.as_ref()))
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            expected: self.suffix,
            actual,
        }
    }
}
