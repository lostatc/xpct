use std::fmt;

use crate::core::style::{ALL_OK_HEADER, AT_LESAT_ONE_OK_HEADER};
use crate::core::Matcher;
use crate::matchers::EveryMatcher;

use super::{HeaderFormat, SomeFailuresFormat};

/// Succeeds when the matcher succeeds for every element of the actual value.
///
/// This accepts a closure which returns a matcher for every element in the collection.
///
/// # Examples
///
/// Expect every element to be `Some(_)`, unwrapping into a vec of strings.
///
/// ```
/// use xpct::{be_some, every, expect};
///
/// let items = vec![Some("foo"), Some("bar")];
///
/// let output: Vec<&str> = expect!(items)
///     .to(every(be_some))
///     .into_inner();
/// ```
///
/// Expect every element to be in the range `20..30`.
///
/// ```
/// use xpct::{be_in, every, expect};
///
/// expect!(vec![20, 25]).to(every(|| be_in(20..30)));
/// ```
pub fn every<'a, PosOut, NegOut, IntoIter>(
    matcher: impl Fn() -> Matcher<'a, IntoIter::Item, PosOut, NegOut> + 'a,
) -> Matcher<'a, IntoIter, Vec<PosOut>, Vec<NegOut>>
where
    IntoIter: fmt::Debug + IntoIterator + 'a,
    PosOut: 'a,
    NegOut: 'a,
{
    Matcher::transform(
        EveryMatcher::new(matcher),
        HeaderFormat::new(
            SomeFailuresFormat::new(),
            ALL_OK_HEADER,
            AT_LESAT_ONE_OK_HEADER,
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::every;
    use crate::{be_some, expect};

    #[test]
    fn succeeds_when_every_element_succeeds() {
        expect!(vec![Some("foo"), Some("bar")]).to(every(be_some));
    }

    #[test]
    fn succeeds_when_not_every_element_succeeds() {
        expect!(vec![Some("foo"), None]).to_not(every(be_some));
    }

    #[test]
    #[should_panic]
    fn fails_when_every_element_succeeds() {
        expect!(vec![Some("foo"), Some("bar")]).to_not(every(be_some));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_every_element_succeeds() {
        expect!(vec![Some("foo"), None]).to(every(be_some));
    }
}
