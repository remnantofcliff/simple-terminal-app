use crate::scene::Scene;
use std::io::{self, stdin, Stdout, Write};
use termion::{
    clear, cursor,
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
}

impl App {
    pub fn start(scene: Box<dyn Scene>) -> Result<(), io::Error> {
        Self {
            state: State {
                running: true,

                size: termion::terminal_size().unwrap().into(),

                stdout: io::stdout().into_raw_mode()?,

                next_scene: None,
            },
        }
        .run_scene(scene)?;

        Ok(())
    }
    fn run_scene(&mut self, scene: Box<dyn Scene>) -> Result<(), io::Error> {
        write!(self.state.stdout, "{}{}", clear::All, cursor::Goto(1, 1))?;

        self.state.stdout.flush()?;

        scene.init(&mut self.state)?;

        for event in stdin().keys().map(|r| r.unwrap()) {
            scene.process_input(event, &mut self.state)?;

            if !self.state.running {
                break;
            }
        }

        if let Some(scene) = self.state.next_scene.take() {
            self.state.running = true;

            self.run_scene(scene)?;
        }

        Ok(())
    }
}

/// Represents the global state of the program.
pub struct State {
    pub running: bool,

    pub next_scene: Option<Box<dyn Scene>>,

    pub size: Position,

    pub stdout: RawTerminal<Stdout>,
}
