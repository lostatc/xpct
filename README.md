# xpct

xpct is an assertions library for Rust. It's designed to be ergonomic,
batteries-included, and test framework agnostic.

## About

xpct is highly extensible. In addition to allowing you to write custom matchers,
it separates the logic of matchers from how they format their output, meaning
you can:

1. Hook into existing formatters to write custom matchers with pretty output
   without having to worry about formatting.
2. Customize the formatting of existing matchers without having to reimplement
   their logic.

Want to get started? [Check out the
tutorial](https://docs.rs/xpct/latest/xpct/docs/tutorial/index.html).

*How do you pronounce "xpct"?*

However you choose pronounce it is how it's pronounced. I pronounce it like
"expect."

## Examples

```rust
use xpct::{expect, equal};

expect!("disco").to(equal("Disco"));
```

![Example 1 output](./examples/example_1.png)

[*Plain-text transcript*](./examples/example_1.txt)

```rust,should_panic
use xpct::{any, equal, expect, map, not, why};

let value = String::from("Disco");

expect!(value).to(not(any(|ctx| {
    ctx.borrow::<str>()
        .to(equal("Superstar"))
        .to(equal("Disco"))
        .to(equal("Sorry"));
})));
```

![Example 2 output](./examples/example_2.png)

[*Plain-text transcript*](./examples/example_2.txt)

```rust,should_panic
use xpct::{all, be_lt, be_ok, be_some, be_true, equal, expect, fields, match_fields, why};

struct Person {
    name: Option<String>,
    id: String,
    age: u32,
    is_superstar: bool,
}

fn get_person() -> anyhow::Result<Person> {
    Ok(Person {
        name: None,
        id: String::from("LTN-2JFR"),
        age: 44,
        is_superstar: true,
    })
}

expect!(get_person())
    .to(be_ok())
    .to(match_fields(fields!(Person {
        name: why(
            all(|ctx| ctx.to(be_some())?.to(equal("Dick Mullen"))),
            "this is a required field",
        ),
        id: equal("LTN-2JFR"),
        age: be_lt(40),
        is_superstar: be_true(),
    })));
```

![Example 3 output](./examples/example_3.png)

[*Plain-text transcript*](./examples/example_3.txt)
