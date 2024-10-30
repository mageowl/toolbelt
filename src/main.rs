use std::{
    env,
    io::{self, stdin, stdout, Write},
    process,
    sync::mpsc::{self, Sender},
    thread::{self, JoinHandle},
};

use output::Output;
use signal_hook::iterator::Signals;
use termion::{event::Key, input::TermRead, raw::IntoRawMode, terminal_size};

use app::{App, Instruction};
use config::Config;

mod app;
mod config;
mod output;
mod style;

enum Event {
    Key(Key),
    Resize(usize, usize),
}

fn start_resize_thread(sender: Sender<Event>) -> Option<JoinHandle<()>> {
    // SIGWINCH
    let Ok(mut signals) = Signals::new(&[28]) else {
        return None;
    };
    Some(thread::spawn(move || loop {
        if signals.pending().count() > 0 {
            let (width, height) = terminal_size().expect("failed to get terminal size.");
            if let Err(_) = sender.send(Event::Resize(width as usize, height as usize)) {
                break;
            }
        }
    }))
}

fn start_key_thread(sender: Sender<Event>) -> Option<JoinHandle<()>> {
    let mut events = stdin().events();
    Some(thread::spawn(move || loop {
        match events.next() {
            Some(Ok(termion::event::Event::Key(key))) => {
                if let Err(_) = sender.send(Event::Key(key)) {
                    break;
                }
            }
            Some(res) => {
                res.unwrap();
            }
            None => continue,
        };
    }))
}

fn main() -> io::Result<()> {
    let menu_name = env::args().skip(1).next().expect("no menu name provided.");
    let config = Config::get_menu(menu_name);

    let mut terminal = stdout().into_raw_mode()?;
    write!(terminal, "{}", termion::cursor::BlinkingBar)?;

    let mut app: Box<dyn App> = app::from_config(config);
    app.draw(&mut terminal)?;

    let (sender, receiver) = mpsc::channel::<Event>();
    start_resize_thread(sender.clone());
    start_key_thread(sender);

    let mut cmd = None;
    for event in &receiver {
        match event {
            Event::Key(Key::Esc) => break,
            Event::Key(key) => match app.handle_input(key) {
                Instruction::None => (),
                Instruction::Quit => break,
                Instruction::SetApp(new_app) => app = new_app,
                Instruction::HoldOutput(mut command) => {
                    terminal.clear()?;
                    terminal.move_cursor(1, 1)?;
                    terminal.flush()?;
                    drop(terminal);

                    cmd = Some(command.spawn().expect("failed to spawn"));
                    break;
                }
            },

            Event::Resize(w, h) => app.handle_resize(w, h),
        }

        app.draw(&mut terminal)?;
    }

    if let Some(mut child) = cmd {
        let res = child.wait();
        let mut temp = stdout().into_raw_mode()?;

        temp.print("\n")?;
        let code = res.expect("failed to get exit code.");
        if !code.success() {
            temp.print(format!("Process exited with code {code}.\n"))?;
        }

        temp.print("Press any key to exit.")?;
        temp.flush()?;
        while let Ok(Event::Resize(_, _)) | Err(_) = receiver.recv() {}
    }

    process::exit(0);
}
