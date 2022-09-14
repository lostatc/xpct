mod all;
mod any;
mod each;
mod equal;
mod not;
mod why;

#[cfg(feature = "fmt")]
pub mod format;

#[cfg(feature = "fmt")]
pub use {
    all::all,
    any::any,
    each::each,
    equal::equal,
    not::not,
    why::{why, why_lazy},
};

pub mod matcher {
    pub use super::all::{AllAssertion, AllMatcher};
    pub use super::any::{
        AllFailures, AnyContext, AnyMatcher, ByRefAnyAssertion, ClonedAnyAssertion,
        CopiedAnyAssertion, SomeFailures,
    };
    pub use super::each::{
        ByRefEachAssertion, ClonedEachAssertion, CopiedEachAssertion, EachContext, EachMatcher,
    };
    pub use super::equal::{EqualMatcher, Mismatch};
    pub use super::not::NotMatcher;
}
