# xpct

xpct is an assertions library for Rust. It's designed to be ergonomic,
batteries-included, and test framework agnostic.

```rust
use xpct::{expect, match_fields, fields, equal, be_some, be_none, be_ge, be_true};

struct Person {
    name: Option<String>,
    id: String,
    age: u32,
    is_superstar: bool,
}

let value = Some(Person {
    name: None,
    id: String::from("LTN-2JFR"),
    age: 44,
    is_superstar: true,
});

expect!(value)
    .to(be_some())
    .to(match_fields(fields!(Person {
        name: be_none(),
        id: equal("LTN-2JFR"),
        age: be_ge(44),
        is_superstar: be_true(),
    })));
```

xpct is highly extensible. In addition to allowing you to write custom matchers,
it separates the logic of matchers from how they format their output, meaning
you can:

1. Hook into existing formatters to write custom matchers with pretty output
   without having to worry about formatting.
2. Customize the formatting of existing matchers without having to reimplement
   their logic.

Want to get started? [Check out the
tutorial](https://docs.rs/xpct/latest/xpct/docs/tutorial/index.html).
