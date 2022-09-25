use crate::core::{strings, style, Format, Formatter, PosMatcher};
use crate::matchers::{CombinatorContext, CombinatorMatcher, CombinatorMode, SomeFailures};

use super::HeaderFormat;

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct SomeFailuresFormat;

impl SomeFailuresFormat {
    pub fn new() -> Self {
        Self
    }
}

impl Format for SomeFailuresFormat {
    type Value = SomeFailures;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> crate::Result<()> {
        let num_failures = value.len();
        let failure_indent = strings::int_len(num_failures, 10) + 4;

        for (i, maybe_fail) in value.into_iter().enumerate() {
            f.set_style(style::index());
            f.write_str(&format!(
                "{}[{}]  ",
                strings::pad_int(i, num_failures, 10),
                i,
            ));
            f.reset_style();

            match maybe_fail {
                Some(fail) => {
                    f.set_style(style::failure());
                    f.write_str(style::FAILED_MSG);
                    f.reset_style();
                    f.write_char('\n');

                    f.write_fmt(fail.into_fmt().indented(failure_indent));
                }
                None => {
                    f.set_style(style::success());
                    f.write_str(style::MATCHED_MSG);
                    f.reset_style();
                    f.write_char('\n');
                }
            }

            f.write_char('\n');
        }

        Ok(())
    }
}

/// Matches when all of the passed matchers match.
///
/// This matcher is similar to [`all`], except it does not short-circuit and it does not chain the
/// output of each matcher into the next. You can use matcher this when:
///
/// 1. You want to test all the matchers instead of just failing early and printing the first
///    failure.
/// 2. You want to perform multiple assertions on the same value without transforming it (like
///    [`be_ok`] and [`be_some`] do).
///
/// This matcher owns its value and passes it to each matcher, either by reference, or by value if
/// the value is [`Clone`] or [`Copy`]. The closure you pass to this matcher accepts a
/// [`CombinatorContext`], which has methods like [`borrow`], [`cloned`] and [`copied`] to
/// determine how the value is passed to matchers. From there, you can call [`to`] and [`to_not`]
/// to use matchers.
///
/// This matcher cannot be negated, such as with [`not`]. Instead, you can just negate each of the
/// matchers passed to it by calling [`to_not`] or using [`not`] on them.
///
/// # Examples
///
/// Passing the value to matchers by reference:
///
/// ```
/// use xpct::{expect, each, be_lt, be_gt};
///
/// expect!("Billie").to(each(|ctx| {
///     ctx.borrow::<str>()
///         .to(be_lt("Cuno"))
///         .to(be_gt("Annette"));
/// }));
/// ```
///
/// Passing the value to matchers by value via [`Copy`]:
///
/// ```
/// use xpct::{expect, each, be_lt, be_gt};
///
/// expect!(20.0).to(each(|ctx| {
///     ctx.copied()
///         .to(be_lt(130.0))
///         .to(be_gt(0.40));
/// }));
///
/// ```
///
/// [`all`]: crate::all
/// [`not`]: crate::not
/// [`be_ok`]: crate::be_ok
/// [`be_some`]: crate::be_some
/// [`to`]: crate::matchers::CombinatorAssertion::to
/// [`to_not`]: crate::matchers::CombinatorAssertion::to_not
/// [`borrow`]: crate::matchers::CombinatorContext::borrow
/// [`copied`]: crate::matchers::CombinatorContext::copied
/// [`cloned`]: crate::matchers::CombinatorContext::cloned
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn each<'a, T>(block: impl FnOnce(&mut CombinatorContext<T>) + 'a) -> PosMatcher<'a, T, T>
where
    T: 'a,
{
    PosMatcher::new(
        CombinatorMatcher::new(CombinatorMode::All, block),
        HeaderFormat::new(SomeFailuresFormat::new(), "Expected all of these to match:"),
    )
}
