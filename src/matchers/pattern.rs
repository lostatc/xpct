use std::fmt;

use crate::core::SimpleMatch;

use super::Mismatch;

/// A runtime representation of a `match` pattern.
///
/// You can get a value of this type using the [`pattern!`] macro.
///
/// Printing this value with [`Debug`] or [`Display`] prints the stringified pattern.
///
/// [`pattern!`]: crate::pattern
/// [`match_pattern`]: crate::match_pattern
/// [`Debug`]: std::fmt::Debug
/// [`Display`]: std::fmt::Display
pub struct Pattern<'a, T> {
    pattern: &'static str,
    matches: Box<dyn for<'b> Fn(&'b T) -> bool + 'a>,
}

impl<'a, T> fmt::Debug for Pattern<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.pattern)
    }
}

impl<'a, T> fmt::Display for Pattern<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.pattern)
    }
}

impl<'a, T> Pattern<'a, T> {
    /// This method is an implementation detail of the [`pattern!`][crate::pattern] macro and IS NOT
    /// part of the public API.
    #[doc(hidden)]
    pub fn __new(pattern: &'static str, matches: impl for<'b> Fn(&'b T) -> bool + 'a) -> Self {
        Self {
            pattern,
            matches: Box::new(matches),
        }
    }

    /// Returns whether the given value matches the pattern.
    ///
    /// # Examples
    ///
    /// ```
    /// use xpct::pattern;
    ///
    /// let pat = pattern!(Some(value) if *value == "foo");
    ///
    /// assert!(pat.matches(&Some("foo")));
    /// ```
    pub fn matches(&self, value: &T) -> bool {
        (self.matches)(value)
    }
}

/// The matcher for [`match_pattern`].
///
/// [`match_pattern`]: crate::match_pattern
#[derive(Debug)]
pub struct PatternMatcher<'a, Actual> {
    spec: Pattern<'a, Actual>,
}

impl<'a, Actual> PatternMatcher<'a, Actual> {
    /// Create a new [`PatternMatcher`] from the given spec.
    ///
    /// This accepts a [`Pattern`], which you can generate using the [`pattern!`][crate::pattern]
    /// macro.
    pub fn new(spec: Pattern<'a, Actual>) -> Self {
        Self { spec }
    }
}

impl<'a, 'b: 'a, Actual> SimpleMatch<Actual> for PatternMatcher<'a, Actual> {
    type Fail = Mismatch<Pattern<'a, Actual>, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok((self.spec.matches)(actual))
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            expected: self.spec,
            actual,
        }
    }
}

/// Construct a new [`Pattern`] value from a `match` pattern.
///
/// This macro is commonly used with the [`match_pattern`] matcher to test if an expression matches
/// a pattern.
///
/// This macro supports the `pattern_1 | pattern_2` syntax supported by `match` arms:
///
/// ```
/// use xpct::pattern;
/// use xpct::matchers::Pattern;
///
/// let pat: Pattern<Option<&str>> = pattern!(Some("foo") | Some("bar"));
/// ```
///
/// Match guards are also supported:
///
/// ```
/// use xpct::pattern;
/// use xpct::matchers::Pattern;
///
/// let pat: Pattern<Option<&str>> = pattern!(Some(value) if *value == "foo");
/// ```
///
/// [`match_pattern`]: crate::match_pattern
#[macro_export]
macro_rules! pattern {
    ($(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        $crate::matchers::Pattern::__new(
            stringify!($( $pattern )|+ $( if $guard )?),
            |ref actual| match actual {
                $( $pattern )|+ $( if $guard )? => true,
                _ => false,
            },
        )
    };
}
