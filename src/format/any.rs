use crate::core::{strings, style, Format, Formatter, MatchFailure, PosMatcher};
use crate::matchers::{AllFailures, AnyContext, AnyMatcher};

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct AllFailuresFormat;

impl AllFailuresFormat {
    pub fn new() -> Self {
        Self
    }
}

impl Format for AllFailuresFormat {
    type Value = AllFailures;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> anyhow::Result<()> {
        let num_failures = value.len();
        let failure_indent = strings::int_len(num_failures, 10) + 4;

        for (i, fail) in value.into_iter().enumerate() {
            f.set_style(style::index());
            f.write_str(&format!(
                "{}[{}]  ",
                strings::pad_int(i, num_failures, 10),
                i,
            ));
            f.reset_style();

            f.set_style(style::failure());
            f.write_str(style::FAILED_MSG);
            f.reset_style();
            f.write_char('\n');

            f.write_fmt(fail.into_fmt().indented(failure_indent));
            f.write_char('\n');
        }

        Ok(())
    }
}

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

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> anyhow::Result<()> {
        f.set_style(style::important());
        f.write_str(self.header);
        f.reset_style();
        f.write_char('\n');

        match value {
            MatchFailure::Pos(fail) => self.inner.fmt(f, fail),
            MatchFailure::Neg(fail) => self.inner.fmt(f, fail),
        }
    }
}

/// Matches when any of the passed matchers match.
///
/// This is a matcher than can be used to compose other matchers. Because this matcher needs to
/// test all the matchers that are passed to it, it can't short-circuit. This means that it doesn't
/// support chaining the output of one matcher into the next.
///
/// Instead, this matcher owns its value and passes it to each matcher, either by reference, or by
/// value if the value is [`Clone`] or [`Copy`]. The closure you pass to this matcher accepts an
/// [`AnyContext`], which has methods like [`borrow`][`AnyContext::borrow`],
/// [`cloned`][`AnyContext::cloned`], [`copied`][`AnyContext::copied`], and
/// [`map`][`AnyContext::map`] to determine how the value is passed to matchers. From there, you
/// can call methods like `to` and `to_not`.
///
/// This matcher cannot be negated, such as with [`not`]. Instead, you can just negate each of the
/// matchers passed to it by calling `to_not` or using [`not`] on them.
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
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn any<'a, T>(block: impl Fn(&mut AnyContext<T>) + 'a) -> PosMatcher<'a, T, T>
where
    T: 'a,
{
    PosMatcher::new(
        AnyMatcher::new(block),
        HeaderFormat::new(
            AllFailuresFormat::new(),
            "Expected at least one of these to match.",
        ),
    )
}
