use std::fmt;

use crate::core::{FormattedFailure, Match, MatchOutcome};
use crate::{fail, success};

use super::CombinatorMode;

/// A pairing of field names to optional match failures.
///
/// This can be used by matchers that test each field of a struct or tuple.
pub type FailuresByField = Vec<(&'static str, Option<FormattedFailure>)>;

type BoxFieldMatcherSpecFunc<'a, T> =
    Box<dyn FnOnce(T, bool) -> crate::Result<FailuresByField> + 'a>;

/// An opaque type to be used with [`match_fields`] and [`match_any_fields`].
///
/// This type is returned by [`fields!`] and can be passed to [`match_fields`] and
/// [`match_any_fields`].
///
/// [`fields!`]: crate::fields
/// [`match_fields`]: crate::match_fields
/// [`match_any_fields`]: crate::match_any_fields
pub struct FieldMatcherSpec<'a, T> {
    func: BoxFieldMatcherSpecFunc<'a, T>,
}

impl<'a, T> fmt::Debug for FieldMatcherSpec<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FieldMatcherSpec").finish_non_exhaustive()
    }
}

impl<'a, T> FieldMatcherSpec<'a, T> {
    /// This is only meant to be called from the [`fields!`][crate::fields] macro.
    #[doc(hidden)]
    pub fn __new(func: impl FnOnce(T, bool) -> crate::Result<FailuresByField> + 'a) -> Self {
        Self {
            func: Box::new(func),
        }
    }
}

/// A matcher for matching on fields of a struct.
///
/// See [`match_fields`] for details.
///
/// [`match_fields`]: crate::match_fields
#[derive(Debug)]
pub struct FieldMatcher<'a, T> {
    mode: CombinatorMode,
    spec: FieldMatcherSpec<'a, T>,
}

impl<'a, T> FieldMatcher<'a, T> {
    /// Create a new matcher.
    ///
    /// This accepts a [`FieldMatcherSpec`], which you can generate using the
    /// [`fields!`][crate::fields] macro.
    pub fn new(mode: CombinatorMode, spec: FieldMatcherSpec<'a, T>) -> Self {
        Self { spec, mode }
    }
}

impl<'a, T> Match for FieldMatcher<'a, T> {
    type In = T;

    type PosOut = ();
    type NegOut = ();

    type PosFail = FailuresByField;
    type NegFail = FailuresByField;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        let failures = (self.spec.func)(actual, false)?;
        match self.mode {
            CombinatorMode::Any => {
                if failures.iter().any(|(_, fail)| fail.is_none()) {
                    success!(())
                } else {
                    fail!(failures)
                }
            }
            CombinatorMode::All => {
                if failures.iter().all(|(_, fail)| fail.is_none()) {
                    success!(())
                } else {
                    fail!(failures)
                }
            }
        }
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        let failures = (self.spec.func)(actual, true)?;
        match self.mode {
            CombinatorMode::Any => {
                if failures.iter().all(|(_, fail)| fail.is_none()) {
                    success!(())
                } else {
                    fail!(failures)
                }
            }
            CombinatorMode::All => {
                if failures.iter().any(|(_, fail)| fail.is_none()) {
                    success!(())
                } else {
                    fail!(failures)
                }
            }
        }
    }
}

/// Apply matchers to multiple struct fields.
///
/// This macro is meant to be used with matchers like [`match_fields`] and [`match_any_fields`]. It
/// provides a Rust-like syntax for mapping struct fields to matchers. The syntax looks like this:
///
/// ```
/// use xpct::{fields, equal, be_ge};
///
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// fields!(Person {
///     name: equal("Jean Vicquemare"),
///     age: be_ge(34),
/// });
///
/// // You can omit fields.
/// fields!(Person {
///     name: equal("Jean Vicquemare"),
/// });
/// ```
///
/// This macro also supports tuple structs; the syntax is identical, except you replace the field
/// names with indices.
///
/// ```
/// use xpct::{fields, equal};
///
/// struct Point(u64, u64);
///
/// fields!(Point {
///     0: equal(41),
///     1: equal(57),
/// });
/// ```
///
/// The syntax for tuple structs looks different from the Rust syntax because it makes it easy to
/// skip fields:
///
/// ```
/// # use xpct::{fields, equal};
/// # struct Point(u64, u64);
/// fields!(Point {
///     1: equal(57),
/// });
/// ```
///
/// This macro returns a [`FieldMatcherSpec`] value that can be passed to [`match_fields`] or
/// [`match_any_fields`].
///
/// [`match_fields`]: crate::match_fields
/// [`match_any_fields`]: crate::match_any_fields
#[macro_export]
macro_rules! fields {
    (
        $struct_type:ty {
            $(
                $field_name:tt: $matcher:expr
            ),+
            $(,)?
        }
    ) => {
        $crate::matchers::FieldMatcherSpec::__new(
            |input: $struct_type, negated: ::std::primitive::bool| -> $crate::Result<::std::vec::Vec<(&::std::primitive::str, ::std::option::Option<$crate::core::FormattedFailure>)>> {
                $crate::Result::Ok(vec![$(
                    (
                        stringify!($field_name),
                        if negated {
                            match $crate::core::DynMatch::match_neg(::std::boxed::Box::new($matcher), input.$field_name)? {
                                $crate::core::MatchOutcome::Success(_) => ::std::option::Option::None,
                                $crate::core::MatchOutcome::Fail(fail) => ::std::option::Option::Some(fail),
                            }
                        } else {
                            match $crate::core::DynMatch::match_pos(::std::boxed::Box::new($matcher), input.$field_name)? {
                                $crate::core::MatchOutcome::Success(_) => ::std::option::Option::None,
                                $crate::core::MatchOutcome::Fail(fail) => ::std::option::Option::Some(fail),
                            }
                        },
                    ),
                )+])
            }
        )
    };
}
