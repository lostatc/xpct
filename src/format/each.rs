use crate::core::style::{ALL_OK_HEADER, AT_LESAT_ONE_OK_HEADER};
use crate::core::{strings, style, Format, FormattedOutput, Formatter, Matcher};
use crate::matchers::{CombinatorContext, CombinatorMatcher, CombinatorMode, SomeFailures};

use super::HeaderFormat;

/// A formatter which prints a vec of pre-formatted [`FormattedFailure`] values.
///
/// This formatter just writes the pre-formatted values via [`Formatter::write_fmt`]. It's mostly
/// useful for combinator matchers which need to print the output of the matchers they compose.
///
/// If you only need to print a single [`FormattedFailure`], use [`FailureFormat`].
///
/// [`FormattedFailure`]: crate::core::FormattedFailure
/// [`FailureFormat`]: crate::format::FailureFormat
#[non_exhaustive]
#[derive(Debug, Default)]
pub struct SomeFailuresFormat;

impl SomeFailuresFormat {
    /// Create a new [`SomeFailuresFormat`].
    pub fn new() -> Self {
        Self
    }
}

impl Format for SomeFailuresFormat {
    type Value = SomeFailures;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> crate::Result<()> {
        let num_failures = value.len();
        let failure_indent =
            strings::whitespace((strings::int_len(num_failures, 10) + style::INDENT_LEN) as usize);

        for (i, maybe_fail) in value.into_iter().enumerate() {
            if let Some(fail) = maybe_fail {
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

                f.write_fmt(FormattedOutput::from(fail).indented(failure_indent.as_ref()));

                f.write_char('\n');
            };
        }

        Ok(())
    }
}

/// Succeeds when each of the passed matchers succeeds.
///
/// This is a matcher than can be used to compose other matchers. It's similar to [`all`], except it
/// does not short-circuit and it does not chain the output of each matcher into the next. You can
/// use matcher this when:
///
/// 1. You want to test all the matchers instead of just failing early and printing the first
///    failure.
/// 2. You want to perform multiple assertions on the same value without transforming it (like
///    [`be_ok`] and [`be_some`] do).
///
/// This matcher owns its value and passes it to each matcher, either by reference, or by value if
/// the value is [`Clone`] or [`Copy`]. The closure you pass to this matcher accepts a
/// [`CombinatorContext`], which has methods like [`borrow`], [`cloned`] and [`copied`] to determine
/// how the value is passed to matchers. From there, you can call [`to`] and [`to_not`] to use
/// matchers.
///
/// # Examples
///
/// Passing the value to matchers by reference:
///
/// ```
/// use xpct::{each, expect, have_len, match_regex};
///
/// expect!("11b72db5-ff70-40a5-8728-937faf86ce48").to(each(|ctx| {
///     ctx.borrow::<str>()
///         .to(have_len(36))
///         .to(match_regex("[0-9a-f-]+"));
/// }));
/// ```
///
/// Passing the value to matchers by value via [`Clone`]:
///
/// ```
/// use xpct::{each, expect, have_len, match_regex};
///
/// let uuid = String::from("11b72db5-ff70-40a5-8728-937faf86ce48");
///
/// expect!(uuid).to(each(|ctx| {
///     ctx.cloned()
///         .to(have_len(36))
///         .to(match_regex("[0-9a-f-]+"));
/// }));
/// ```
///
/// [`all`]: crate::all
/// [`not`]: crate::not
/// [`be_ok`]: crate::be_ok
/// [`be_some`]: crate::be_some
/// [`to`]: crate::matchers::CombinatorAssertion::to
/// [`to_not`]: crate::matchers::CombinatorAssertion::to_not
/// [`done`]: crate::matchers::CombinatorAssertion::done
/// [`borrow`]: crate::matchers::CombinatorContext::borrow
/// [`copied`]: crate::matchers::CombinatorContext::copied
/// [`cloned`]: crate::matchers::CombinatorContext::cloned
pub fn each<'a, T>(block: impl FnOnce(&mut CombinatorContext<T>) + 'a) -> Matcher<'a, T, T>
where
    T: 'a,
{
    Matcher::transform(
        CombinatorMatcher::new(CombinatorMode::All, block),
        HeaderFormat::new(
            SomeFailuresFormat::new(),
            ALL_OK_HEADER,
            AT_LESAT_ONE_OK_HEADER,
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::each;
    use crate::{be_gt, be_lt, expect};

    #[test]
    fn succeeds_when_all_matchers_succeed() {
        expect!(1).to(each(|ctx| ctx.copied().to(be_lt(2)).to(be_gt(0)).done()));
    }

    #[test]
    fn succeeds_when_not_all_matchers_succeed() {
        expect!(1).to_not(each(|ctx| ctx.copied().to(be_lt(0)).to(be_gt(0)).done()));
    }

    #[test]
    #[should_panic]
    fn fails_when_all_matchers_succeed() {
        expect!(1).to_not(each(|ctx| ctx.copied().to(be_lt(2)).to(be_gt(0)).done()));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_all_matchers_succeed() {
        expect!(1).to(each(|ctx| ctx.copied().to(be_lt(0)).to(be_gt(0)).done()));
    }
}
