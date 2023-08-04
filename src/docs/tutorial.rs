/*!
# Tutorial

A brief guide on how to use xpct to write tests.

[↩︎ Back to User Docs](crate::docs)

To make an assertion, you'll usually start with the [`expect!`] macro, which
returns an [`Assertion`].

```
use xpct::{expect, equal};

expect!("Disco").to(equal("Disco"));
```

In the above example, [`equal`] is a *matcher*. This crate provides [a number of
matchers][crate::docs::matcher_list], and you can implement custom matchers as
well.

When an assertion fails, it panics with an error message.

You can also chain matchers like this:

```
use xpct::{expect, be_gt, be_lt};

expect!(41)
    .to(be_gt(0)) // 41 > 0
    .to(be_lt(57)); // 41 < 57
```

When you chain together multiple matchers like this, the assertion only succeeds
if *all* of them succeed.

You can also negate matchers by calling [`Assertion::to_not`] or using the
[`not`] matcher:

```
use xpct::{expect, equal, not};

// These are equivalent.
expect!(41).to_not(equal(57));
expect!(41).to(not(equal(57)));
```

Some matchers are actually just aliases for negating other matchers:

```
use xpct::{expect, be_some, be_none};

let value: Option<&str> = None;

// These are equivalent.
expect!(value).to(be_none());
expect!(value).to_not(be_some());
```

When you chain together matchers, they pass the value you passed to [`expect!`]
into the next matcher in the chain. Matchers can change the type of this value,
which allows some matchers to do things like unwrap [`Result`] and [`Option`]
types.

```
use std::io;
use xpct::{expect, equal, be_ok};

fn location() -> io::Result<String> {
    Ok(String::from("Martinaise"))
}

expect!(location())
    .to(be_ok())
    .to(equal("Martinaise"));
```

In the above example, we don't need to unwrap the [`Result`], because the
[`be_ok`] matcher did it for us! If we were to negate this matcher with [`not`]
(or just use [`be_err`]), then it would return the value of the [`Err`] variant
instead.

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

// We use `try_map` for conversions that can fail.
expect!(vec![0x43, 0x75, 0x6e, 0x6f])
    .try_map(|bytes| Ok(String::from_utf8(bytes)?))
    .to(equal("Cuno"));
```

If you need to convert between types that implement [`From`] or [`TryFrom`], you
can use the matchers [`into`] and [`try_into`] as well as [`Assertion::into`]
and [`Assertion::try_into`].

```
use xpct::{expect, equal};

expect!(41u64)
    .try_into::<u32>()
    .to(equal(41u32));
```

You can always get the value back out at the end of a chain of matchers by
calling [`Assertion::into_inner`]. This lets you use the same value in another
assertion.

```
use xpct::{be_some, equal, expect, have_len};

let name = expect!(["Mañana", "Evrart"])
    .to(have_len(2))
    .into_inner();

expect!(name.first())
    .to(be_some())
    .to(equal(&"Mañana"));
```

There are combinator matchers like [`all`], [`each`], and [`any`] which allow
you to combine matchers in different ways:

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
use xpct::{expect, match_regex, why};

expect!("Kim").to(why(
    match_regex(r"^\p{Lu}"),
    "names should start with a capital letter",
));
```

You can test that a value matches a pattern using the [`match_pattern`] matcher
and the [`pattern!`] macro.

```
use xpct::{expect, match_pattern, pattern};

#[derive(Debug)]
enum Command {
    Create(String),
    Update(String),
    Delete,
}

let command = Command::Create("foo".into());

expect!(command).to(match_pattern(pattern!(
    Command::Create(_) | Command::Delete
)));
```

You can use [`eq_diff`] instead of [`equal`] for any type that implements
[`Diffable`] to print a rich diff when the values are not equal.

```
use xpct::{expect, eq_diff};

expect!("diffing strings").to(eq_diff("diffing strings"));
expect!(["slices", "too"]).to(eq_diff(["slices", "too"]));
// Also sets and maps!
```

If you want to assert on multiple fields of a struct, rather than using a
separate [`expect!`] assertion for each field, you can use [`match_fields`] with
the [`fields!`] macro.

```
use xpct::{be_gt, be_some, be_true, expect, fields, have_prefix, match_fields};

struct Person {
    id: String,
    name: Option<String>,
    age: u32,
    is_superstar: bool,
}

let value = Person {
    id: String::from("REV12-62-05-JAM41"),
    name: Some(String::from("Raphaël")),
    age: 44,
    is_superstar: true,
};

expect!(value).to(match_fields(fields!(Person {
    id: have_prefix("REV"),
    name: be_some(),
    age: be_gt(0),
    is_superstar: be_true(),
})));
```

There are also a number of matchers for dealing with collections. For example,
you can assert that a collection contains certain elements using
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

The [`every`] matcher allows you to test every element in a collection against
the same matcher.

```
use xpct::{be_some, every, expect, have_prefix};

let items = vec![Some("Cuno"), Some("Cindy")];

// Notice it unwraps the `Vec<Option<&str>>` to a `Vec<&str>`.
let unwrapped: Vec<&str> = expect!(items)
    .to(every(be_some))
    .into_inner();
```

The [`match_elements`] matcher allows you to tests each element of a collection
against a different matcher.

```
use xpct::{be_in, equal, expect, have_prefix, match_elements};

let items = vec!["apple", "banana", "cucumber"];

expect!(items).to(match_elements([
    equal("apple"),
    be_in(["banana", "orange"]),
    have_prefix("c"),
]));
```

The matchers for collections are implemented using the [`Len`] and [`Contains`]
traits. You can implement these traits for your own types to use them with the
collections matchers.

Check out [Provided Matchers][crate::docs::matcher_list] for a list of all the
matchers provided by this crate.

[`equal`]: crate::equal
[`eq_diff`]: crate::eq_diff
[`not`]: crate::not
[`be_ok`]: crate::be_ok
[`be_err`]: crate::be_err
[`map`]: crate::map
[`try_map`]: crate::try_map
[`into`]: crate::into
[`try_into`]: crate::try_into
[`all`]: crate::all
[`each`]: crate::each
[`any`]: crate::any
[`why`]: crate::why
[`why_lazy`]: crate::why_lazy
[`match_pattern`]: crate::match_pattern
[`Diffable`]: crate::matchers::Diffable
[`pattern!`]: crate::pattern
[`match_fields`]: crate::match_fields
[`expect!`]: crate::expect
[`fields!`]: crate::fields
[`contain_element`]: crate::contain_element
[`contain_elements`]: crate::contain_elements
[`consist_of`]: crate::consist_of
[`be_in`]: crate::be_in
[`every`]: crate::every
[`match_elements`]: crate::match_elements
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
