#![cfg(feature = "regex")]

use std::fmt;

use crate::core::Matcher;
use crate::matchers::MatchRegexMatcher;

use super::MismatchFormat;

/// Succeeds when the actual value matches the given regular expression.
///
/// This succeeds if the actual string contains any match for the given regex. If you want to test
/// that the regex matches the *entire* string, you should use the `^` and `$` anchors.
///
/// The regex engine does not support lookarounds or backreferences.
///
/// # Examples
///
/// ```
/// use xpct::{expect, match_regex};
///
/// expect!("foobar").to(match_regex(r"^foo\w*$"));
/// ```
pub fn match_regex<'a, Actual>(regex: &str) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + AsRef<str> + 'a,
{
    Matcher::simple(
        MatchRegexMatcher::new(regex),
        MismatchFormat::new("to match the regex", "to not match the regex"),
    )
}

#[cfg(test)]
mod tests {
    use super::match_regex;
    use crate::expect;

    #[test]
    fn succeeds_when_matches_regex() {
        expect!("foobar").to(match_regex("^foo"));
    }

    #[test]
    fn succeeds_when_not_matches_regex() {
        expect!("foobar").to_not(match_regex("^a regex that does not match$"));
    }

    #[test]
    #[should_panic]
    fn fails_when_matches_regex() {
        expect!("foobar").to_not(match_regex("^foo"));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_matches_regex() {
        expect!("foobar").to(match_regex("^a regex that does not match$"));
    }

    #[test]
    #[should_panic]
    fn fails_when_regex_is_invalid() {
        expect!("foobar").to(match_regex("[an invalid regex"));
    }
}
