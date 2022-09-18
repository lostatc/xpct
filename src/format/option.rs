use std::convert::Infallible;

use crate::core::{Matcher, NegFormat};
use crate::matchers::BeSomeMatcher;

use super::MessageFormat;

fn option_format() -> MessageFormat {
    MessageFormat::new("Expected this to be Some.", "Expected this to be None")
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn be_some<'a, T>() -> Matcher<'a, Option<T>, T, Option<Infallible>>
where
    T: 'a,
{
    Matcher::new(BeSomeMatcher::new(), option_format())
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn be_none<'a, T>() -> Matcher<'a, Option<T>, Option<Infallible>, T>
where
    T: 'a,
{
    Matcher::neg(BeSomeMatcher::new(), NegFormat(option_format()))
}
