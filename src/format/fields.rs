use std::any::type_name;
use std::convert::Infallible;

use crate::core::{style, Format, FormattedOutput, Formatter, MatchFailure, PosMatcher};
use crate::matchers::{CombinatorMode, FailuresByField, FieldMatcher};

/// A formatter that formats failures for each field of a struct.
#[derive(Debug)]
pub struct ByFieldFormat {
    type_name: String,
}

impl ByFieldFormat {
    /// Return a new formatter given the name of the type being matched against.
    pub fn new(type_name: impl Into<String>) -> Self {
        Self {
            type_name: type_name.into(),
        }
    }
}

impl Format for ByFieldFormat {
    type Value = FailuresByField;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> anyhow::Result<()> {
        f.write_str(format!("{} {{\n", &self.type_name));

        for (field_name, maybe_fail) in value {
            f.write_str(format!("{}{}: ", style::indent(1), field_name));

            if let Some(fail) = maybe_fail {
                f.set_style(style::failure());
                f.write_str(style::FAILED_MSG);
                f.reset_style();
                f.write_char('\n');
                f.write_fmt(fail.into_fmt().indented(style::indent_len(2)));
            } else {
                f.set_style(style::success());
                f.write_str(style::MATCHED_MSG);
                f.reset_style();
                f.write_char('\n');
            }
        }

        f.write_char('}');

        Ok(())
    }
}

/// A formatter for `
#[derive(Debug)]
pub struct ByFieldMatcherFormat<Fmt> {
    inner: Fmt,
    mode: CombinatorMode,
}

impl<Fmt> ByFieldMatcherFormat<Fmt> {
    pub fn new(inner: Fmt, mode: CombinatorMode) -> Self {
        Self { inner, mode }
    }
}

impl<Fmt> Format for ByFieldMatcherFormat<Fmt>
where
    Fmt: Format<Value = FailuresByField>,
{
    type Value = MatchFailure<FailuresByField, Infallible>;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> anyhow::Result<()> {
        f.set_style(style::important());
        f.write_str(match self.mode {
            CombinatorMode::All => "Expected all of these to match:\n",
            CombinatorMode::Any => "Expected at least one of these to match:\n",
        });
        f.reset_style();

        if let MatchFailure::Pos(failures) = value {
            FormattedOutput::new(failures, self.inner)?.indented(style::indent_len(1));
        }

        Ok(())
    }
}

/// Matches when all the fields of a struct match.
///
/// This matcher operates on a struct and allows for matching on each field separately. You'll
/// generally want to use this matcher with the [`fields!`] macro.
///
/// This matches when each field of the struct matches, and skipping/omitting fields does not make
/// it fail.
///
/// This matcher can be used for both regular structs and tuple structs. See [`fields!`] for
/// details.
///
/// # Examples
///
/// ```
/// use xpct::{expect, match_fields, fields, equal, be_some, be_gt, be_true, all};
///
/// struct Person {
///     name: Option<String>,
///     age: u32,
///     is_superstar: bool,
/// }
///
/// let value = Person {
///     name: Some(String::from("Dick Mullen")),
///     age: 37,
///     is_superstar: true,
/// };
///
/// expect!(value).to(match_fields(fields!(Person {
///     name: all(|ctx| ctx
///         .to(be_some())?
///         .to(equal("Dick Mullen"))
///     ),
///     age: be_gt(0),
///     is_superstar: be_true(),
/// })));
/// ```
///
/// [`fields!`]: crate::fields
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn match_fields<'a, T>(
    func: impl FnOnce(T) -> anyhow::Result<FailuresByField> + 'a,
) -> PosMatcher<'a, T, ()>
where
    T: 'a,
{
    PosMatcher::new(
        FieldMatcher::new(CombinatorMode::All, func),
        ByFieldMatcherFormat::new(ByFieldFormat::new(type_name::<T>()), CombinatorMode::All),
    )
}

/// Matches when any of the fields of a struct match.
///
/// This matcher is similar to [`match_fields`], except it matches when *any* of the fields of a
/// struct match instead of all of them.
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn match_any_fields<'a, T>(
    func: impl FnOnce(T) -> anyhow::Result<FailuresByField> + 'a,
) -> PosMatcher<'a, T, ()>
where
    T: 'a,
{
    PosMatcher::new(
        FieldMatcher::new(CombinatorMode::Any, func),
        ByFieldMatcherFormat::new(ByFieldFormat::new(type_name::<T>()), CombinatorMode::Any),
    )
}
