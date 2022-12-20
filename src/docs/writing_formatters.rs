/*!
How to write custom formatters.

Most of the time, when writing matchers, you should be able to reuse existing
formatters in this module. However, if you want to write a formatter for your
custom matcher, or even to change the formatting of an existing matcher, here's
how to do it.

Formatters are types that implement the [`Format`] trait. Formatters have a
[`Format::Value`], which is the type passed to them by the matcher. If you want
to implement [`Format`] for use with a matcher, its [`Format::Value`] must be a
[`MatchFailure`].

Let's write a simple formatter that accepts a [`Mismatch`] and prints that the
two values are not equal.

```
use std::fmt;
use std::marker::PhantomData;

use xpct::core::{Format, Formatter, MatchFailure, Matcher};
use xpct::matchers::{Mismatch, EqualMatcher};

pub struct NotEqualFormat<Actual, Expected> {
    marker: PhantomData<(Actual, Expected)>,
}

impl<Actual, Expected> NotEqualFormat<Actual, Expected> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}


impl<Actual, Expected> Format for NotEqualFormat<Actual, Expected>
where
    Actual: fmt::Debug,
    Expected: fmt::Debug,
{
    // `MatchFailure` accepts two type parameters for the positive case (we
    // expected the matcher to match) and the negative case (we expected the
    // matcher to fail) respectively. However, if they have the same type, you
    // can omit the second one like this.
    type Value = MatchFailure<Mismatch<Actual, Expected>>;

    // This trait is similar to `std::fmt::Display` where you call methods on
    // `Formatter` to generate the output.
    fn fmt(self, f: &mut Formatter, value: Self::Value) -> xpct::Result<()> {
        match value {
            MatchFailure::Pos(mismatch) => {
                f.write_str("Expected:\n");
                f.write_str(format!("    {:?}\n", mismatch.actual));
                f.write_str("to equal:\n");
                f.write_str(format!("    {:?}\n", mismatch.expected));
            },
            MatchFailure::Neg(mismatch) => {
                f.write_str("Expected:\n");
                f.write_str(format!("    {:?}\n", mismatch.actual));
                f.write_str("to not equal:\n");
                f.write_str(format!("    {:?}\n", mismatch.expected));
            },
        };

        Ok(())
    }
}

// We can make our own `equal` matcher that uses our custom formatter without
// rewriting the matcher logic; we can just reuse the existing `EqualMatcher`!
pub fn equal<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialEq<Expected> + Eq + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::simple(
        EqualMatcher::new(expected),
        NotEqualFormat::new(),
    )
}
```

Formatters also support styling the output with colors and font styles using the
[`Formatter::set_style`] and [`Formatter::reset_style`] methods. You can disable
stylized terminal output by disabling the default `color` Cargo feature.

If your matcher composes other matchers, it will likely pass a
[`FormattedFailure`] to the formatter, which represents the formatted output of
those matchers. You can use [`Formatter::write_fmt`] to efficiently write this
to your formatter's output. You can also indent the output of the inner matcher
using [`FormattedOutput::indented`] like this:

```
# use xpct::core::{Formatter, FormattedFailure, FormattedOutput};
# fn fmt(f: &mut Formatter, failure: FormattedFailure) {
f.write_fmt(FormattedOutput::from(failure).indented(4));
# }
```

If you really hate the default formatters and you want to replace all the
provided formatters in this module with your own, you can disable the default
`fmt` Cargo feature.

[`Format`]: crate::core::Format
[`Format::Value`]: crate::core::Format::Value
[`MatchFailure`]: crate::core::MatchFailure
[`Mismatch`]: crate::matchers::Mismatch
[`Formatter::write_fmt`]: crate::core::Formatter::write_fmt
[`Formatter::set_style`]: crate::core::Formatter::set_style
[`Formatter::reset_style`]: crate::core::Formatter::reset_style
[`FormattedFailure`]: crate::core::FormattedFailure
[`FormattedOutput::indented`]: crate::core::FormattedOutput::indented
*/
