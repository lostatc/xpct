/*!
How to write custom matchers.

If none of the provided matchers suit your needs, xpct allows you to write
custom matchers. There are three ways to do this, in increasing order of
complexity and flexibility:

1. Compose existing matchers.
2. Implement [`SimpleMatch`].
3. Implement [`Match`].

# Composing existing matchers

The simplest way to make custom matchers is to just compose existing matchers.
The combinator matchers [`each`], [`any`], and [`all`] are useful for this.

```
use std::fmt;
use xpct::{each, be_lt, be_gt};
use xpct::core::Matcher;

pub fn be_between<'a, Actual, Low, High>(
    low: &'a Low,
    high: &'a High,
) -> Matcher<'a, Actual, Actual>
where
    Actual: PartialOrd<Low> + PartialOrd<High> + fmt::Debug + 'a,
    Low: fmt::Debug,
    High: fmt::Debug,
{
    each(move |ctx| {
        ctx.borrow().to(be_gt(low)).to(be_lt(high));
    })
}
```

# Implementing `SimpleMatch`

The next simplest way is to implement the [`SimpleMatch`] trait. Here's an
implementation of the [`equal`] matcher.

```
use xpct::core::SimpleMatch;
use xpct::matchers::Mismatch;

pub struct EqualMatcher<Expected> {
    expected: Expected,
}

impl<Expected> EqualMatcher<Expected> {
    pub fn new(expected: Expected) -> Self {
        Self { expected }
    }
}

impl<Expected, Actual> SimpleMatch<Actual> for EqualMatcher<Expected>
where
    Actual: PartialEq<Expected> + Eq,
{
    type Fail = Mismatch<Expected, Actual>;

    fn matches(&mut self, actual: &Actual) -> xpct::Result<bool> {
        Ok(actual == &self.expected)
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            actual,
            expected: self.expected,
        }
    }
}

```

Now let's make a function to call this matcher ergonomically from tests!

To make `EqualMatcher` into a `Matcher`, you just need to wrap it with
[`Matcher::simple`]. This method also accepts the formatter which is used to
format the output. Thankfully, you don't need to write the formatting logic
yourself to get pretty output! Because our matcher returns a [`Mismatch`] when
it fails, we can use any formatter which accepts a [`Mismatch`], like the aptly
named [`MismatchFormat`].

```
# use xpct::matchers::EqualMatcher;
use std::fmt;

use xpct::expect;
use xpct::core::Matcher;
use xpct::format::MismatchFormat;

pub fn equal<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialEq<Expected> + Eq + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::simple(
        EqualMatcher::new(expected),
        MismatchFormat::new("to equal", "to not equal"),
    )
}

```

What if we wanted to make a matcher which is the negated version of
`EqualMatcher`, like `not_equal`? For a matcher created by implementing
[`SimpleMatch`], we can call [`Matcher::simple_neg`] to negate it.

```
# use xpct::matchers::EqualMatcher;
use std::fmt;

use xpct::expect;
use xpct::core::Matcher;
use xpct::format::MismatchFormat;

pub fn not_equal<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialEq<Expected> + Eq + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::simple_neg(
        EqualMatcher::new(expected),
        // Remember that we need to flip these cases, because `actual !=
        // expected` is now the *positive* case and `actual == expected` is now
        // the *negative* case.
        MismatchFormat::new("to not equal", "to equal"),
    )
}

expect!("disco").to(not_equal("not disco"));
```

# Implementing `Match`

The major limitation of [`SimpleMatch`] is that it always returns the same value
that was passed in, hence the name "simple." If you have more complex needs for
your matcher, like you need it to transform the value like the [`be_some`] and
[`be_ok`] matchers do, you can implement the [`Match`] trait

```
use std::marker::PhantomData;

use xpct::{success, fail};
use xpct::core::{Matcher, Match, MatchOutcome};

pub struct BeOkMatcher<T, E> {
    // Matchers created by implementing `Match` will often need to use
    // `PhantomData` so they know their input and output types.
    marker: PhantomData<(T, E)>,
}

impl<T, E> BeOkMatcher<T, E> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<T, E> Match for BeOkMatcher<T, E> {
    // The type the matcher accepts.
    type In = Result<T, E>;

    // In the positive case, this should return the `Ok` value.
    type PosOut = T;

    // In the negative case, this should return the `Err` value.
    type NegOut = E;

    // Because we don't have any interesting information to provide other than
    // "the value was not `Ok`" or "the value was not `Err`", we just make these
    // the unit type.
    type PosFail = ();
    type NegFail = ();

    fn match_pos(
        self,
        actual: Self::In,
    ) -> xpct::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        match actual {
            Ok(value) => Ok(MatchOutcome::Success(value)),
            Err(_) => Ok(MatchOutcome::Fail(())),
        }
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> xpct::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        match actual {
            // This crate provides these macros as a shorthand for returning a
            // `MatchOutcome`. You don't have to use them.
            Ok(_) => fail!(()),
            Err(error) => success!(error),
        }
    }
}
```

You'll see the terms "pos" and "neg", short for *positive* and *negative*,
throughout the API. These refer to whether a matcher is negated (negative)
or not negated (positive).

If a matcher is negated (the negative case), it means that we're expecting it to
fail. If a matcher is *not* negated (the positive case), it means we're
expecting it to succeed.

Now let's make some functions for invoking our matcher.

```
# use xpct::matchers::BeOkMatcher;
use xpct::core::{Matcher, NegFormat};
use xpct::format::MessageFormat;

// `MessageFormat` is a simple formatter that just returns a static message in
// each case. It doesn't care what the types of `PosFail` and `NegFail` are.
fn result_format() -> MessageFormat {
    MessageFormat::new("Expected this to be Ok.", "Expected this to be Err.")
}

pub fn be_ok<'a, T: 'a, E: 'a>() -> Matcher<'a, Result<T, E>, T, E> {
    // For matchers implemented with `Match`, you use `Matcher::new`.
    Matcher::new(BeOkMatcher::new(), result_format())
}

pub fn be_err<'a, T: 'a, E: 'a>() -> Matcher<'a, Result<T, E>, E, T> {
    // You can use `Matcher::neg` to negate a matcher created by implementing
    // `Match`. You can use `NegFormat` to negate the formatter.
    Matcher::neg(BeOkMatcher::new(), NegFormat(result_format()))
}
```

[`Match`]: crate::core::Match
[`each`]: crate::each
[`any`]: crate::any
[`all`]: crate::all
[`be_some`]: crate::be_some
[`be_ok`]: crate::be_ok
[`SimpleMatch`]: crate::core::SimpleMatch
[`equal`]: crate::equal
[`Matcher::new`]: crate::core::Matcher::new
[`Mismatch`]: crate::matchers::Mismatch
[`MismatchFormat`]: crate::format::MismatchFormat
[`Matcher::simple`]: crate::core::Matcher::simple
[`Matcher::simple_neg`]: crate::core::Matcher::simple_neg
*/
