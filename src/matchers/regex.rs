#![cfg(feature = "regex")]

use regex::Regex;

use crate::core::Match;

use super::Mismatch;

/// The matcher for [`match_regex`].
///
/// [`match_regex`]: crate::match_regex
#[derive(Debug)]
pub struct RegexMatcher {
    regex: Result<Regex, regex::Error>,
}

impl RegexMatcher {
    /// Create a new [`RegexMatcher`] from the expected regex.
    ///
    /// If the given regex is invalid, the matcher will return an error.
    pub fn new(regex: impl AsRef<str>) -> Self {
        Self {
            regex: Regex::new(regex.as_ref()),
        }
    }
}

impl<Actual> Match<Actual> for RegexMatcher
where
    Actual: AsRef<str>,
{
    type Fail = Mismatch<String, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        match &self.regex {
            Ok(unwrapped_regex) => Ok(unwrapped_regex.is_match(actual.as_ref())),
            Err(error) => Err(error.clone().into()),
        }
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        // If the regex is invalid, `matches` will be called first, it will return an error, and
        // this method will never be called. If something else is happening, it's a bug.
        let unwrapped_regex = self.regex.expect(
            "The given regex is invalid, but the matcher was never used. This shouldn't happen.",
        );

        Mismatch {
            expected: unwrapped_regex.to_string(),
            actual,
        }
    }
}
