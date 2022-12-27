# Contributing

## Tests

Matchers, even trivial ones, should have basic tests written to guard against
regressions. Generally, each matcher should have at least four tests written for
it:

```rust,no_run
#[test]
fn succeeds_when_foo() {
    // The matcher should succeed when it is not negated.
}

#[test]
fn succeeds_when_not_foo() {
    // The matcher should succeed when it is negated.
}

#[test]
#[should_panic]
fn fails_when_foo() {
    // The matcher should fail when it is negated.
}

#[test]
#[should_panic]
fn fails_when_not_foo() {
    // The matcher should fail when it is not negated.
}
```

See the tests for the `equals` matcher for an example.

## Docs

When building the documentation locally, markers that identify which features
are required to use various parts of the library will be missing. That is
because this is an [unstable
feature](https://github.com/rust-lang/rust/issues/43781) of rustdoc that
happens to be enabled in docs.rs.

Additionally, the user docs in the `crate::docs` module will not be available
when building the docs locally because they are feature-gated to only build on
docs.rs.

To build the documentation correctly, run the following command:

```shell
RUSTDOCFLAGS='--cfg docsrs' cargo +nightly doc --all-features
```
