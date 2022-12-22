use std::borrow::Cow;
use std::fmt;

use crate::core::Matcher;
use crate::matchers::ContainSubstrMatcher;

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

#[cfg(test)]
mod tests {
    use super::contain_substr;
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
    #[should_panic]
    fn fails_when_contains_substr() {
        expect!("foobar").to_not(contain_substr("ooba"));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_contains_substr() {
        expect!("foobar").to(contain_substr("not a substring"));
    }
}
