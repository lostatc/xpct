use std::fmt;

use crate::core::SimpleMatch;

use super::Mismatch;

/// An opaque type used with [`match_pattern`].
///
/// This type is returned by [`pattern!`] and can be passed to [`match_pattern`].
///
/// [`pattern!`]: crate::pattern
/// [`match_pattern`]: crate::match_pattern
pub struct PatternSpec<'a, T> {
    pattern: Pattern,
    func: Box<dyn for<'b> Fn(&'b T) -> bool + 'a>,
}

/// A printable runtime representation of a pattern.
pub struct Pattern {
    pattern: &'static str,
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.pattern)
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.pattern)
    }
}

impl<'a, T> fmt::Debug for PatternSpec<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PatternSpec")
            .field("pattern", &self.pattern)
            .finish_non_exhaustive()
    }
}

impl<'a, T> PatternSpec<'a, T> {
    /// This method is an implementation detail of the [`pattern!`][crate::pattern] macro and IS NOT
    /// part of the public API.
    #[doc(hidden)]
    pub fn __new(pattern: &'static str, func: impl for<'b> Fn(&'b T) -> bool + 'a) -> Self {
        Self {
            pattern: Pattern { pattern },
            func: Box::new(func),
        }
    }
}

/// The matcher for [`match_pattern`].
///
/// [`match_pattern`]: crate::match_pattern
#[derive(Debug)]
pub struct PatternMatcher<'a, Actual> {
    spec: PatternSpec<'a, Actual>,
}

impl<'a, Actual> PatternMatcher<'a, Actual> {
    /// Create a new [`PatternMatcher`] from the given spec.
    ///
    /// This accepts a [`PatternSpec`], which you can generate using the
    /// [`pattern!`][crate::pattern] macro.
    pub fn new(spec: PatternSpec<'a, Actual>) -> Self {
        Self { spec }
    }
}

impl<'a, 'b: 'a, Actual> SimpleMatch<Actual> for PatternMatcher<'a, Actual> {
    type Fail = Mismatch<Pattern, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        Ok((self.spec.func)(actual))
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            expected: self.spec.pattern,
            actual,
        }
    }
}

/// Construct a pattern to use with the [`match_pattern`] matcher.
///
/// This macro is used with [`match_pattern`] to test if an expression matches a pattern. It returns
/// an opaque [`PatternSpec`] value.
///
/// This macro supports the `pattern_1 | pattern_2` syntax supported by `match` arms, as well as
/// match guards.
///
/// [`match_pattern`]: crate::match_pattern
#[macro_export]
macro_rules! pattern {
    ($(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        $crate::matchers::PatternSpec::__new(
            stringify!($( $pattern )|+ $( if $guard )?),
            |ref actual| match actual {
                $( $pattern )|+ $( if $guard )? => true,
                _ => false,
            },
        )
    };
}
