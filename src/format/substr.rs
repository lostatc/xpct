use std::borrow::Cow;
use std::fmt;

use crate::core::Matcher;
use crate::matchers::{ContainSubstrMatcher, HavePrefixMatcher};

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

#[cfg(test)]
mod tests {
    use super::{contain_substr, have_prefix};
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
}
