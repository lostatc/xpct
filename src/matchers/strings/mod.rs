#[cfg(feature = "casefold")]
mod casefold;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "regex")]
mod regex;
mod substr;

#[cfg(feature = "regex")]
pub use self::regex::RegexMatcher;
#[cfg(feature = "casefold")]
pub use casefold::EqCasefoldMatcher;
#[cfg(feature = "json")]
pub use json::JsonMatcher;
pub use substr::{ContainSubstrMatcher, HavePrefixMatcher, HaveSuffixMatcher};
