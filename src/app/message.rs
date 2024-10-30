use std::io::{self, Stdout, Write};

use termion::raw::RawTerminal;

use crate::{output::Output, style::Styled};

use super::{App, Instruction};

pub struct MessageApp(pub Styled);

impl App for MessageApp {
    fn draw(&self, terminal: &mut RawTerminal<Stdout>) -> io::Result<()> {
        terminal.print(&self.0)?;
        terminal.print("\n\nPress any key to exit.")?;
        terminal.flush()
    }

    fn handle_input(&mut self, _key: termion::event::Key) -> Instruction {
        Instruction::None
    }
    fn handle_resize(&mut self, _width: usize, _height: usize) {}
}
