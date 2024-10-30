use std::{
    io::{self, Stdout},
    process::Command,
};

use list::ListApp;
use prompt::PromptApp;
use termion::{event::Key, raw::RawTerminal, terminal_size};

use crate::{
    config::{Config, MenuConfig},
    style::Styled,
};

pub mod list;
pub mod message;
pub mod prompt;

pub trait App {
    fn draw(&self, terminal: &mut RawTerminal<Stdout>) -> io::Result<()>;
    fn handle_input(&mut self, key: Key) -> Instruction;
    fn handle_resize(&mut self, width: usize, height: usize);
}

pub enum Instruction {
    None,
    Quit,
    SetApp(Box<dyn App>),
    HoldOutput(Command),
}

pub fn from_config(config: Config) -> Box<dyn App> {
    if let Some((w, h)) = config.window_size {
        Command::new("hyprctl")
            .args([
                "--batch",
                &format!("dispatch resizeactive exact {w} {h}; dispatch centerwindow"),
            ])
            .output()
            .expect("failed to resize window");
    }

    let size = terminal_size().expect("failed to measure size of the terminal");

    match config.menu {
        MenuConfig::List {
            entries,
            selected_style,
        } => Box::new(ListApp {
            list: (0..entries.len()).collect(),

            entries,
            placeholder: Styled::from(config.prompt),
            selected_style,
            icon: config.icon.into(),

            filter: String::new(),
            selected: 0,

            width: size.0 as usize,
            height: size.1 as usize,
        }),
        MenuConfig::Prompt { action } => Box::new(PromptApp {
            action,
            input: String::new(),
            placeholder: Styled::from(config.prompt),
            icon: config.icon.into(),
        }),
    }
}
