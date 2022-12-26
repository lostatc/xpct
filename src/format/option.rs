use std::convert::Infallible;

use crate::core::{Matcher, NegFormat};
use crate::matchers::BeSomeMatcher;

use super::MessageFormat;

fn option_format() -> MessageFormat {
    MessageFormat::new("Expected this to be Some(_)", "Expected this to be None")
}

/// Succeeds when the actual value is [`Some`].
///
/// If this matcher succeeds, it unwraps the [`Some`] value. When negated, it behaves like
/// [`be_none`].
///
/// # Examples
///
/// ```
/// use xpct::{expect, be_some, equal};
///
/// let value = Some("foobar");
///
/// expect!(value)
///     .to(be_some())
///     .to(equal("foobar"));
/// ```
pub fn be_some<'a, T>() -> Matcher<'a, Option<T>, T, Option<Infallible>>
where
    T: 'a,
{
    Matcher::new(BeSomeMatcher::new(), option_format())
}

/// Succeeds when the actual value is [`None`].
///
/// If this matcher succeeds, it converts the value to `Option<Infallible>`. When negated, it
/// behaves like [`be_some`].
///
/// # Examples
///
/// ```
/// use std::convert::Infallible;
/// use xpct::{expect, be_none, equal};
///
/// let value: Option<String> = None;
///
/// let output: Option<Infallible> = expect!(value)
///     .to(be_none())
///     .into_inner();
/// ```
pub fn be_none<'a, T>() -> Matcher<'a, Option<T>, Option<Infallible>, T>
where
    T: 'a,
{
    Matcher::neg(BeSomeMatcher::new(), NegFormat(option_format()))
}

#[cfg(test)]
mod tests {
    use super::{be_none, be_some};
    use crate::expect;

    fn some() -> Option<()> {
        Some(())
    }

    fn none() -> Option<()> {
        None
    }

    #[test]
    fn succeeds_when_some() {
        expect!(some()).to(be_some());
    }

    #[test]
    fn succeeds_when_not_some() {
        expect!(none()).to_not(be_some());
    }

    #[test]
    #[should_panic]
    fn fails_when_some() {
        expect!(some()).to_not(be_some());
    }

    #[test]
    #[should_panic]
    fn fails_when_not_some() {
        expect!(none()).to(be_some());
    }

    #[test]
    fn succeeds_when_none() {
        expect!(none()).to(be_none());
    }

    #[test]
    fn succeeds_when_not_none() {
        expect!(some()).to_not(be_none());
    }

    #[test]
    #[should_panic]
    fn fails_when_none() {
        expect!(none()).to_not(be_none());
    }

    #[test]
    #[should_panic]
    fn fails_when_not_none() {
        expect!(some()).to(be_none());
    }
}
