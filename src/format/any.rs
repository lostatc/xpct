use crate::core::style::{ALL_OK_HEADER, AT_LESAT_ONE_OK_HEADER};
use crate::core::{style, Format, FormattedOutput, Formatter, MatchFailure, Matcher};
use crate::matchers::combinators::{CombinatorContext, CombinatorMatcher, CombinatorMode};

use super::SomeFailuresFormat;

/// A formatter that adds a header to the output of another formatter.
///
/// # Examples
///
/// ```
/// # use xpct::format::{HeaderFormat, SomeFailuresFormat};
/// let formatter = HeaderFormat::new(
///     SomeFailuresFormat::new(),
///     "Expected at least one of these to match:",
///     "Expected all of these to match:",
/// );
/// ```
#[derive(Debug, Default)]
pub struct HeaderFormat<Fmt> {
    inner: Fmt,
    pos_header: String,
    neg_header: String,
}

impl<Fmt> HeaderFormat<Fmt> {
    /// Create a new [`HeaderFormat`] from the header string and inner formatter.
    ///
    /// This accepts two header strings, one for the positive case and one for the negative case
    /// respectively. The first is used normally, and the second is used when the matcher is
    /// negated.
    pub fn new(inner: Fmt, pos_header: impl Into<String>, neg_header: impl Into<String>) -> Self {
        Self {
            inner,
            pos_header: pos_header.into(),
            neg_header: neg_header.into(),
        }
    }
}

impl<Fmt> Format for HeaderFormat<Fmt>
where
    Fmt: Format,
{
    type Value = MatchFailure<Fmt::Value>;

    fn fmt(&self, f: &mut Formatter, value: Self::Value) -> crate::Result<()> {
        f.set_style(style::important());
        if value.is_pos() {
            f.write_str(&self.pos_header);
        } else {
            f.write_str(&self.neg_header)
        }
        f.reset_style();
        f.write_char('\n');

        let fail = value.into_inner();

        f.write_fmt(FormattedOutput::new(fail, &self.inner)?.indented(style::indent(1)));

        Ok(())
    }
}

/// Succeeds when any of the passed matchers succeed.
///
/// This is a matcher than can be used to compose other matchers. This matcher doesn't
/// short-circuit; it tests all the matchers that are passed to it.
///
/// This matcher doesn't chain the output of each matcher into the next. Instead, it owns its value
/// and passes it to each matcher, either by reference, or by value if the value is [`Clone`] or
/// [`Copy`]. The closure you pass to this matcher accepts a [`CombinatorContext`], which has
/// methods like [`borrow`], [`cloned`] and [`copied`] to determine how the value is passed to
/// matchers. From there, you can call [`to`] and [`to_not`] to use matchers.
///
/// # Examples
///
/// Passing the value to matchers by reference:
///
/// ```
/// use xpct::{any, expect, have_prefix};
///
/// expect!("https://example.com").to(any(|ctx| {
///     ctx.borrow::<str>()
///         .to(have_prefix("http://"))
///         .to(have_prefix("https://"));
/// }));
/// ```
///
/// Passing the value to matchers by value via [`Copy`]:
///
/// ```
/// use xpct::{expect, any, be_gt, be_lt};
///
/// expect!(60).to(any(|ctx| {
///     ctx.copied()
///         .to(be_lt(41))
///         .to(be_gt(57));
/// }));
/// ```
///
/// [`not`]: crate::not
/// [`to`]: crate::matchers::combinators::CombinatorAssertion::to
/// [`to_not`]: crate::matchers::combinators::CombinatorAssertion::to_not
/// [`borrow`]: crate::matchers::combinators::CombinatorContext::borrow
/// [`copied`]: crate::matchers::combinators::CombinatorContext::copied
/// [`cloned`]: crate::matchers::combinators::CombinatorContext::cloned
pub fn any<'a, T>(block: impl Fn(&mut CombinatorContext<T>) + 'a) -> Matcher<'a, T, T>
where
    T: 'a,
{
    Matcher::transform(
        CombinatorMatcher::new(CombinatorMode::Any, block),
        HeaderFormat::new(
            SomeFailuresFormat::new(),
            AT_LESAT_ONE_OK_HEADER,
            ALL_OK_HEADER,
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::any;
    use crate::{be_gt, be_lt, expect};

    #[test]
    fn succeeds_when_any_matchers_succeed() {
        expect!(1).to(any(|ctx| ctx.copied().to(be_lt(0)).to(be_gt(0)).done()));
    }

    #[test]
    fn succeeds_when_not_any_matchers_succeed() {
        expect!(1).to_not(any(|ctx| ctx.copied().to(be_lt(0)).to(be_gt(2)).done()));
    }

    #[test]
    #[should_panic]
    fn fails_when_any_matchers_succeed() {
        expect!(1).to_not(any(|ctx| ctx.copied().to(be_lt(0)).to(be_gt(0)).done()));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_any_matchers_succeed() {
        expect!(1).to(any(|ctx| ctx.copied().to(be_lt(0)).to(be_gt(2)).done()));
    }
}
