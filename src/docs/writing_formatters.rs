/*!
# Writing Custom Formatters

How to write custom formatters, either for formatting output from your own
matchers, or for changing the formatting of the provided matchers.

[↩︎ Back to User Docs](crate::docs)

## Writing a formatter

Most of the time, when writing matchers, you should be able to reuse existing
formatters in [`crate::format`]. However, if you want to write a formatter for
your custom matcher, or even to change the formatting of an existing matcher,
here's how to do it.

Matcher formatters are types that implement the [`MatcherFormat`] trait.
However, you can instead implement the more generic [`Format`] trait, which will
implement [`MatcherFormat`] via a blanket impl.

Let's write a simple formatter that accepts a [`Mismatch`] and prints that the
two values are not equal. This is an example; in practice, you can just use
[`MismatchFormat`] for this.

We'll also need to write our own [`equal`] function that calls our formatter
instead of the provided one. Notice that we don't need to reimplement me logic
of the matcher; we can just reuse the existing [`EqualMatcher`] and plug in our
custom formatter.

```
use std::fmt;
use std::marker::PhantomData;

use xpct::core::{whitespace, Format, Formatter, MatchFailure, Matcher};
use xpct::matchers::Mismatch;
use xpct::matchers::equal::EqualMatcher;

#[derive(Debug)]
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
    type Value = MatchFailure<Mismatch<Actual, Expected>>;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> xpct::Result<()> {
        match value {
            MatchFailure::Pos(mismatch) => {
                f.write_str("Expected:\n");

                f.indented(whitespace(4), |f| {
                    f.write_str(format!("{:?}", mismatch.actual));

                    Ok(())
                })?;

                f.write_str("to equal:\n");

                f.indented(whitespace(4), |f| {
                    f.write_str(format!("{:?}", mismatch.expected));

                    Ok(())
                })?;
            }
            MatchFailure::Neg(mismatch) => {
                f.write_str("Expected:\n");

                f.indented(whitespace(4), |f| {
                    f.write_str(format!("{:?}", mismatch.actual));

                    Ok(())
                })?;

                f.write_str("to not equal:\n");

                f.indented(whitespace(4), |f| {
                    f.write_str(format!("{:?}", mismatch.expected));

                    Ok(())
                })?;
            }
        };

        Ok(())
    }
}

pub fn equal<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialEq<Expected> + Eq + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::new(EqualMatcher::new(expected), NotEqualFormat::new())
}
```

## Colors and text styles

Formatters also support styling the output with colors and text styles using the
[`Formatter::set_style`] and [`Formatter::reset_style`] methods.

Colors and text styles are never emitted when stderr is not a tty or when the
[`NO_COLOR`](https://no-color.org/) environment variable is set. You can also
remove support for colors and text styles by disabling the default `color` Cargo
feature. See [Cargo Features][crate::docs::cargo_features] for information.

Colors and text styles can be useful to make the output easier to read, but they
should not convey any information that's not already in the text. Some
developers don't experience color the same way you do, some developers use
screen readers, and some developers just prefer to have colors disabled.

## Composing formatters

If your matcher composes other matchers, it will likely pass a
[`FormattedFailure`] to the formatter, which represents the formatted output of
those matchers. You can use [`Formatter::write_fmt`] to efficiently pass this
through to your formatter's output.

If you really hate the default formatters and you want to replace all the
provided formatters in this module with your own, you can disable the default
`fmt` Cargo feature. There's more info on the [Cargo
Features][crate::docs::cargo_features] page.

[`equal`]: crate::equal
[`EqualMatcher`]: crate::matchers::equal::EqualMatcher
[`MatcherFormat`]: crate::core::MatcherFormat
[`Format`]: crate::core::Format
[`Format::Value`]: crate::core::Format::Value
[`MatchFailure`]: crate::core::MatchFailure
[`Mismatch`]: crate::matchers::Mismatch
[`MismatchFormat`]: crate::format::MismatchFormat
[`Formatter::write_fmt`]: crate::core::Formatter::write_fmt
[`Formatter::set_style`]: crate::core::Formatter::set_style
[`Formatter::reset_style`]: crate::core::Formatter::reset_style
[`FormattedFailure`]: crate::core::FormattedFailure
*/
