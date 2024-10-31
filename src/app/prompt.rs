use std::{
    io::{self, stderr, stdout, Stdout, Write},
    process::Command,
    thread,
    time::Duration,
};

use termion::{event::Key, raw::RawTerminal};

use crate::{
    config::{Action, Config},
    output::Output,
    style::Styled,
};

use super::{message::MessageApp, App, Instruction};

pub struct PromptApp {
    pub(super) input: String,
    pub(super) cursor_index: usize,

    pub(super) placeholder: Styled,
    pub(super) icon: Styled,
    pub(super) action: Action,
    pub(super) history: Option<Vec<(usize, String)>>,

    pub(super) width: usize,
    pub(super) height: usize,
}

impl App for PromptApp {
    fn draw(&self, terminal: &mut RawTerminal<Stdout>) -> io::Result<()> {
        terminal.clear()?;

        let cursor_pos = if self.history.is_some() {
            let msg_width = self.placeholder.len() + 2;
            let prompt_offset = self.width / 2 - msg_width / 2;
            terminal.print(" ".repeat(prompt_offset))?;

            self.cursor_index + prompt_offset + 3
        } else {
            self.cursor_index + 3
        };

        terminal.print(&self.icon)?;
        terminal.print(" ")?;
        if self.input.is_empty() {
            terminal.print(&self.placeholder)?;
        } else {
            terminal.print(&self.input)?;
        }

        if let Some(vec) = &self.history {
            terminal.move_cursor(1, 2)?;
            terminal.print("\x1b[38;5;235m")?;
            terminal.print("─".repeat(self.width))?;
            terminal.print("\x1b[0m")?;

            let mut ln = 0;
            for (size, entry) in vec.iter().rev().take(self.height - 2) {
                terminal.move_cursor(1, ln + 3)?;
                terminal.print(" ")?;
                terminal.print(&entry)?;

                ln += size;
            }
        }

        terminal.move_cursor(cursor_pos, 1)?;
        terminal.flush()
    }

    fn handle_input(&mut self, key: Key) -> Instruction {
        match key {
            Key::Char('\n') => match &self.action {
                Action::Exec(name) => {
                    let output = Command::new("hyprctl")
                        .args(["dispatch", "exec"])
                        .arg(name.replace("{input}", &self.input))
                        .output();
                    if let Err(err) = output {
                        Instruction::SetApp(Box::new(MessageApp(format!("{err}").into())))
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
                            thread::sleep(Duration::from_millis(100));
                        }

                        command.stdout(stdout()).stderr(stderr());
                        Instruction::HoldOutput(command)
                    } else {
                        let out = command.output();
                        if let Some(vec) = &mut self.history {
                            let str = match out {
                                Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
                                Err(err) => err.to_string(),
                            }
                            .replace("\n", "\n\r ");
                            let size = str.matches("\n").count();

                            vec.push((size, str));

                            self.input = String::new();
                            self.cursor_index = 0;
                            Instruction::None
                        } else {
                            Instruction::Quit
                        }
                    }
                }
                Action::OpenMenu(name) => Instruction::SetApp(super::from_config(
                    Config::get_menu(name.replace("{input}", &self.input)),
                )),
            },

            Key::Backspace => {
                if self.cursor_index > 0 {
                    self.input.remove(self.cursor_index - 1);
                    self.cursor_index -= 1;
                }
                Instruction::None
            }
            Key::Char(ch) => {
                self.input.insert(self.cursor_index, ch);
                self.cursor_index += 1;
                Instruction::None
            }

            Key::Left => {
                if self.cursor_index > 0 {
                    self.cursor_index -= 1;
                }
                Instruction::None
            }
            Key::Right => {
                if self.cursor_index < self.input.len() {
                    self.cursor_index += 1;
                }
                Instruction::None
            }

            _ => Instruction::None,
        }
    }

    fn handle_resize(&mut self, _: usize, _: usize) {}
}
