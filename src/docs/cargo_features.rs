/*!
# Cargo Features

A list of all the Cargo features exposed by this crate.

[↩︎ Back to User Docs](crate::docs)

## `regex`

Enables the [`match_regex`] matcher, which requires additional dependencies.

## `json`

Enables the [`match_json`] matcher, which requires additional dependencies.

## `float`

Enables the [`approx_eq_f32`] and [`approx_eq_f64`] matchers, which require
additional dependencies.

## `casefold`

Enables the [`eq_casefold`] matcher, which requires additional dependencies.

## `color` *(default)*

Enable colors and text styles in the output. This is enabled by default.

Disabling this feature does not change the API, but does remove some
dependencies and prevent styles from actually being emitted to stderr. This
means that [`Formatter::set_style`] still exists, but is basically a no-op.

Even with this feature enabled, text colors and styles are disabled when stderr
is not a tty or when the [`NO_COLOR`](https://no-color.org/) environment variable
is set.

## `fmt` *(default)*

Enable the provided formatters. This is enabled by default.

You do not want to disable this feature unless you want reimplement **all** the
provided formatters yourself, which also includes the matcher functions
([`equal`], [`be_some`], etc.), since they rely on these formatters. What this
leaves is [`crate::core`] and [`crate::matchers`]. Disable this if you want to
write "xpct, but with better formatting."

Because of how much of the API surface is not compiled when this feature is
disabled, the flags for it are hidden in docs.rs to reduce noise.

[`match_regex`]: crate::match_regex
[`match_json`]: crate::match_json
[`approx_eq_f32`]: crate::approx_eq_f32
[`approx_eq_f64`]: crate::approx_eq_f64
[`eq_casefold`]: crate::eq_casefold
[`equal`]: crate::equal
[`be_some`]: crate::be_some
[`Formatter::set_style`]: crate::core::Formatter::set_style
*/
