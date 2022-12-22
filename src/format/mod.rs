//! The formatters provided by this crate.
//!
//! The formatters in this module can be used to implement custom matchers without having to
//! manually implement their formatting logic. You can use these formatters to get pretty
//! formatting "for free."
//!
//! The [`Format::Value`] of a formatter tells you what failure output it accepts. For example,
//! [`MismatchFormat`] can format any matcher that returns a [`Mismatch`].
//!
//! [`Format::Value`]: crate::core::Format::Value
//! [`Mismatch`]: crate::matchers::Mismatch

#![cfg(feature = "fmt")]
#![cfg_attr(docsrs, doc(cfg(feature = "fmt")))]

mod all;
mod any;
mod boolean;
mod default;
mod each;
mod equal;
mod fields;
mod map;
mod not;
mod option;
mod ord;
mod result;
mod substr;
mod why;

pub use any::HeaderFormat;
pub use boolean::MessageFormat;
pub use each::SomeFailuresFormat;
pub use equal::MismatchFormat;
pub use fields::ByFieldFormat;
pub use not::FailureFormat;
pub use why::WhyFormat;

pub(crate) mod matchers {
    pub use super::all::all;
    pub use super::any::any;
    pub use super::boolean::{be_false, be_true};
    pub use super::default::be_default;
    pub use super::each::each;
    pub use super::equal::equal;
    pub use super::fields::{match_any_fields, match_fields};
    pub use super::map::{map, try_map};
    pub use super::not::not;
    pub use super::option::{be_none, be_some};
    pub use super::ord::{be_ge, be_gt, be_le, be_lt};
    pub use super::result::{be_err, be_ok};
    pub use super::substr::contain_substr;
    pub use super::why::{why, why_lazy};
}
