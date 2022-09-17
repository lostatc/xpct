# Contributing

When building the documentation normally, markers which identify which features
are required to use various parts of the library will be missing. That is
because this is an [unstable
feature](https://github.com/rust-lang/rust/issues/43781) of rustdoc that
happens to be enabled in docs.rs. To build the documentation correctly, run the
following command:

```shell
RUSTDOCFLAGS='--cfg docsrs' cargo +nightly doc --all-features
```