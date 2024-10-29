use std::{
    env,
    io::{self, stdin, stdout, Stdout, Write},
    usize,
};

use config::{Config, Entry};
use output::Output;
use style::{Style, Styled};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    terminal_size,
};

mod config;
mod output;
mod style;

struct App {
    entries: Option<Vec<Entry>>,
    placeholder: Styled,
    icon: Styled,

    filter: String,

    width: usize,
    height: usize,
}

impl App {
    fn new(config: Config) -> Self {
        let size = terminal_size().expect("failed to measure size of terminal");

        Self {
            placeholder: config.prompt.dim(),
            icon: config.icon,
            entries: config.entries,

            filter: String::new(),

            width: size.0 as usize,
            height: size.1 as usize,
        }
    }

    fn draw(&self, terminal: &mut RawTerminal<Stdout>) -> io::Result<()> {
        let msg_width = self.placeholder.len() + 2;
        let prompt_offset = self.width / 2 - msg_width / 2;

        terminal.clear()?;
        terminal.print(" ".repeat(prompt_offset))?;
        terminal.print(&self.icon)?;
        terminal.print("  ")?;

        if self.filter.is_empty() {
            terminal.print(&self.placeholder)?;
        } else {
            terminal.print(&self.filter)?;
        }

        if let Some(vec) = &self.entries {
            terminal.move_cursor(1, 2)?;
            terminal.print("\x1b[38;5;235m")?;
            terminal.print("─".repeat(self.width))?;
            terminal.print("\x1b[0m")?;

            for (i, entry) in vec
                .iter()
                .filter(|Entry { name, keywords, .. }| {
                    name.contains(&self.filter)
                        || keywords.as_ref().is_some_and(|s| s.contains(&self.filter))
                })
                .take(self.height - 2)
                .enumerate()
            {
                terminal.move_cursor(1, i + 3)?;
                terminal.print(" ")?;
                terminal.print(&entry.icon)?;
                terminal.print("  ")?;
                terminal.print(&entry.name)?;
            }
        }

        terminal.move_cursor(prompt_offset + self.filter.len() + 4, 1)?;

        terminal.flush()
    }

    fn handle_input(&mut self, key: Key) {
        match key {
            Key::Char('\n') => (),

            Key::Backspace => {
                self.filter.pop();
            }
            Key::Char(ch) => self.filter.push(ch),
            _ => (),
        }
    }
}

fn main() -> io::Result<()> {
    let menu_name = env::args().skip(1).next().expect("no menu name provided.");
    let config = Config::get_menu(menu_name);

    let mut terminal = stdout().into_raw_mode()?;
    write!(terminal, "{}", termion::cursor::BlinkingBar)?;

    let mut app = App::new(config);
    app.draw(&mut terminal)?;

    for event in stdin().keys() {
        match event.expect("failed to parse input") {
            Key::Esc => break,
            key => app.handle_input(key),
        }

        app.draw(&mut terminal)?;
    }

    Ok(())
}
