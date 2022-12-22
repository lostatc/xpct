use std::borrow::Cow;
use std::fmt;

use crate::core::Matcher;
use crate::matchers::{ContainSubstrMatcher, HavePrefixMatcher, HaveSuffixMatcher};

use super::MismatchFormat;

/// Succeeds when the actual value contains the expected substring.
///
/// # Examples
///
/// ```
/// use xpct::{expect, contain_substr};
///
/// expect!("foobar").to(contain_substr("ooba"));
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn contain_substr<'a, Actual>(substr: impl Into<Cow<'a, str>>) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + AsRef<str> + 'a,
{
    Matcher::simple(
        ContainSubstrMatcher::new(substr),
        MismatchFormat::new("to contain the substring", "to not contain the substring"),
    )
}

/// Succeeds when the actual value has the expected prefix.
///
/// # Examples
///
/// ```
/// use xpct::{expect, have_prefix};
///
/// expect!("foobar").to(have_prefix("foo"));
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn have_prefix<'a, Actual>(prefix: impl Into<Cow<'a, str>>) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + AsRef<str> + 'a,
{
    Matcher::simple(
        HavePrefixMatcher::new(prefix),
        MismatchFormat::new("to have the prefix", "to not have the prefix"),
    )
}

/// Succeeds when the actual value has the expected suffix.
///
/// # Examples
///
/// ```
/// use xpct::{expect, have_suffix};
///
/// expect!("foobar").to(have_suffix("bar"));
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn have_suffix<'a, Actual>(suffix: impl Into<Cow<'a, str>>) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + AsRef<str> + 'a,
{
    Matcher::simple(
        HaveSuffixMatcher::new(suffix),
        MismatchFormat::new("to have the suffix", "to not have the suffix"),
    )
}

#[cfg(test)]
mod tests {
    use super::{contain_substr, have_prefix, have_suffix};
    use crate::expect;

    #[test]
    fn succeeds_when_contains_substr() {
        expect!("foobar").to(contain_substr("ooba"));
    }

    #[test]
    fn succeeds_when_not_contains_substr() {
        expect!("foobar").to_not(contain_substr("not a substring"));
    }

    #[test]
    fn succeeds_when_has_prefix() {
        expect!("foobar").to(have_prefix("foo"));
    }

    #[test]
    fn succeeds_when_not_has_prefix() {
        expect!("foobar").to_not(have_prefix("not a prefix"));
    }

    #[test]
    fn succeeds_when_has_suffix() {
        expect!("foobar").to(have_suffix("bar"));
    }

    #[test]
    fn succceeds_when_not_has_suffix() {
        expect!("foobar").to_not(have_suffix("not a suffix"));
    }

    #[test]
    #[should_panic]
    fn fails_when_contains_substr() {
        expect!("foobar").to_not(contain_substr("ooba"));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_contains_substr() {
        expect!("foobar").to(contain_substr("not a substring"));
    }

    #[test]
    #[should_panic]
    fn fails_when_has_prefix() {
        expect!("foobar").to_not(have_prefix("foo"));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_has_prefix() {
        expect!("foobar").to(have_prefix("not a prefix"));
    }

    #[test]
    #[should_panic]
    fn fails_when_has_suffix() {
        expect!("foobar").to_not(have_suffix("bar"));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_has_suffix() {
        expect!("foobar").to(have_suffix("not a suffix"));
    }
}
