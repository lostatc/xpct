mod assertion;
mod base;
mod color;
mod formatter;
mod formatter_color;

pub(crate) mod strings;
pub(crate) mod style;

pub use assertion::DefaultAssertionFormat;
pub use base::{AssertionFormat, DispatchFormat, Format, NegFormat, ResultFormat};
pub use color::{Color, OutputStyle, TextColor, TextStyle};

#[cfg(feature = "color")]
pub use formatter_color::{FormattedOutput, Formatter};

#[cfg(not(feature = "color"))]
pub use formatter::{FormattedOutput, Formatter};
