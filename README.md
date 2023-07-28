[![Tests Workflow Status (main)](https://img.shields.io/github/actions/workflow/status/lostatc/xpct/test.yaml?branch=main&label=Tests&style=for-the-badge&logo=github)](https://github.com/lostatc/xpct/actions/workflows/test.yaml)
[![Crates.io](https://img.shields.io/crates/v/xpct?logo=rust&style=for-the-badge)](https://crates.io/crates/xpct)
[![docs.rs](https://img.shields.io/docsrs/xpct?logo=docs.rs&style=for-the-badge)](https://docs.rs/xpct)

# xpct

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

## Docs

- [API Docs](https://docs.rs/xpct/latest/xpct/index.html)
- [User Docs](https://docs.rs/xpct/latest/xpct/docs/index.html)
  - [Tutorial](https://docs.rs/xpct/latest/xpct/docs/tutorial/index.html)
  - [Provided Matchers](https://docs.rs/xpct/latest/xpct/docs/matcher_list/index.html)
  - [Cargo Features](https://docs.rs/xpct/latest/xpct/docs/cargo_features/index.html)
  - [Writing Custom Matchers](https://docs.rs/xpct/latest/xpct/docs/writing_matchers/index.html)
  - [Writing Custom Formatters](https://docs.rs/xpct/latest/xpct/docs/writing_formatters/index.html)

## Examples

A simple equality assertion, like `assert_eq`:

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

Unwrapping a `Some` value to make an assertion on the wrapped value:

```rust,should_panic
use xpct::{be_gt, be_some, expect};

expect!(Some(41))
    .to(be_some())
    .to(be_gt(57));
```

```text
[src/main.rs:6:5] = Some(41)
    Expected:
        41
    to be greater than:
        57
```

Making assertions about individual fields of a struct:

```rust,should_panic
use xpct::{
    be_empty, be_gt, be_ok, be_true, expect, fields, have_prefix, match_fields, match_regex,
    not, why,
};

struct Person {
    id: String,
    name: String,
    age: u32,
    is_superstar: bool,
}

let person = Person {
    id: String::from("LTN-2JFR"),
    name: String::new(),
    age: 44,
    is_superstar: false,
};

expect!(person).to(match_fields(fields!(Person {
    id: match_regex(r"^\w{3}(-\dJFR)?$"),
    name: why(not(be_empty()), "this is a required field"),
    age: be_gt(0),
    is_superstar: be_true(),
})));
```

```text
[src/main.rs:23:5] = person
    Expected all of these fields to succeed:
        my_crate::main::Person {
            id: OK
            name: FAILED
                🛈 this is a required field
                Expected:
                    ""
                to not be empty
            age: OK
            is_superstar: FAILED
                Expected this to be true
        }

```

## MSRV Policy

The last two stable Rust releases are supported. Older releases may be supported
as well.

The MSRV will only be increased when necessary to take advantage of new Rust
features—not every time there is a new Rust release. An increase in the MSRV
will be accompanied by a minor semver bump if >=1.0.0 or a patch semver bump if
<1.0.0.

## Semver Policy

Prior to version 1.0.0, breaking changes will be accompanied by a minor version
bump, and new features and bug fixes will be accompanied by a patch version
bump.
