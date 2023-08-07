#![cfg(feature = "color")]

use std::fmt;
use std::io::{self, IsTerminal};

use super::strings::{indent_segments, OutputSegment};
use super::{Format, OutputStyle};

// Disable colors and text styles if stderr is not a tty.
fn check_disable_color() {
    if !io::stderr().is_terminal() {
        colored::control::set_override(false)
    }
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

    fn into_segments(self) -> Vec<OutputSegment> {
        let mut segments = self.prev;
        segments.push(self.current);
        segments
    }

    fn push_segments(&mut self, segments: Vec<OutputSegment>) {
        let new_current = OutputSegment {
            buf: String::new(),
            style: self.current.style.clone(),
        };

        self.prev
            .push(std::mem::replace(&mut self.current, new_current));
        self.prev.extend(segments);
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
        self.push_segments(formatted.segments)
    }

    fn indented_inner(
        &mut self,
        prefix: impl AsRef<str>,
        hanging: bool,
        func: impl FnOnce(&mut Formatter) -> crate::Result<()>,
    ) -> crate::Result<()> {
        let mut formatter = Self::new();
        formatter.current.style = self.current.style.clone();

        func(&mut formatter)?;

        let segments = formatter.into_segments();
        let output = FormattedOutput { segments };

        let indented = output.indented_inner(prefix.as_ref(), hanging);
        self.push_segments(indented.segments);

        Ok(())
    }

    /// Write some indented text to the output.
    ///
    /// Anything written to the [`Formatter`] passed to `func` is indented by `prefix`.
    ///
    /// To indent with whitespace, you can use [`whitespace`].
    ///
    /// The current [`style`] is inherited by the [`Formatter`] passed to `func`.
    ///
    /// [`whitespace`]: crate::core::whitespace
    /// [`style`]: crate::core::Formatter::style
    pub fn indented(
        &mut self,
        prefix: impl AsRef<str>,
        func: impl FnOnce(&mut Formatter) -> crate::Result<()>,
    ) -> crate::Result<()> {
        self.indented_inner(prefix, false, func)
    }

    /// Write some indented text to the output with a hanging indent.
    ///
    /// This is the same as [`indented`], but generates a hanging indent.
    ///
    /// [`indented`]: crate::core::Formatter::indented
    pub fn indented_hanging(
        &mut self,
        prefix: impl AsRef<str>,
        func: impl FnOnce(&mut Formatter) -> crate::Result<()>,
    ) -> crate::Result<()> {
        self.indented_inner(prefix, true, func)
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

/// A value that has been formatted with [`Format`].
///
/// Formatting a value with [`Format`] returns this opaque type rather than a string, since we need
/// to encapsulate the colors and text styles information in a cross-platform way. While ANSI escape
/// codes can be included in a string, other platforms (such as Windows) have their own mechanisms
/// for including colors and text styles in stdout/stderr.
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
        Ok(Self {
            segments: formatter.into_segments(),
        })
    }

    fn indented_inner(self, prefix: impl AsRef<str>, hanging: bool) -> Self {
        if prefix.as_ref().is_empty() {
            return self;
        }

        Self {
            segments: indent_segments(self.segments, prefix.as_ref(), hanging),
        }
    }

    /// Return a new [`FormattedOutput`] which has been indented by `prefix`.
    ///
    /// To indent with whitespace, you can use [`whitespace`].
    ///
    /// Also see [`FormattedFailure::into_indented`].
    ///
    /// [`whitespace`]: crate::core::whitespace
    /// [`FormattedFailure::into_indented`]: crate::core::FormattedFailure::into_indented
    pub fn indented(self, prefix: impl AsRef<str>) -> Self {
        self.indented_inner(prefix, false)
    }

    /// Return a new [`FormattedOutput`] which has been indented by `prefix` with a hanging indent.
    ///
    /// This is the same as [`indented`], but generates a hanging indent.
    ///
    /// [`indented`]: crate::core::FormattedOutput::indented
    pub fn indented_hanging(self, prefix: impl AsRef<str>) -> Self {
        self.indented_inner(prefix, true)
    }

    /// Panic with this output as the error message.
    ///
    /// This does not print colors or text styles when the [`NO_COLOR`](https://no-color.org/)
    /// environment variable is set or when stderr is not a tty.
    pub fn fail(&self) -> ! {
        // See CONTRIBUTING.md for an explanation of why we do this.
        if cfg!(debug_screenshot) {
            println!("{}", self);
            std::process::exit(0);
        } else {
            check_disable_color();
            panic!("\n{}\n", self);
        }
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
