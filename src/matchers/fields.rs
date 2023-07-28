use std::fmt;

use crate::core::{FormattedFailure, MatchOutcome, TransformMatch};

use super::CombinatorMode;

/// A pairing of field names to optional match failures.
///
/// This can be used by matchers that test each field of a struct or tuple.
pub type FailuresByField = Vec<(&'static str, Option<FormattedFailure>)>;

/// This method is an implementation detail of the [`fields!`][crate::fields] macro and IS NOT part
/// of the public API.
#[doc(hidden)]
#[derive(Debug)]
pub struct __FieldsSpecParams<T> {
    pub actual: T,
    pub negated: bool,
}

type FieldsSpecFunc<'a, T> =
    Box<dyn FnOnce(__FieldsSpecParams<T>) -> crate::Result<FailuresByField> + 'a>;

/// An opaque type used with [`match_fields`] and [`match_any_fields`].
///
/// This type is returned by [`fields!`] and can be passed to [`match_fields`] and
/// [`match_any_fields`].
///
/// [`fields!`]: crate::fields
/// [`match_fields`]: crate::match_fields
/// [`match_any_fields`]: crate::match_any_fields
pub struct FieldsSpec<'a, T> {
    func: FieldsSpecFunc<'a, T>,
}

impl<'a, T> fmt::Debug for FieldsSpec<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FieldsSpec").finish_non_exhaustive()
    }
}

impl<'a, T> FieldsSpec<'a, T> {
    /// This method is an implementation detail of the [`fields!`][crate::fields] macro and IS NOT
    /// part of the public API.
    #[doc(hidden)]
    pub fn __new(
        func: impl FnOnce(__FieldsSpecParams<T>) -> crate::Result<FailuresByField> + 'a,
    ) -> Self {
        Self {
            func: Box::new(func),
        }
    }
}

/// The matcher for [`match_fields`] and [`match_any_fields`].
///
/// [`match_fields`]: crate::match_fields
/// [`match_any_fields`]: crate::match_any_fields
#[derive(Debug)]
pub struct FieldMatcher<'a, T> {
    mode: CombinatorMode,
    spec: FieldsSpec<'a, T>,
}

impl<'a, T> FieldMatcher<'a, T> {
    /// Create a new matcher.
    ///
    /// This accepts a [`FieldsSpec`], which you can generate using the [`fields!`][crate::fields]
    /// macro.
    pub fn new(mode: CombinatorMode, spec: FieldsSpec<'a, T>) -> Self {
        Self { spec, mode }
    }
}

impl<'a, T> TransformMatch for FieldMatcher<'a, T> {
    type In = T;

    type PosOut = ();
    type NegOut = ();

    type PosFail = FailuresByField;
    type NegFail = FailuresByField;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        let failures = (self.spec.func)(__FieldsSpecParams {
            actual,
            negated: false,
        })?;

        match self.mode {
            CombinatorMode::Any => {
                if failures.iter().any(|(_, fail)| fail.is_none()) {
                    Ok(MatchOutcome::Success(()))
                } else {
                    Ok(MatchOutcome::Fail(failures))
                }
            }
            CombinatorMode::All => {
                if failures.iter().all(|(_, fail)| fail.is_none()) {
                    Ok(MatchOutcome::Success(()))
                } else {
                    Ok(MatchOutcome::Fail(failures))
                }
            }
        }
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        let failures = (self.spec.func)(__FieldsSpecParams {
            actual,
            negated: true,
        })?;

        match self.mode {
            CombinatorMode::Any => {
                if failures.iter().all(|(_, fail)| fail.is_none()) {
                    Ok(MatchOutcome::Success(()))
                } else {
                    Ok(MatchOutcome::Fail(failures))
                }
            }
            CombinatorMode::All => {
                if failures.iter().any(|(_, fail)| fail.is_none()) {
                    Ok(MatchOutcome::Success(()))
                } else {
                    Ok(MatchOutcome::Fail(failures))
                }
            }
        }
    }
}

/// Apply matchers to multiple struct fields.
///
/// This macro is meant to be used with the [`match_fields`] and [`match_any_fields`] matchers. It
/// provides a Rust-like syntax for mapping struct fields to matchers.
///
/// This macro returns an opaque [`FieldsSpec`] value that can be passed to [`match_fields`] or
/// [`match_any_fields`].
///
/// The syntax of this macro looks like this:
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
        $crate::matchers::FieldsSpec::__new(
            |params: $crate::matchers::__FieldsSpecParams<$struct_type>,| -> $crate::Result<::std::vec::Vec<(&::std::primitive::str, ::std::option::Option<$crate::core::FormattedFailure>)>> {
                $crate::Result::Ok(vec![$(
                    (
                        stringify!($field_name),
                        if params.negated {
                            match $crate::core::DynTransformMatch::match_neg(::std::boxed::Box::new($matcher), params.actual.$field_name)? {
                                $crate::core::MatchOutcome::Success(_) => ::std::option::Option::None,
                                $crate::core::MatchOutcome::Fail(fail) => ::std::option::Option::Some(fail),
                            }
                        } else {
                            match $crate::core::DynTransformMatch::match_pos(::std::boxed::Box::new($matcher), params.actual.$field_name)? {
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
