/*!
Cargo Features

[↩︎ Back to User Docs](crate::docs)

This is a list of cargo features exposed by this crate.

- `regex`: Enables the [`match_regex`] matcher, which requires additional
dependencies.
- `json`: Enables the [`match_json`] matcher, which requires additional
dependencies.
- `color` (default): Enable colors and text styles in the output. This is enabled by default.
Disabling it does not change the API, but does remove some dependencies. Text colors and styles are
always disabled when stderr is not a tty or when the [`NO_COLOR`](https://no-color.org/) environment
variable is set.
- `fmt` (default): Enable the default formatters for the builtin matchers. This is enabled by
default. You do not want to disable this feature unless you want reimplement **all** the builtin
formatters yourself.

[`match_regex`]: crate::match_regex
[`match_json`]: crate::match_json
*/
