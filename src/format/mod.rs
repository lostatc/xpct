mod assertion;
mod base;
mod color;
mod formatter;
mod formatter_color;
mod indent;

pub mod style;

pub use base::{AssertionFormat, Format, OutputStream, ResultFormat};

pub use indent::dedent;

#[cfg(feature = "color")]
pub use {
    color::{OutputStyle, TerminalColor, TextColor, TextStyle},
    formatter_color::{FormattedOutput, Formatter},
};

#[cfg(not(feature = "color"))]
pub use formatter::{FormattedOutput, Formatter};

#[cfg(feature = "fmt")]
pub use assertion::DefaultAssertionFormat;
