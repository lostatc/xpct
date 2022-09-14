mod matchers;

pub mod core;

#[cfg(feature = "fmt")]
pub use matchers::format;

pub use matchers::matcher;
pub use matchers::*;
