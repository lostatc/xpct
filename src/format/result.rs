use crate::{
    core::{Matcher, NegFormat},
    matchers::BeOkMatcher,
};

use super::MessageFormat;

fn result_format() -> MessageFormat {
    MessageFormat::new("Expected this to be Ok(_)", "Expected this to be Err(_)")
}

/// Succeeds when the actual value is [`Ok`].
///
/// If this matcher succeeds, it unwraps the [`Ok`] value. When negated, it behaves like
/// [`be_err`].
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn be_ok<'a, T, E>() -> Matcher<'a, Result<T, E>, T, E>
where
    T: 'a,
    E: 'a,
{
    Matcher::new(BeOkMatcher::new(), result_format())
}

/// Succeeds when the actual value is [`Err`].
///
/// If this matcher succeeds, it unwraps the [`Err`] value. When negated, it behaves like
/// [`be_ok`].
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn be_err<'a, T, E>() -> Matcher<'a, Result<T, E>, E, T>
where
    T: 'a,
    E: 'a,
{
    Matcher::neg(BeOkMatcher::new(), NegFormat(result_format()))
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
