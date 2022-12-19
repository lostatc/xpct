use crate::core::{style, Format, FormattedOutput, Formatter, MatchFailure, Matcher};
use crate::matchers::{CombinatorContext, CombinatorMatcher, CombinatorMode};

use super::SomeFailuresFormat;

#[derive(Debug, Default)]
pub struct HeaderFormat<Fmt> {
    inner: Fmt,
    header: String,
}

impl<Fmt> HeaderFormat<Fmt> {
    pub fn new(inner: Fmt, header: impl Into<String>) -> Self {
        Self {
            inner,
            header: header.into(),
        }
    }
}

impl<Fmt> Format for HeaderFormat<Fmt>
where
    Fmt: Format,
{
    type Value = MatchFailure<Fmt::Value>;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> crate::Result<()> {
        f.set_style(style::important());
        f.write_str(self.header);
        f.reset_style();
        f.write_char('\n');

        let fail = match value {
            MatchFailure::Pos(fail) => fail,
            MatchFailure::Neg(fail) => fail,
        };

        f.write_fmt(FormattedOutput::new(fail, self.inner)?.indented(style::indent_len(1)));

        Ok(())
    }
}

/// Matches when any of the passed matchers match.
///
/// This is a matcher than can be used to compose other matchers. Because this matcher needs to
/// test all the matchers that are passed to it, it can't short-circuit. This means that it doesn't
/// support chaining the output of one matcher into the next.
///
/// Instead, this matcher owns its value and passes it to each matcher, either by reference, or by
/// value if the value is [`Clone`] or [`Copy`]. The closure you pass to this matcher accepts a
/// [`CombinatorContext`], which has methods like [`borrow`], [`cloned`] and [`copied`] to
/// determine how the value is passed to matchers. From there, you can call [`to`] and [`to_not`]
/// to use matchers.
///
/// # Examples
///
/// Passing the value to matchers by reference:
///
/// ```
/// use xpct::{expect, any, equal};
///
/// expect!("Martinaise").to(any(|ctx| {
///     ctx.borrow::<str>()
///         .to(equal("The Pox"))
///         .to(equal("Central Jamrock"))
///         .to(equal("Martinaise"));
/// }));
/// ```
///
/// Passing the value to matchers by value via [`Copy`]:
///
/// ```
/// use xpct::{expect, any, equal};
///
/// expect!(41).to(any(|ctx| {
///     ctx.copied()
///         .to(equal(41))
///         .to(equal(57));
/// }));
/// ```
///
/// [`not`]: crate::not
/// [`to`]: crate::matchers::CombinatorAssertion::to
/// [`to_not`]: crate::matchers::CombinatorAssertion::to_not
/// [`borrow`]: crate::matchers::CombinatorContext::borrow
/// [`copied`]: crate::matchers::CombinatorContext::copied
/// [`cloned`]: crate::matchers::CombinatorContext::cloned
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn any<'a, T>(block: impl Fn(&mut CombinatorContext<T>) + 'a) -> Matcher<'a, T, T>
where
    T: 'a,
{
    Matcher::new(
        CombinatorMatcher::new(CombinatorMode::Any, block),
        HeaderFormat::new(
            SomeFailuresFormat::new(),
            "Expected at least one of these to match.",
        ),
    )
}
