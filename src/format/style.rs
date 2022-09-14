use crate::{OutputStyle, TerminalColor, TextColor, TextStyle};

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
            fg: Some(TerminalColor::Cyan),
            bg: None,
        },
    }
}

pub fn bad() -> OutputStyle {
    OutputStyle {
        style: TextStyle::BOLD,
        color: TextColor {
            fg: Some(TerminalColor::Red),
            bg: None,
        },
    }
}
