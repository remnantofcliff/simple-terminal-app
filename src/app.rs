use crate::scene::Scene;
use std::io::{self, stdin, Stdout, Write};
use termion::{
    clear,
    cursor::{self, DetectCursorPos},
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

/// Represents the global state of the app. 
pub struct State {
    running: bool,
    next_scene: Option<Box<dyn Scene>>,
    stdout: RawTerminal<Stdout>,
}

impl State {
    /// Stops this scene and starts a new scene.
    pub fn change_scene(&mut self, scene: Box<dyn Scene>) {
        self.next_scene = Some(scene);

        self.stop();
    }
    /// Returns the current position of the cursor.
    pub fn position(&mut self) -> Result<(u16, u16), io::Error> {
        self.stdout.cursor_pos()
    }
    /// Returns the current size of the terminal.
    pub fn size(&self) -> Result<(u16, u16), io::Error> {
        termion::terminal_size()
    }
    /// Stops the scene from running.
    pub fn stop(&mut self) {
        self.running = false;
    }
}

impl Write for State {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}
