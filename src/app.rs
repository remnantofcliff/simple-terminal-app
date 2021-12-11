use crate::scene::Scene;
use std::io::{self, stdin, Stdout, Write};
use termion::{
    clear, cursor::{self, DetectCursorPos},
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};
/// Start the app by running the start(...)-method.
pub struct App {
    state: State,
}

impl App {
    pub fn start(scene: Box<dyn Scene>) -> Result<(), io::Error> {
        Self {
            state: State {
                running: true,

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

    pub stdout: RawTerminal<Stdout>,
}

impl State {
    pub fn position(&mut self) -> Result<(u16, u16), io::Error> {
        self.stdout.cursor_pos()
    }
    pub fn size(&self) -> Result<(u16, u16), io::Error> {
        termion::terminal_size()
    }
}