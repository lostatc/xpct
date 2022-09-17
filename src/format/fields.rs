use std::any::type_name;
use std::convert::Infallible;

use crate::core::{style, Format, FormattedOutput, Formatter, MatchFailure, PosMatcher};
use crate::matchers::{ByFieldMatcher, ByMatchMode, FailuresByField};

#[derive(Debug)]
pub struct ByFieldFormat {
    type_name: String,
}

impl ByFieldFormat {
    pub fn new(type_name: impl AsRef<str>) -> Self {
        Self {
            type_name: type_name.as_ref().into(),
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

        f.write_str("}\n");

        Ok(())
    }
}

#[derive(Debug)]
pub struct ByFieldMatcherFormat<Fmt> {
    inner: Fmt,
    mode: ByMatchMode,
}

impl<Fmt> ByFieldMatcherFormat<Fmt> {
    pub fn new(inner: Fmt, mode: ByMatchMode) -> Self {
        Self { inner, mode }
    }
}

impl ByFieldMatcherFormat<ByFieldFormat> {
    pub fn default(name: impl AsRef<str>, mode: ByMatchMode) -> Self {
        Self {
            inner: ByFieldFormat::new(name),
            mode,
        }
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
            ByMatchMode::All => "Expected all of these to match:\n",
            ByMatchMode::Any => "Expected at least one of these to match:\n",
        });
        f.reset_style();

        match value {
            MatchFailure::Pos(failures) => f.write_fmt(
                FormattedOutput::new(failures, self.inner)?.indented(style::indent_len(1)),
            ),
            MatchFailure::Neg(_) => unreachable!(),
        }

        Ok(())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn match_all_fields<'a, T>(
    func: impl FnOnce(T) -> anyhow::Result<FailuresByField> + 'a,
) -> PosMatcher<'a, T, ()>
where
    T: 'a,
{
    PosMatcher::new(
        ByFieldMatcher::new(ByMatchMode::All, func),
        ByFieldMatcherFormat::default(type_name::<T>(), ByMatchMode::All),
    )
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn match_any_fields<'a, T>(
    func: impl FnOnce(T) -> anyhow::Result<FailuresByField> + 'a,
) -> PosMatcher<'a, T, ()>
where
    T: 'a,
{
    PosMatcher::new(
        ByFieldMatcher::new(ByMatchMode::Any, func),
        ByFieldMatcherFormat::default(type_name::<T>(), ByMatchMode::Any),
    )
}
