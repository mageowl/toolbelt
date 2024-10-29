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
    fn code(self) -> u8 {
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

#[derive(Deserialize)]
pub struct Styled {
    text: String,

    #[serde(default)]
    fg: Color,
    #[serde(default)]
    bg: Color,

    #[serde(default)]
    dim: bool,
    #[serde(default)]
    bold: bool,
    #[serde(default)]
    italic: bool,
    #[serde(default)]
    underline: bool,
}

impl From<String> for Styled {
    fn from(value: String) -> Self {
        Self {
            text: value,

            fg: Color::Default,
            bg: Color::Default,
            dim: false,
            bold: false,
            italic: false,
            underline: false,
        }
    }
}

impl Display for Styled {
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

        write!(
            f,
            "\x1b[{code}m{inner}\x1b[0m",
            inner = self.text,
            code = code.join(";")
        )
    }
}

pub trait Style: Display {
    fn fg(self, color: Color) -> Styled;
    fn bg(self, color: Color) -> Styled;

    fn dim(self) -> Styled;
    fn bold(self) -> Styled;
    fn italic(self) -> Styled;
    fn underline(self) -> Styled;
}

impl Style for Styled {
    fn fg(mut self, color: Color) -> Styled {
        self.fg = color;
        self
    }
    fn bg(mut self, color: Color) -> Styled {
        self.bg = color;
        self
    }

    fn dim(mut self) -> Styled {
        self.dim = true;
        self
    }
    fn bold(mut self) -> Styled {
        self.bold = true;
        self
    }
    fn italic(mut self) -> Styled {
        self.italic = true;
        self
    }
    fn underline(mut self) -> Styled {
        self.underline = true;
        self
    }
}

impl Deref for Styled {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

impl<T> Style for T
where
    T: Into<String> + Display,
{
    fn fg(self, color: Color) -> Styled {
        Styled::from(self.into()).fg(color)
    }
    fn bg(self, color: Color) -> Styled {
        Styled::from(self.into()).bg(color)
    }

    fn dim(self) -> Styled {
        Styled::from(self.into()).dim()
    }
    fn bold(self) -> Styled {
        Styled::from(self.into()).bold()
    }
    fn italic(self) -> Styled {
        Styled::from(self.into()).italic()
    }
    fn underline(self) -> Styled {
        Styled::from(self.into()).underline()
    }
}
