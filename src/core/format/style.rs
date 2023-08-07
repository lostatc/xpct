#![allow(dead_code)]

use std::borrow::Cow;

use super::{strings, Color, OutputStyle, TextColor, TextStyle};

pub const WHY_SYMBOL: &str = "[why]";
pub const OK_MSG: &str = "OK";
pub const FAILED_MSG: &str = "FAILED";
pub const INDENT_LEN: u32 = 4;

pub const ALL_OK_HEADER: &str = "Expected all of these to succeed:";
pub const AT_LESAT_ONE_OK_HEADER: &str = "Expected at least one of these to succeed:";

pub const ALL_FIELDS_OK_HEADER: &str = "Expected all of these fields to succeed:";
pub const AT_LESAT_ONE_FIELD_OK_HEADER: &str = "Expected at least one of these fields to succeed:";

pub const AT_LESAT_ONE_NOT_OK_MSG: &str = "Expected at least one of these to fail.";

pub fn indent_len(levels: u32) -> u32 {
    INDENT_LEN * levels
}

pub fn indent(levels: u32) -> Cow<'static, str> {
    strings::whitespace(indent_len(levels) as usize)
}

pub fn important() -> OutputStyle {
    OutputStyle {
        style: TextStyle::BOLD,
        color: Default::default(),
    }
}

pub fn info() -> OutputStyle {
    OutputStyle {
        style: Default::default(),
        color: TextColor {
            fg: Some(Color::Cyan),
            bg: None,
        },
    }
}

pub fn bad() -> OutputStyle {
    OutputStyle {
        style: TextStyle::BOLD,
        color: TextColor {
            fg: Some(Color::Red),
            bg: None,
        },
    }
}

pub fn failure() -> OutputStyle {
    OutputStyle {
        style: TextStyle::BOLD | TextStyle::UNDERLINE,
        color: TextColor {
            fg: Some(Color::Red),
            bg: None,
        },
    }
}

pub fn good() -> OutputStyle {
    OutputStyle {
        style: TextStyle::BOLD,
        color: TextColor {
            fg: Some(Color::Green),
            bg: None,
        },
    }
}

pub fn success() -> OutputStyle {
    OutputStyle {
        style: TextStyle::BOLD | TextStyle::UNDERLINE,
        color: TextColor {
            fg: Some(Color::Green),
            bg: None,
        },
    }
}

pub fn index() -> OutputStyle {
    OutputStyle {
        style: Default::default(),
        color: TextColor {
            fg: Some(Color::Yellow),
            bg: None,
        },
    }
}
