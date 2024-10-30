use std::{fmt::Display, ops::Deref};

use serde::Deserialize;

#[derive(Default, Clone, Copy, PartialEq, Eq, Deserialize)]
#[allow(unused)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    #[default]
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    pub fn code(self) -> u8 {
        match self {
            Color::Default => 9,
            Color::Black => 0,
            Color::Red => 1,
            Color::Green => 2,
            Color::Yellow => 3,
            Color::Blue => 4,
            Color::Magenta => 5,
            Color::Cyan => 6,
            Color::White => 7,
        }
    }
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,

    pub dim: bool,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut code = Vec::new();

        if self.fg != Color::Default {
            code.push(self.fg.code() + 30);
        }
        if self.bg != Color::Default {
            code.push(self.bg.code() + 40);
        }
        if self.bold {
            code.push(1);
        }
        if self.dim {
            code.push(2);
        }
        if self.italic {
            code.push(3);
        }
        if self.underline {
            code.push(4);
        }

        let code: Vec<_> = code.iter().map(ToString::to_string).collect();

        write!(f, "\x1b[{code}m", code = code.join(";"))
    }
}

#[derive(Deserialize)]
pub struct Styled {
    pub text: String,

    #[serde(flatten)]
    pub style: Style,
}

impl From<String> for Styled {
    fn from(value: String) -> Self {
        Self {
            text: value,
            style: Style::default(),
        }
    }
}

impl Display for Styled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{style}{text}\x1b[0m",
            style = self.style,
            text = self.text
        )
    }
}

impl Deref for Styled {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.text
    }
}
