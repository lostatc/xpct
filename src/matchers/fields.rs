use std::any::type_name;
use std::fmt;

use crate::core::{DynMatchFailure, MatchBase, MatchPos, MatchResult};
use crate::{fail, success};

use super::CombinatorMode;

/// A pairing of field names to optional match failures.
///
/// This can be used by matchers that test each field of a struct or tuple.
pub type FailuresByField = Vec<(&'static str, Option<DynMatchFailure>)>;

/// A matcher for matching on fields of a struct.
///
/// See [`match_fields`] for details.
///
/// [`match_fields`]: crate::match_fields
pub struct FieldMatcher<'a, T> {
    func: Box<dyn FnOnce(T) -> anyhow::Result<FailuresByField> + 'a>,
    mode: CombinatorMode,
}

impl<'a, T> fmt::Debug for FieldMatcher<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FieldMatcher")
            .field(
                "func",
                &type_name::<Box<dyn FnOnce(T) -> anyhow::Result<FailuresByField> + 'a>>(),
            )
            .field("mode", &self.mode)
            .finish()
    }
}

impl<'a, T> FieldMatcher<'a, T> {
    /// Create a new matcher.
    ///
    /// This accepts a function which is passed the struct and returns any failures along with
    /// their field names. You can use the [`fields`] macro to generate a function of this type.
    pub fn new(
        mode: CombinatorMode,
        func: impl FnOnce(T) -> anyhow::Result<FailuresByField> + 'a,
    ) -> Self {
        Self {
            func: Box::new(func),
            mode,
        }
    }
}

impl<'a, T> MatchBase for FieldMatcher<'a, T> {
    type In = T;
}

impl<'a, T> MatchPos for FieldMatcher<'a, T> {
    type PosOut = ();
    type PosFail = FailuresByField;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        let failures = (self.func)(actual)?;
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
}

/// Apply matchers to multiple struct fields.
///
/// This macro is meant to be used with matchers like [`match_fields`] and [`match_any_fields`]. It
/// provides a Rust-like syntax for mapping struct fields to matchers. The syntax looks like this:
///
/// ```
/// # use xpct::{fields, equal, be_gt};
/// # struct Person {
/// #     name: String,
/// #     age: u32,
/// # }
/// fields!(Person {
///     name: equal("Jean Vicquemare"),
///     age: be_gt(34),
/// });
/// ```
///
/// This macro also supports tuple structs; the syntax is identical, except you replace the field
/// names with indices.
///
/// ```
/// # use xpct::{fields, equal};
/// # struct Point(u32, u32);
/// fields!(Point {
///     0: equal(41),
///     1: equal(57),
/// });
/// ```
///
/// This syntax looks more like the Rust syntax for regular structs than tuple structs because it
/// allows you to skip fields:
///
/// ```
/// # use xpct::{fields, equal};
/// # struct Point(u32, u32);
/// fields!(Point {
///     1: equal(57),
/// });
/// ```
///
/// This macro returns a value that can be passed to [`FieldMatcher::new`].
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
        |input: $struct_type| -> ::anyhow::Result<::std::vec::Vec<(&::std::primitive::str, ::std::option::Option<$crate::core::DynMatchFailure>)>> {
            ::anyhow::Result::Ok(vec![$(
                (
                    stringify!($field_name),
                    match $crate::core::DynMatchPos::match_pos(::std::boxed::Box::new($matcher), input.$field_name)? {
                        $crate::core::MatchResult::Success(_) => ::std::option::Option::None,
                        $crate::core::MatchResult::Fail(fail) => ::std::option::Option::Some(fail),
                    },
                ),
            )+])
        }
    };
}
