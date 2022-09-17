#![allow(dead_code)]

use std::borrow::Cow;

use super::{strings, Color, OutputStyle, TextColor, TextStyle};

pub const INFO_SYMBOL: &'static str = "\u{1f6c8}";
pub const MATCHED_MSG: &'static str = "MATCHED";
pub const FAILED_MSG: &'static str = "FAILED";
pub const INDENT_LEN: u32 = 4;

pub fn indent(levels: u32) -> Cow<'static, str> {
    strings::whitespace((INDENT_LEN * levels) as usize)
}

pub fn indent_len(levels: u32) -> u32 {
    INDENT_LEN * levels
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
