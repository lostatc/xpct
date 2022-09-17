#![allow(dead_code)]

use super::{Color, OutputStyle, TextColor, TextStyle};

pub const INFO_SYMBOL: &'static str = "\u{1f6c8}";

pub const fn indent() -> &'static str {
    "    "
}

pub const fn indent_len() -> u32 {
    indent().len() as u32
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
