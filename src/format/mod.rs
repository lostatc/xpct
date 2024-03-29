//! The formatters provided by this crate.
//!
//! The formatters in this module can be used to implement custom matchers without having to
//! manually implement their formatting logic. You can use these formatters to get pretty formatting
//! for free.
//!
//! If you're just writing tests and not writing custom matchers or formatters, you don't need
//! anything in this module.
//!
//! The [`Format::Value`] of a formatter tells you what failure output it accepts. For example,
//! [`MismatchFormat`] can format any matcher that returns a [`Mismatch`].
//!
//! See [Writing Custom Matchers][crate::docs::writing_matchers] to learn how to implement your own
//! matchers that use these provided formatters.
//!
//! See [Writing Custom Formatters][crate::docs::writing_formatters] to learn how to implement your
//! own formatters like the ones in this module.
//!
//! [`Format::Value`]: crate::core::Format::Value
//! [`Mismatch`]: crate::matchers::Mismatch

#![cfg(feature = "fmt")]

mod all;
mod any;
mod boolean;
mod casefold;
mod contain;
mod default;
#[cfg(feature = "diff")]
mod diffing;
mod each;
mod elements;
mod equal;
mod every;
mod fields;
mod file;
mod float;
mod json;
mod len;
mod map;
mod not;
mod option;
mod ord;
mod pattern;
mod regex;
mod result;
mod substr;
mod time;
mod why;
mod zero;

/// Types for styling formatted diffs.
#[cfg(feature = "diff")]
pub mod diff {
    pub use super::diffing::{
        CollectionDiffStyle, DiffFormat, DiffSegmentStyle, DiffStyle, StringDiffStyle,
    };
}

pub use any::HeaderFormat;
pub use boolean::MessageFormat;
pub use each::SomeFailuresFormat;
pub use equal::MismatchFormat;
pub use fields::ByFieldFormat;
pub use map::InfallibleFormat;
pub use not::FailureFormat;
pub use option::ExpectationFormat;
pub use why::WhyFormat;

#[cfg(feature = "diff")]
pub use diffing::DiffFormat;

pub(crate) mod matchers {
    pub use super::all::all;
    pub use super::any::any;
    pub use super::boolean::{be_false, be_true};
    pub use super::contain::{be_in, consist_of, contain_element, contain_elements};
    pub use super::default::be_default;
    pub use super::each::each;
    pub use super::elements::match_elements;
    pub use super::equal::equal;
    pub use super::every::every;
    pub use super::fields::{match_any_fields, match_fields};
    pub use super::file::{be_directory, be_existing_file, be_regular_file, be_symlink};
    pub use super::len::{be_empty, have_len};
    pub use super::map::{into, iter_map, iter_try_map, map, try_into, try_map};
    pub use super::not::not;
    pub use super::option::{be_none, be_some};
    pub use super::ord::{be_ge, be_gt, be_le, be_lt, be_sorted_asc, be_sorted_by, be_sorted_desc};
    pub use super::pattern::match_pattern;
    pub use super::result::{be_err, be_ok};
    pub use super::substr::{contain_substr, have_prefix, have_suffix};
    pub use super::time::approx_eq_time;
    pub use super::why::{why, why_lazy};
    pub use super::zero::be_zero;

    #[cfg(feature = "diff")]
    pub use super::diffing::eq_diff;

    #[cfg(feature = "casefold")]
    pub use super::casefold::eq_casefold;

    #[cfg(feature = "float")]
    pub use super::float::{approx_eq_f32, approx_eq_f64};

    #[cfg(feature = "json")]
    pub use super::json::match_json;

    #[cfg(feature = "regex")]
    pub use super::regex::match_regex;
}
