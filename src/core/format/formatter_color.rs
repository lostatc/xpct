#![cfg(feature = "color")]

use std::fmt;

use super::strings::indent_vec;
use super::{Format, OutputStyle};

// Disable colored output if stderr is not a tty.
fn check_disable_color() {
    if atty::isnt(atty::Stream::Stderr) {
        colored::control::set_override(false)
    }
}

#[derive(Debug, Default)]
struct OutputSegment {
    buf: String,
    style: OutputStyle,
}

/// Configuration for formatting with [`Format`].
///
/// This value is passed to [`Format::fmt`] and is used to set various options to configure how the
/// value will be formatted.
///
/// [`Format::fmt`]: crate::core::Format::fmt
#[derive(Debug)]
pub struct Formatter {
    prev: Vec<OutputSegment>,
    current: OutputSegment,
}

impl Formatter {
    fn new() -> Self {
        Self {
            prev: Vec::new(),
            current: Default::default(),
        }
    }

    /// Write a string to the output.
    pub fn write_str(&mut self, s: impl AsRef<str>) {
        self.current.buf.push_str(s.as_ref());
    }

    /// Write a `char` to the output.
    pub fn write_char(&mut self, c: char) {
        self.current.buf.push(c);
    }

    /// Pass some pre-formatted output through to the output.
    ///
    /// This method is often used when writing formatters for matchers which compose other
    /// matchers, such as [`not`] or [`each`]. It's used by formatters such as [`FailureFormat`]
    /// and [`SomeFailuresFormat`].
    ///
    /// [`not`]: crate::not
    /// [`each`]: crate::each
    /// [`FailureFormat`]: crate::format::FailureFormat
    /// [`SomeFailuresFormat`]: crate::format::SomeFailuresFormat
    pub fn write_fmt(&mut self, output: impl Into<FormattedOutput>) {
        let formatted = output.into();
        let new_current = OutputSegment {
            buf: String::new(),
            style: self.current.style.clone(),
        };
        self.prev
            .push(std::mem::replace(&mut self.current, new_current));
        self.prev.extend(formatted.segments);
    }

    /// Get the current [`OutputStyle`].
    pub fn style(&self) -> &OutputStyle {
        &self.current.style
    }

    /// Set the current [`OutputStyle`].
    ///
    /// This is used to configure colors and text styles in the output. Output formatting is
    /// stripped out when stderr is not a tty.
    pub fn set_style(&mut self, style: OutputStyle) {
        self.prev.push(std::mem::replace(
            &mut self.current,
            OutputSegment {
                buf: String::new(),
                style,
            },
        ));
    }

    /// Reset the current colors and text styles to their defaults.
    pub fn reset_style(&mut self) {
        self.set_style(Default::default());
    }
}

/// A value that has been formatted with a [formatter][`Format`].
#[derive(Debug)]
pub struct FormattedOutput {
    segments: Vec<OutputSegment>,
}

impl FormattedOutput {
    /// Create a new [`FormattedOutput`] by formatting `value` with `format`.
    pub fn new<Value, Fmt>(value: Value, format: Fmt) -> crate::Result<Self>
    where
        Fmt: Format<Value = Value>,
    {
        let mut formatter = Formatter::new();
        format.fmt(&mut formatter, value)?;
        let mut segments = formatter.prev;
        segments.push(formatter.current);
        Ok(Self { segments })
    }

    fn indented_inner(self, spaces: u32, hanging: bool) -> Self {
        if spaces == 0 {
            return self;
        }

        let mut styles = Vec::with_capacity(self.segments.len());
        let mut buffers = Vec::with_capacity(self.segments.len());

        for segment in self.segments {
            styles.push(segment.style);
            buffers.push(segment.buf);
        }

        Self {
            segments: indent_vec(buffers, spaces, hanging)
                .into_iter()
                .zip(styles)
                .map(|(buf, style)| OutputSegment { buf, style })
                .collect(),
        }
    }

    /// Return a new [`FormattedOutput`] which has been indented by the given number of spaces.
    ///
    /// This is helpful when writing custom matchers that compose other matchers, so you can indent
    /// their output and include it in your matcher's output.
    pub fn indented(self, spaces: u32) -> Self {
        self.indented_inner(spaces, false)
    }

    /// Panic with this output as the error message.
    pub fn fail(&self) -> ! {
        check_disable_color();
        panic!("\n{}\n", self);
    }
}

impl fmt::Display for FormattedOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.segments {
            f.write_fmt(format_args!("{}", segment.style.apply(&segment.buf)))?;
        }

        Ok(())
    }
}
