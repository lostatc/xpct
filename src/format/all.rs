use crate::core::style::AT_LESAT_ONE_NOT_OK_MSG;
use crate::core::{DispatchFormat, MatchError, Matcher};
use crate::matchers::{ChainAssertion, ChainMatcher};

use super::{FailureFormat, MessageFormat};

/// Succeeds when all of the passed matchers succeed.
///
/// This is a matcher than can be used to compose other matchers. It's similar to [`each`], except
/// it short-circuits on the first failed match and chains the output of each matcher into the next.
///
/// This matcher accepts a closure which is passed a [`ChainAssertion`] value, which is similar to
/// the [`Assertion`] value returned by [`expect!`]. You can call [`to`] and [`to_not`] on it to use
/// matchers.
///
/// You typically do not want to negate this matcher (such as with [`not`]). Instead, you probably
/// want to negate the individual matchers you're composing with it. Here's why:
///
/// 1. If you negate this matcher and it succeeds (meaning that all the matchers failed), it can't
///    return the transformed value at the end (it will return `()`). Matchers don't return the
///    value that was passed into them when they fail.
/// 2. If you negate this matcher and it fails (meaning that all the matchers succeeded), the output
///    it produces won't be particularly useful. Matchers don't produce failure output when they
///    succeed.
///
/// # Examples
///
/// Normally, you can just chain together matchers like this:
///
/// ```
/// use xpct::{expect, be_some, equal};
///
/// expect!(Some("horrific"))
///     .to(be_some())
///     .to(equal("horrific"));
/// ```
///
/// However, if you need to do this inside of another matcher, such as when using [`match_fields`],
/// you can use [`all`]:
///
/// ```
/// use xpct::{expect, match_fields, fields, equal, all, not, be_empty, be_some};
///
/// struct Person {
///     name: Option<String>,
///     age: u32,
/// }
///
/// let person = Person {
///     name: Some(String::from("Kim Kitsuragi")),
///     age: 43,
/// };
///
/// expect!(person).to(match_fields(fields!(Person {
///     name: all(|ctx| ctx
///         .to(be_some())?
///         .to(not(be_empty()))
///     ),
///     age: equal(43),
/// })));
/// ```
///
/// [`each`]: crate::each
/// [`Assertion`]: crate::core::Assertion
/// [`to`]: crate::matchers::ChainAssertion::to
/// [`to_not`]: crate::matchers::ChainAssertion::to_not
/// [`not`]: crate::not
/// [`expect!`]: crate::expect
/// [`match_fields`]: crate::match_fields
pub fn all<'a, In, Out>(
    block: impl FnOnce(ChainAssertion<In>) -> Result<ChainAssertion<Out>, MatchError> + 'a,
) -> Matcher<'a, In, Out, ()>
where
    In: 'a,
    Out: 'a,
{
    let format = DispatchFormat::new(
        FailureFormat::new(),
        MessageFormat::new("", AT_LESAT_ONE_NOT_OK_MSG),
    );

    Matcher::transform(ChainMatcher::new(block), format)
}

#[cfg(test)]
mod tests {
    use super::all;
    use crate::{be_gt, be_lt, expect};

    #[test]
    fn succeeds_when_all_matchers_succeed() {
        expect!(1).to(all(|ctx| ctx.to(be_lt(2))?.to(be_gt(0))));
    }

    #[test]
    fn succeeds_when_not_all_matchers_succeed() {
        expect!(1).to_not(all(|ctx| ctx.to(be_lt(0))?.to(be_gt(0))));
    }

    #[test]
    #[should_panic]
    fn fails_when_all_matchers_succeed() {
        expect!(1).to_not(all(|ctx| ctx.to(be_lt(2))?.to(be_gt(0))));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_all_matchers_succeed() {
        expect!(1).to(all(|ctx| ctx.to(be_lt(0))?.to(be_gt(0))));
    }
}
