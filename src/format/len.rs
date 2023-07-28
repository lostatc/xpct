use std::fmt;

use crate::core::Matcher;
use crate::matchers::{BeEmptyMatcher, HaveLenMatcher, Len};

use super::{ExpectationFormat, MismatchFormat};

/// Succeeds when the actual value has the given length.
///
/// You can use this matcher for your own types by implementing [`Len`] on them.
///
/// # Examples
///
/// ```
/// use xpct::{expect, have_len};
///
/// expect!("foo").to(have_len(3));
/// expect!(&vec!["bar"]).to(have_len(1));
/// ```
pub fn have_len<'a, Actual>(len: usize) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + Len + 'a,
{
    Matcher::simple(
        HaveLenMatcher::new(len),
        MismatchFormat::new("to have length", "to not have length"),
    )
}

/// Succeeds when the actual value is empty.
///
/// You can use this matcher for your own types by implementing [`Len`] on them.
///
/// # Examples
///
/// ```
/// use xpct::{expect, be_empty};
///
/// expect!("").to(be_empty());
/// expect!(&Vec::<()>::new()).to(be_empty());
/// ```
pub fn be_empty<'a, Actual>() -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + Len + 'a,
{
    Matcher::simple(
        BeEmptyMatcher::new(),
        ExpectationFormat::new("to be empty", "to not be empty"),
    )
}

#[cfg(test)]
mod tests {
    use super::{be_empty, have_len};
    use crate::expect;

    #[test]
    fn succeeds_when_has_len() {
        expect!(&vec!["foo"]).to(have_len(1));
    }

    #[test]
    fn succeeds_when_not_has_len() {
        expect!(&vec!["foo"]).to_not(have_len(100));
    }

    #[test]
    #[should_panic]
    fn fails_when_has_len() {
        expect!(&vec!["foo"]).to_not(have_len(1));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_has_len() {
        expect!(&vec!["foo"]).to(have_len(100));
    }

    #[test]
    fn succeeds_when_is_empty() {
        expect!(&Vec::<&'static str>::new()).to(be_empty());
    }

    #[test]
    fn succeeds_when_not_empty() {
        expect!(&vec!["foo"]).to_not(be_empty());
    }

    #[test]
    #[should_panic]
    fn fails_when_is_empty() {
        expect!(&Vec::<&'static str>::new()).to_not(be_empty());
    }

    #[test]
    #[should_panic]
    fn fails_when_not_empty() {
        expect!(&vec!["foo"]).to(be_empty());
    }
}
