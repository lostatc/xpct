[![Tests Workflow Status (main)](https://img.shields.io/github/actions/workflow/status/lostatc/xpct/test.yaml?branch=main&label=Tests&style=for-the-badge&logo=github)](https://github.com/lostatc/xpct/actions/workflows/test.yaml)
[![Crates.io](https://img.shields.io/crates/v/xpct?logo=rust&style=for-the-badge)](https://crates.io/crates/xpct)
[![docs.rs](https://img.shields.io/docsrs/xpct?logo=docs.rs&style=for-the-badge)](https://docs.rs/xpct)

# xpct

xpct is an extensible test assertion library for Rust. It's designed to be
ergonomic, batteries-included, and test framework agnostic.

Want to get started? [Check out the
tutorial](https://docs.rs/xpct/latest/xpct/docs/tutorial/index.html).

## About

xpct is extensible. In addition to allowing you to write custom matchers, it
separates the logic of matchers from how they format their output, meaning you
can:

1. Hook into existing formatters to write custom matchers with pretty output
   without having to worry about formatting.
2. Customize the formatting of existing matchers without having to reimplement
   their logic.

This crate aims to provide many useful matchers out of the box. Check out the
full [list of provided
matchers](https://docs.rs/xpct/latest/xpct/docs/matcher_list/index.html).

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

<img src="https://media.githubusercontent.com/media/lostatc/xpct/main/examples/equality.png" width="60%" alt="stderr from failed assertion" />

[*Image transcript*](https://raw.githubusercontent.com/lostatc/xpct/main/examples/equality.txt)

Unwrapping a `Some` value to make an assertion on the wrapped value:

```rust,should_panic
use xpct::{be_gt, be_some, expect};

expect!(Some(41))
    .to(be_some())
    .to(be_gt(57));
```

<img src="https://media.githubusercontent.com/media/lostatc/xpct/main/examples/chaining.png" width="60%" alt="stderr from failed assertion" />

[*Image transcript*](https://raw.githubusercontent.com/lostatc/xpct/main/examples/chaining.txt)

Making assertions about individual fields of a struct:

```rust,should_panic
use xpct::{be_empty, be_in, be_true, expect, fields, have_prefix, match_fields, not, why};

struct Player {
    id: String,
    name: String,
    level: u32,
    is_superstar: bool,
}

let player = Player {
    id: String::from("REV12-62-05-JAM41"),
    name: String::from(""),
    level: 21,
    is_superstar: false,
};

expect!(player).to(match_fields(fields!(Player {
    id: have_prefix("REV"),
    name: not(be_empty()),
    level: be_in(1..=20),
    is_superstar: why(be_true(), "only superstars allowed"),
})));
```

<img src="https://media.githubusercontent.com/media/lostatc/xpct/main/examples/fields.png" width="80%" alt="stderr from failed assertion" />

[*Image transcript*](https://raw.githubusercontent.com/lostatc/xpct/main/examples/fields.txt)

Making assertions about elements in a collection:

```rust,should_panic
use xpct::{be_in, contain_substr, equal, expect, have_len, match_elements};

let items = vec!["apple", "pear", "banana"];

expect!(items)
    .to(have_len(3))
    .to(match_elements([
        contain_substr("ana"),
        equal("pear"),
        be_in(["mango", "orange"]),
    ]));
```

<img src="https://media.githubusercontent.com/media/lostatc/xpct/main/examples/collections.png" width="80%" alt="stderr from failed assertion" />

[*Image transcript*](https://raw.githubusercontent.com/lostatc/xpct/main/examples/collections.txt)

Showing rich diffs of data structures:

```rust,should_panic
use xpct::{eq_diff, expect};

expect!(["apple", "banana"]).to(eq_diff(["banana", "orage"]));
```

<img src="https://media.githubusercontent.com/media/lostatc/xpct/main/examples/diff.png" width="80%" alt="stderr from failed assertion" />

[*Image transcript*](https://raw.githubusercontent.com/lostatc/xpct/main/examples/diff.txt)

## MSRV Policy

The last two stable Rust releases are supported. Older releases may be supported
as well.

The MSRV will only be increased when necessary—not every time there is a new
Rust release. An increase in the MSRV will be accompanied by a minor semver bump
if >=1.0.0 or a patch semver bump if <1.0.0.

## Semver Policy

Prior to version 1.0.0, breaking changes will be accompanied by a minor version
bump, and new features and bug fixes will be accompanied by a patch version
bump.
