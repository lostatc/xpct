use std::fmt;

use crate::core::style::{ALL_OK_MSG, AT_LESAT_ONE_OK_MSG};
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
/// Expect every element to be less than 20 or greater than 30.
///
/// ```
/// use xpct::{any, be_gt, be_lt, every, expect};
///
/// expect!(vec![10, 40]).to(every(|| {
///     any(|ctx| {
///         ctx.copied().to(be_lt(20)).to(be_gt(30));
///     })
/// }));
/// ```
pub fn every<'a, PosOut, NegOut, IntoIter>(
    matcher: impl Fn() -> Matcher<'a, IntoIter::Item, PosOut, NegOut> + 'a,
) -> Matcher<'a, IntoIter, Vec<PosOut>, Vec<NegOut>>
where
    IntoIter: fmt::Debug + IntoIterator + 'a,
    PosOut: 'a,
    NegOut: 'a,
{
    Matcher::new(
        EveryMatcher::new(matcher),
        HeaderFormat::new(SomeFailuresFormat::new(), ALL_OK_MSG, AT_LESAT_ONE_OK_MSG),
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
