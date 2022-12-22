use std::any::type_name;

use crate::core::style::{ALL_OK_MSG, AT_LESAT_ONE_OK_MSG};
use crate::core::{style, Format, FormattedOutput, Formatter, Matcher};
use crate::matchers::{CombinatorMode, FailuresByField, FieldMatcher, FieldMatcherSpec};

use super::HeaderFormat;

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

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> crate::Result<()> {
        f.write_str(format!("{} {{\n", &self.type_name));

        for (field_name, maybe_fail) in value {
            f.write_str(format!("{}{}: ", style::indent(1), field_name));

            if let Some(fail) = maybe_fail {
                f.set_style(style::failure());
                f.write_str(style::FAILED_MSG);
                f.reset_style();
                f.write_char('\n');
                f.write_fmt(FormattedOutput::from(fail).indented(style::indent_len(2)));
            } else {
                f.set_style(style::success());
                f.write_str(style::OK_MSG);
                f.reset_style();
                f.write_char('\n');
            }
        }

        f.write_char('}');

        Ok(())
    }
}

/// Succeeds when all the fields of a struct succeed.
///
/// This matcher operates on a struct and allows for matching on each field separately. This is
/// used with the [`fields!`] macro.
///
/// This succeeds when each field of the struct succeeds, and skipping/omitting fields does not
/// make it fail.
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
pub fn match_fields<'a, T>(spec: FieldMatcherSpec<'a, T>) -> Matcher<'a, T, ()>
where
    T: 'a,
{
    Matcher::new(
        FieldMatcher::new(CombinatorMode::All, spec),
        HeaderFormat::new(
            ByFieldFormat::new(type_name::<T>()),
            ALL_OK_MSG,
            AT_LESAT_ONE_OK_MSG,
        ),
    )
}

/// Succeeds when any of the fields of a struct succeed.
///
/// This matcher is similar to [`match_fields`], except it succeeds when *any* of the fields of a
/// struct succeed instead of all of them.
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn match_any_fields<'a, T>(spec: FieldMatcherSpec<'a, T>) -> Matcher<'a, T, ()>
where
    T: 'a,
{
    Matcher::new(
        FieldMatcher::new(CombinatorMode::Any, spec),
        HeaderFormat::new(
            ByFieldFormat::new(type_name::<T>()),
            AT_LESAT_ONE_OK_MSG,
            ALL_OK_MSG,
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::{match_any_fields, match_fields};
    use crate::{equal, expect, fields};

    struct Value {
        foo: String,
        bar: u32,
    }

    #[test]
    fn succeeds_when_all_matchers_succeed() {
        let value = Value {
            foo: "some string".into(),
            bar: 1,
        };

        expect!(value).to(match_fields(fields!(Value {
            foo: equal("some string"),
            bar: equal(1),
        })));
    }

    #[test]
    fn succeeds_when_not_all_matchers_succeed() {
        let value = Value {
            foo: "some string".into(),
            bar: 1,
        };

        expect!(value).to_not(match_fields(fields!(Value {
            foo: equal("a different string"),
            bar: equal(2),
        })));
    }

    #[test]
    fn succeeds_when_any_matchers_succeed() {
        let value = Value {
            foo: "some string".into(),
            bar: 1,
        };

        expect!(value).to(match_any_fields(fields!(Value {
            foo: equal("a different string"),
            bar: equal(1),
        })));
    }

    #[test]
    fn succeeds_when_not_any_matchers_succeed() {
        let value = Value {
            foo: "some string".into(),
            bar: 1,
        };

        expect!(value).to_not(match_any_fields(fields!(Value {
            foo: equal("a different string"),
            bar: equal(2),
        })));
    }

    #[test]
    #[should_panic]
    fn fails_when_all_matchers_succeed() {
        let value = Value {
            foo: "some string".into(),
            bar: 1,
        };

        expect!(value).to_not(match_fields(fields!(Value {
            foo: equal("some string"),
            bar: equal(1),
        })));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_all_matchers_succeed() {
        let value = Value {
            foo: "some string".into(),
            bar: 1,
        };

        expect!(value).to(match_fields(fields!(Value {
            foo: equal("a different string"),
            bar: equal(2),
        })));
    }

    #[test]
    #[should_panic]
    fn fails_when_any_matchers_succeed() {
        let value = Value {
            foo: "some string".into(),
            bar: 1,
        };

        expect!(value).to_not(match_any_fields(fields!(Value {
            foo: equal("a different string"),
            bar: equal(1),
        })));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_any_matchers_succeed() {
        let value = Value {
            foo: "some string".into(),
            bar: 1,
        };

        expect!(value).to(match_any_fields(fields!(Value {
            foo: equal("a different string"),
            bar: equal(2),
        })));
    }
}
