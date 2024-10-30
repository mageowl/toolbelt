use std::{
    io::{self, Stdout},
    process::Command,
};

use termion::{event::Key, raw::RawTerminal};

use crate::{
    config::{Action, Config},
    output::Output,
    style::Styled,
};

use super::{App, Instruction};

pub struct PromptApp {
    pub(super) input: String,

    pub(super) placeholder: Styled,
    pub(super) icon: Styled,
    pub(super) action: Action,
}

impl App for PromptApp {
    fn draw(&self, terminal: &mut RawTerminal<Stdout>) -> io::Result<()> {
        terminal.print(&self.icon)?;
        terminal.print("  ")?;
        if self.input.is_empty() {
            terminal.print(&self.placeholder)?;
        } else {
            terminal.print(&self.input)?;
        }

        terminal.move_cursor(self.input.len() + 4, 1)?;

        Ok(())
    }

    fn handle_input(&mut self, key: Key) -> Instruction {
        match key {
            Key::Char('\n') => {
                match &self.action {
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

            Key::Backspace => {
                self.input.pop();
                Instruction::None
            }
            Key::Char(ch) => {
                self.input.push(ch);
                Instruction::None
            }

            _ => Instruction::None,
        }
    }

    fn handle_resize(&mut self, _: usize, _: usize) {}
}