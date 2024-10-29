use std::{
    fmt::Display,
    io::{self, Stdout, Write},
};

use termion::raw::RawTerminal;

pub trait Output: Write {
    fn clear(&mut self) -> io::Result<()> {
        write!(
            self,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        )
    }

    fn print(&mut self, text: impl Display) -> io::Result<()> {
        write!(self, "{}", text)
    }

    fn move_cursor(&mut self, col: usize, ln: usize) -> io::Result<()> {
        write!(self, "{}", termion::cursor::Goto(col as u16, ln as u16))
    }
}

impl Output for RawTerminal<Stdout> {}
