# xpct

🚧 This repo is under construction 🚧

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

```rust,should_panic
use xpct::{expect, equal};

expect!("disco").to(equal("Disco"));
```

```text
[src/main.rs:6:5] = "disco"
    Expected:
        "disco"
    to equal:
        "Disco"
```

[*Screenshot*](./examples/example_1.png)

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

```text
[src/main.rs:8:5] = value
    Expected all of these to match:
        [0]  MATCHED
        
        [1]  FAILED
             Expected:
                 "Disco"
             to not equal:
                 "Disco"

        [2]  MATCHED
```

[*Screenshot*](./examples/example_2.png)

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

```text
[src/main.rs:22:5] = get_person()
    Expected all of these to match:
        xpct::main::Person {
            name: FAILED
                🛈 this is a required field
                Expected this to be Some(_)
            id: MATCHED
            age: FAILED
                Expected:
                    44
                to be less than:
                    40
            is_superstar: MATCHED
        }
```

[*Screenshot*](./examples/example_3.png)
