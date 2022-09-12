#![cfg(not(feature = "color"))]

use std::fmt;
use std::io::{self, Write};

use super::{Format, OutputStream};

#[derive(Debug)]
pub struct Formatter {
    buf: String,
}

impl Formatter {
    fn new() -> Self {
        Self { buf: String::new() }
    }

    pub fn write_fmt(&mut self, output: impl Into<FormattedOutput>) {
        self.buf.push_str(&output.into().buf);
    }
}

impl fmt::Write for Formatter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buf.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.buf.write_char(c)
    }

    fn write_fmt(self: &mut Self, args: fmt::Arguments<'_>) -> fmt::Result {
        self.buf.write_fmt(args)
    }
}

#[derive(Debug)]
pub struct FormattedOutput {
    buf: String,
}

impl FormattedOutput {
    pub fn new<Value, Fmt>(value: Value, format: Fmt) -> Self
    where
        Fmt: Format<Value = Value>,
    {
        let mut formatter = Formatter::new();
        format.fmt(&mut formatter, value);
        Self { buf: formatter.buf }
    }

    pub fn print(&self, stream: OutputStream) -> io::Result<()> {
        match stream {
            OutputStream::Stdout => {
                let mut stdout = io::stdout().lock();
                stdout.write_all(self.buf.as_bytes())?;
                stdout.flush()
            }
            OutputStream::Stderr => {
                let mut stderr = io::stderr().lock();
                stderr.write_all(self.buf.as_bytes())?;
                stderr.flush()
            }
        }
    }
}

impl fmt::Display for FormattedOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.buf)
    }
}
