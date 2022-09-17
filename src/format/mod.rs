#![cfg(feature = "fmt")]
#![cfg_attr(docsrs, doc(cfg(feature = "fmt")))]

mod all;
mod any;
mod each;
mod equal;
mod fields;
mod not;
mod why;

pub use all::AllFormat;
pub use any::{AllFailuresFormat, AnyFormat};
pub use each::{EachFormat, SomeFailuresFormat};
pub use equal::EqualFormat;
pub use fields::{ByFieldFormat, ByFieldMatcherFormat};
pub use not::FailFormat;
pub use why::WhyFormat;

pub(crate) mod matchers {
    pub use super::all::all;
    pub use super::any::any;
    pub use super::each::each;
    pub use super::equal::equal;
    pub use super::fields::{match_all_fields, match_any_fields};
    pub use super::not::not;
    pub use super::why::{why, why_lazy};
}
