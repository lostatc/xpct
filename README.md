[![Tests Workflow Status (main)](https://img.shields.io/github/actions/workflow/status/lostatc/xpct/test.yaml?branch=main&label=Tests&style=for-the-badge&logo=github)](https://github.com/lostatc/xpct/actions/workflows/test.yaml)

# xpct

🚧 This repo is under construction 🚧

xpct is an assertions library for Rust. It's designed to be ergonomic,
batteries-included, and test framework agnostic.

## About

xpct is extensible. In addition to allowing you to write custom matchers, it
separates the logic of matchers from how they format their output, meaning you
can:

1. Hook into existing formatters to write custom matchers with pretty output
   without having to worry about formatting.
2. Customize the formatting of existing matchers without having to reimplement
   their logic.

Want to get started? [Check out the
tutorial](https://docs.rs/xpct/latest/xpct/docs/tutorial/index.html).

*How do you pronounce "xpct"?*

However you choose to pronounce it is how it's pronounced! I pronounce it like
"expect."

## Examples

```rust,should_panic
use xpct::{expect, equal};

expect!("disco").to(equal("Disco"));
```

```text
[src/main.rs:4:5] = "disco"
    Expected:
        "disco"
    to equal:
        "Disco"
```

```rust,should_panic
use xpct::{any, contain_substr, equal, expect, match_regex};

let location = String::from("Central Jamrock");

expect!(location).to_not(any(|ctx| {
    ctx.borrow::<str>()
        .to(match_regex("^(The )?Pox$"))
        .to(contain_substr("Jamrock"))
        .to(equal("Martinaise"));
}));
```

```text
[src/main.rs:6:5] = location
    Expected all of these to be OK:
        [0]  OK
        
        [1]  FAILED
             Expected:
                 "Central Jamrock"
             to not contain the substring:
                 "Jamrock"

        [2]  OK
```

```rust,should_panic
use xpct::{
    all, be_empty, be_gt, be_ok, be_some, be_true, expect, fields, have_prefix, match_fields,
    why,
};

struct Person {
    name: Option<String>,
    age: u32,
    id: String,
    is_superstar: bool,
}

fn get_person() -> anyhow::Result<Person> {
    Ok(Person {
        name: None,
        age: 44,
        id: String::from("12-62-05-JAM41"),
        is_superstar: true,
    })
}

expect!(get_person())
    .to(be_ok())
    .to(match_fields(fields!(Person {
        name: all(|ctx| ctx
            .to(be_some())?
            .to_not(be_empty())
        ),
        age: be_gt(25),
        id: why(have_prefix("REV"), "all IDs must have this prefix"),
        is_superstar: be_true(),
    })));
```

```text
[src/main.rs:22:5] = get_person()
    Expected all of these to be OK:
        my_crate::main::Person {
            name: FAILED
                Expected this to be Some(_)
            age: OK
            id: FAILED
                🛈 all IDs must have this prefix
                Expected:
                    "12-62-05-JAM41"
                to have the prefix:
                    "REV"
            is_superstar: OK
        }
```
