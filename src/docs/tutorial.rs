/*!
# Tutorial

A brief guide on how to use xpct to write tests.

[↩︎ Back to User Docs](crate::docs)

To make an assertion, you'll usually start with the [`expect!`] macro, which
returns an [`Assertion`].

```
use xpct::{expect, equal};

expect!("disco").to(equal("disco"));
```

In the above example, [`equal`] is a *matcher*. This crate provides a number of
matchers in the crate root, and you can implement custom matchers as well.

When an assertion fails, it panics with an error message.

You can also chain matchers like this:

```
use xpct::{expect, be_gt, be_lt};

expect!(41)
    .to(be_gt(0)) // 41 > 0
    .to(be_lt(57)); // 41 < 57
```

When you chain together multiple matchers like this, the assertion only succeeds
if *all* of them match.

You can also negate matchers by calling [`Assertion::to_not`] or using the
[`not`] matcher:

```
use xpct::{expect, equal, not};

// These are equivalent.
expect!(41).to_not(equal(57));
expect!(41).to(not(equal(57)));
```

When you chain together matchers, they pass the value you passed to [`expect!`]
into the next matcher in the chain. Matchers can change the type of this value,
which allows some matchers to do things like unwrap [`Result`] and [`Option`]
types.

```
use xpct::{expect, equal, be_ok};

fn location() -> anyhow::Result<String> {
    Ok(String::from("Whirling-in-Rags"))
}

expect!(location())
    .to(be_ok())
    .to(equal("Whirling-in-Rags"));
```

In the above example, we don't need to unwrap the [`Result`], because the
[`be_ok`] matcher did it for us! If we were to negate this matcher with [`not`],
then it would return the value of the [`Err`] variant instead.

If you want to map a value by applying a function to it as part of a chain of
matchers, you can use the matchers [`map`] and [`try_map`] as well as
[`Assertion::map`] and [`Assertion::try_map`].

```
use std::convert::Infallible;

use xpct::{expect, map, try_map, equal};

struct Name(String);

expect!(Name(String::from("Cuno")))
    .map(|name| name.0)
    .to(equal("Cuno"));

let name: Result<_, Infallible> = Ok(String::from("Cuno"));

expect!(name)
    .to(map(Result::unwrap))
    .to(equal("Cuno"));

// We use `try_map` for conversions that can fail.
expect!(vec![0x43, 0x75, 0x6e, 0x6f])
    .try_map(|bytes| Ok(String::from_utf8(bytes)?))
    .to(equal("Cuno"));
```

If you need to convert between types that implement [`From`] or [`TryFrom`], you
can use [`Assertion::into`] and [`Assertion::try_into`].

```
use xpct::{expect, equal};

expect!(41u64)
    .try_into::<u32>()
    .to(equal(41u32));
```

You can always get the value back out at the end of a chain of matchers by
calling [`Assertion::into_inner`].

```
use xpct::{expect, be_some};

let name: &'static str = expect!(Some("Raphaël Ambrosius Costeau"))
    .to(be_some())
    .into_inner();
```

There are combinator matchers like [`all`], [`each`], and [`any`] which allow us
to combine matchers in different ways:

```
use xpct::{expect, any, equal, be_none};

fn description() -> Option<String> {
    None
}

// This is either `None` or `Some("horrific")`.
expect!(description()).to(any(|ctx| {
    ctx.map(Option::as_deref)
        .to(be_none())
        .to(equal(Some("horrific")));
}));

```

If you want to attach additional context to a matcher to include in the failure
output, you can use [`why`] and [`why_lazy`]:

```
use xpct::{expect, why, not, equal};

expect!("Kim Kitsuragi").to(why(
    not(equal("kim kitsuragi")),
    "names should be capitalized"
));
```

If you want to match on multiple fields of a struct, rather than using a
separate [`expect!`] assertion for each field, you can use [`match_fields`] with
the [`fields!`] macro.

```
use xpct::{expect, match_fields, fields, equal, be_none, be_ge, be_true};

struct Person {
    name: Option<String>,
    id: String,
    age: u32,
    is_superstar: bool,
}

let value = Person {
    name: None,
    id: String::from("LTN-2JFR"),
    age: 44,
    is_superstar: true,
};

expect!(value).to(match_fields(fields!(Person {
    name: be_none(),
    id: equal("REV12-62-05-JAM41"),
    age: be_ge(26),
    is_superstar: be_true(),
})));
```

There are also a number of builtin matchers for dealing with collections. For
example, you can assert that a collection contains certain elements using
[`contain_element`] and [`contain_elements`].

```
use xpct::{contain_element, expect};

expect!(["Mañana", "Evrart"]).to(contain_element("Evrart"));
```

You can also use [`consist_of`] to assert that a collection consists of exactly
the given elements, in any order.

```
use xpct::{consist_of, expect};

expect!(&["Mañana", "Evrart"]).to(consist_of(["Evrart", "Mañana"]));
```

The [`be_in`] matcher can test that something is in a collection.

```
use xpct::{be_in, expect};

expect!("Mañana").to(be_in(["Evrart", "Mañana"]));
expect!('C').to(be_in("Cuno"));
expect!(50).to(be_in(41..57));
```

The [`every`] matcher is particularly powerful; it allows you to match every
element in a collection against the same matcher.

```
use xpct::{be_some, every, expect, have_prefix};

let items = vec![Some("Cuno"), Some("Cindy")];

// Notice it unwraps the `Vec<Option<&str>>` to a `Vec<&str>`.
let output: Vec<&str> = expect!(items)
    .to(every(be_some))
    .into_inner();

expect!(&items).to(every(|| have_prefix("C")));
```

The matchers for collections are implemented using the [`Len`] and [`Contains`]
traits. You can implement these traits for your own types to use them with the
collections matchers.

Check out [Builtin Matchers][crate::docs::matcher_list] for a list of all the
matchers provided by this crate.

[`equal`]: crate::equal
[`not`]: crate::not
[`be_ok`]: crate::be_ok
[`map`]: crate::map
[`try_map`]: crate::try_map
[`all`]: crate::all
[`each`]: crate::each
[`any`]: crate::any
[`why`]: crate::why
[`why_lazy`]: crate::why_lazy
[`match_fields`]: crate::match_fields
[`expect!`]: crate::expect
[`fields!`]: crate::fields
[`contain_element`]: crate::contain_element
[`contain_elements`]: crate::contain_elements
[`consist_of`]: crate::consist_of
[`be_in`]: crate::be_in
[`every`]: crate::every
[`Len`]: crate::matchers::Len
[`Contains`]: crate::matchers::Contains
[`Assertion`]: crate::core::Assertion
[`Assertion::to_not`]: crate::core::Assertion::to_not
[`Matcher`]: crate::core::Matcher
[`Assertion::into_inner`]: crate::core::Assertion::into_inner
[`Assertion::map`]: crate::core::Assertion::map
[`Assertion::try_map`]: crate::core::Assertion::try_map
[`Assertion::into`]: crate::core::Assertion::into
[`Assertion::try_into`]: crate::core::Assertion::try_into
*/
