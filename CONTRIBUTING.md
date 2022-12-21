# Contributing

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
