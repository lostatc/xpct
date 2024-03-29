use std::fmt;

use crate::core::{Matcher, NegFormat};
use crate::matchers::result::BeOkMatcher;

use super::ExpectationFormat;

fn result_format<T, E>() -> ExpectationFormat<Result<T, E>> {
    ExpectationFormat::new("to be Ok(_)", "to be Err(_)")
}

/// Succeeds when the actual value is [`Ok`].
///
/// If this matcher succeeds, it unwraps the [`Ok`] value. When negated, it behaves like
/// [`be_err`].
///
/// # Examples
///
/// ```
/// use std::io;
/// use xpct::{expect, equal, be_ok};
///
/// fn might_fail() -> io::Result<String> {
///     Ok(String::from("foobar"))
/// }
///
/// expect!(might_fail())
///     .to(be_ok())
///     .to(equal("foobar"));
/// ```
pub fn be_ok<'a, T, E>() -> Matcher<'a, Result<T, E>, T, E>
where
    T: fmt::Debug + 'a,
    E: fmt::Debug + 'a,
{
    Matcher::transform(BeOkMatcher::new(), result_format())
}

/// Succeeds when the actual value is [`Err`].
///
/// If this matcher succeeds, it unwraps the [`Err`] value. When negated, it behaves like
/// [`be_ok`].
///
/// # Examples
///
/// ```
/// use std::io;
/// use xpct::{expect, equal, be_err};
///
/// fn might_fail() -> io::Result<()> {
///     Err(io::Error::new(io::ErrorKind::Other, "something bad happened"))
/// }
///
/// expect!(might_fail())
///     .to(be_err())
///     .map(|err| err.kind())
///     .to(equal(io::ErrorKind::Other));
/// ```
pub fn be_err<'a, T, E>() -> Matcher<'a, Result<T, E>, E, T>
where
    T: fmt::Debug + 'a,
    E: fmt::Debug + 'a,
{
    Matcher::transform_neg(BeOkMatcher::new(), NegFormat(result_format()))
}

#[cfg(test)]
mod tests {
    use super::{be_err, be_ok};
    use crate::expect;

    fn ok() -> Result<(), ()> {
        Ok(())
    }

    fn err() -> Result<(), ()> {
        Err(())
    }

    #[test]
    fn succeeds_when_ok() {
        expect!(ok()).to(be_ok());
    }

    #[test]
    fn succeeds_when_not_ok() {
        expect!(err()).to_not(be_ok());
    }

    #[test]
    #[should_panic]
    fn fails_when_ok() {
        expect!(ok()).to_not(be_ok());
    }

    #[test]
    #[should_panic]
    fn fails_when_not_ok() {
        expect!(err()).to(be_ok());
    }

    #[test]
    fn succeeds_when_err() {
        expect!(err()).to(be_err());
    }

    #[test]
    fn succeeds_when_not_err() {
        expect!(ok()).to_not(be_err());
    }

    #[test]
    #[should_panic]
    fn fails_when_err() {
        expect!(err()).to_not(be_err());
    }

    #[test]
    #[should_panic]
    fn fails_when_not_err() {
        expect!(ok()).to(be_err());
    }
}
