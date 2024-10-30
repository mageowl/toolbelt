use std::{
    io::{self, stderr, stdout, Stdout, Write},
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
        terminal.clear()?;

        terminal.print(&self.icon)?;
        terminal.print("  ")?;
        if self.input.is_empty() {
            terminal.print(&self.placeholder)?;
        } else {
            terminal.print(&self.input)?;
        }

        terminal.move_cursor(self.input.len() + 4, 1)?;
        terminal.flush()
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
                        name,
                        args,
                        hold_output,
                        output_size,
                    } => {
                        let mut command = Command::new(name);
                        command.args(args.into_iter().map(|s| s.replace("{input}", &self.input)));
                        if *hold_output {
                            if let Some((w, h)) = output_size {
                                Command::new("hyprctl")
                                    .args([
                                        "--batch",
                                        &format!("dispatch resizeactive exact {w} {h}; dispatch centerwindow"),
                                    ])
                                    .output()
                                    .expect("failed to resize window");
                            }

                            command.stdout(stdout()).stderr(stderr());
                            Instruction::HoldOutput(command)
                        } else {
                            let _ = command.output();
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
