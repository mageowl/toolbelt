use std::{
    io::{self, Stdout, Write},
    process::Command,
};

use termion::{event::Key, raw::RawTerminal};

use crate::{
    config::{Action, Config, Entry},
    output::Output,
    style::{Style, Styled},
    App,
};

use super::Instruction;

pub struct ListApp {
    pub(super) entries: Vec<Entry>,
    pub(super) placeholder: Styled,
    pub(super) icon: Styled,
    pub(super) selected_style: Style,

    pub(super) filter: String,
    pub(super) selected: usize,
    pub(super) list: Vec<usize>,

    pub(super) width: usize,
    pub(super) height: usize,
}

impl ListApp {
    fn update_list(&mut self) {
        self.list = self
            .entries
            .iter()
            .enumerate()
            .filter_map(|(i, Entry { name, keywords, .. })| {
                if name.contains(&self.filter)
                    || keywords.as_ref().is_some_and(|s| s.contains(&self.filter))
                {
                    Some(i)
                } else {
                    None
                }
            })
            .take(self.height - 2)
            .collect();
    }
}

impl App for ListApp {
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

        terminal.move_cursor(1, 2)?;
        terminal.print("\x1b[38;5;235m")?;
        terminal.print("─".repeat(self.width))?;
        terminal.print("\x1b[0m")?;

        for (i, entry) in self.list.iter().map(|i| &self.entries[*i]).enumerate() {
            terminal.move_cursor(1, i + 3)?;
            terminal.print(" ")?;

            if i == self.selected {
                terminal.print(&self.selected_style)?;
            }
            terminal.print(&entry.icon)?;
            terminal.print("  ")?;
            terminal.print(&entry.name)?;
            if i == self.selected {
                terminal.print(termion::style::Reset)?;
            }
        }

        terminal.move_cursor(prompt_offset + self.filter.len() + 4, 1)?;

        terminal.flush()
    }

    fn handle_input(&mut self, key: Key) -> Instruction {
        match key {
            Key::Char('\n') => {
                let item = &self.entries[self.list[self.selected]];
                match &item.action {
                    Action::Exec(name) => {
                        let output = Command::new("hyprctl")
                            .args(["dispatch", "exec"])
                            .arg(name)
                            .output();
                        if let Err(_err) = output {
                            //Instruction::SetApp(Box::new(MessageApp::new(format!("{err}"))))
                            Instruction::Quit
                        } else {
                            Instruction::Quit
                        }
                    }
                    Action::Command {
                        cmd,
                        args,
                        hold_output,
                    } => {
                        let mut child = Command::new(cmd)
                            .args(args)
                            .spawn()
                            .expect("failed to launch command");
                        if *hold_output {
                            //Instruction::SetApp(Box::new(OutputApp::new(child)))
                            Instruction::Quit
                        } else {
                            let _ = child.wait();
                            Instruction::Quit
                        }
                    }
                    Action::OpenMenu(name) => {
                        Instruction::SetApp(super::from_config(Config::get_menu(name.to_string())))
                    }
                }
            }

            Key::Up | Key::BackTab => {
                if self.selected > 0 {
                    self.selected -= 1
                } else {
                    self.selected = self.list.len() - 1
                }
                Instruction::None
            }
            Key::Down | Key::Char('\t') => {
                if self.selected < self.list.len() - 1 {
                    self.selected += 1;
                } else {
                    self.selected = 0;
                }
                Instruction::None
            }

            Key::Backspace => {
                self.selected = 0;
                self.filter.pop();
                self.update_list();
                Instruction::None
            }
            Key::Char(ch) => {
                self.selected = 0;
                self.filter.push(ch);
                self.update_list();
                Instruction::None
            }

            _ => Instruction::None,
        }
    }

    fn handle_resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }
}