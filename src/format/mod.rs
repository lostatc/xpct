#![cfg(feature = "fmt")]
#![cfg_attr(docsrs, doc(cfg(feature = "fmt")))]

mod all;
mod any;
mod boolean;
mod each;
mod equal;
mod fields;
mod not;
mod option;
mod ord;
mod why;

pub use all::AllFormat;
pub use any::{AllFailuresFormat, AnyFormat};
pub use boolean::MessageFormat;
pub use each::{EachFormat, SomeFailuresFormat};
pub use equal::MismatchFormat;
pub use fields::{ByFieldFormat, ByFieldMatcherFormat};
pub use not::FailFormat;
pub use option::BeSomeFormat;
pub use why::WhyFormat;

pub(crate) mod matchers {
    pub use super::all::all;
    pub use super::any::any;
    pub use super::boolean::{be_false, be_true};
    pub use super::each::each;
    pub use super::equal::equal;
    pub use super::fields::{match_any_fields, match_fields};
    pub use super::not::not;
    pub use super::option::{be_none, be_some};
    pub use super::ord::{be_ge, be_gt, be_le, be_lt};
    pub use super::why::{why, why_lazy};
}
