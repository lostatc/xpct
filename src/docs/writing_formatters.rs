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
However, you can instead implement the more generic [`Format`] trait, making its
[`Format::Value`] a [`MatchFailure`], which will automatically implement
[`MatcherFormat`] via a blanket impl.

Let's write a simple formatter that accepts a [`Mismatch`] and prints that the
two values are not equal. This is an example; in practice, you can just use
[`MismatchFormat`] for this.

```
use std::fmt;
use std::marker::PhantomData;

use xpct::core::{Format, Formatter, MatchFailure};
use xpct::matchers::Mismatch;

pub struct NotEqualFormat<Actual, Expected> {
    marker: PhantomData<(Actual, Expected)>,
}

impl<Actual, Expected> NotEqualFormat<Actual, Expected> {
    pub fn new() -> Self {
        Self { marker: PhantomData }
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
```

Now that we've written a custom formatter, we can make our own [`equal`] matcher
that uses our custom formatter without rewriting the matcher logic; we can just
reuse the existing [`EqualMatcher`]!

```
# use std::marker::PhantomData;
# use xpct::core::{Format, Formatter, MatchFailure};
# use xpct::matchers::Mismatch;
# struct NotEqualFormat<A, E>(PhantomData<(A, E)>);
# impl<A, E> NotEqualFormat<A, E> {
#     fn new() -> Self { NotEqualFormat(PhantomData) }
# }
# impl<A, E> Format for NotEqualFormat<A, E> {
#     type Value = MatchFailure<Mismatch<A, E>>;
#     fn fmt(self, f: &mut Formatter, value: Self::Value) -> xpct::Result<()> {
#         Ok(())
#     }
# }
use std::fmt;

use xpct::core::Matcher;
use xpct::matchers::EqualMatcher;

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

## Colors and text styles

Formatters also support styling the output with colors and text styles using the
[`Formatter::set_style`] and [`Formatter::reset_style`] methods.

Colors and text styles are never emitted when stderr is not a tty or when the
[`NO_COLOR`](https://no-color.org/) environment variable is set. You can also
remove support for colors and text styles by disabling the default `color` Cargo
feature. See [Cargo Features][crate::docs::cargo_features] for information.

Colors and text styles can be useful to make the output easier to read, but they
should not convey any information that's not already in the text. Some
developers may not experience color the same way you do, some developers use
screen readers, and some developers just prefer to have colors disabled.

## Composing formatters

If your matcher composes other matchers, it will likely pass a
[`FormattedFailure`] to the formatter, which represents the formatted output of
those matchers. You can use [`Formatter::write_fmt`] to efficiently pass this
through to your formatter's output.

You can also indent the output of the inner matcher using
[`FormattedFailure::into_indented`] like this:

```
# use xpct::core::{Formatter, FormattedFailure};
# fn fmt(f: &mut Formatter, failure: FormattedFailure) {
f.write_fmt(failure.into_indented(4));
# }
```

Printing the output of nested matchers on a separate line and indenting them is
a strategy the provided formatters use to make sure that formatters can compose
each other nicely without having to worry about what the output of the inner
formatter actually looks like.

If you really hate the default formatters and you want to replace all the
provided formatters in this module with your own, you can disable the default
`fmt` Cargo feature. Again, there's more info on the [Cargo
Features][crate::docs::cargo_features] page.

[`equal`]: crate::equal
[`EqualMatcher`]: crate::matchers::EqualMatcher
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
[`FormattedFailure::into_indented`]: crate::core::FormattedFailure::into_indented
*/
