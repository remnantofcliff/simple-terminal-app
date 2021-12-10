use crate::scene::Scene;
use std::{
    io::{self, Stdout, Write},
    sync::mpsc::{self, Receiver},
    thread,
};
use termion::{
    clear, cursor,
    event::Event,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};
/// Represents a position on the terminal. Upper-left corner equals (1, 1)
#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: u16,

    pub y: u16,
}

impl From<(u16, u16)> for Position {
    fn from(tuple: (u16, u16)) -> Position {
        Position {
            x: tuple.0,

            y: tuple.1,
        }
    }
}
/// Start the app by running the start(...)-method.
pub struct App {
    state: State,
    input_reveiver: Receiver<Event>,
}

impl App {
    pub fn start(scene: Box<dyn Scene>) -> Result<(), io::Error> {
        fn event_thread_spawn(transmitter: mpsc::Sender<Event>) {
            let stdin = io::stdin();

            thread::spawn(move || {
                for event in stdin.lock().events().map(|r| r.unwrap()) {
                    if let Event::Key(_) = event {
                        transmitter.send(event).unwrap();
                    }
                }
            });
        }

        let (transmitter, receiver) = mpsc::channel();

        event_thread_spawn(transmitter);

        Self {
            state: State {
                cursor_position: Position { x: 1, y: 1 },

                running: true,

                size: termion::terminal_size().unwrap().into(),

                stdout: io::stdout().into_raw_mode()?,

                next_scene: None,
            },

            input_reveiver: receiver,
        }
        .run_scene(scene)?;

        Ok(())
    }
    fn run_scene(&mut self, scene: Box<dyn Scene>) -> Result<(), io::Error> {
        write!(self.state.stdout, "{}{}", clear::All, cursor::Goto(1, 1))?;

        self.state.stdout.flush()?;

        while self.state.running {
            let event = self
                .input_reveiver
                .recv()
                .expect("Event thread has hung up");

            scene.process_input(event, &mut self.state)?;
        }

        if let Some(scene) = self.state.next_scene.take() {
            self.state.running = true;

            scene.init(&mut self.state)?;

            self.run_scene(scene)?;
        }
        Ok(())
    }
}

/// Represents the global state of the program.
pub struct State {
    pub cursor_position: Position,

    pub running: bool,

    pub next_scene: Option<Box<dyn Scene>>,

    pub size: Position,

    pub stdout: RawTerminal<Stdout>,
}
