use crate::{
    core::{Matcher, NegFormat},
    matchers::BeOkMatcher,
};

use super::MessageFormat;

fn result_format() -> MessageFormat {
    MessageFormat::new("Expected this to be Ok.", "Expected this to be Err.")
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
