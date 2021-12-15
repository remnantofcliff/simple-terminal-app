use crate::{commands::cursor, scene::Scene, Point};
use std::{
    fmt::Display,
    io::{self, stdin, Stdout, Write},
};
use termion::{
    clear,
    cursor::DetectCursorPos,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};
/// Start the app by running the start(...)-method.
pub struct App {
    state: State,
}

impl App {
    /// Creates a new app and runs the scene.
    pub fn start(scene: Box<dyn Scene>) -> Result<(), io::Error> {
        let mut app = Self {
            state: State {
                running: true,

                stdout: io::stdout().into_raw_mode()?,

                next_scene: None,
            },
        };

        app.state
            .command()
            .append(clear::All)
            .append(cursor::Goto(Point::new(0, 0)))
            .execute()?;

        app.state.stdout.flush()?;

        app.run_scene(scene)?;

        Ok(())
    }

    fn run_scene(&mut self, mut scene: Box<dyn Scene>) -> Result<(), io::Error> {
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
    /// Returns a CommandBuilder that can be used to build and execute commands.
    pub fn command(&mut self) -> CommandBuilder {
        CommandBuilder {
            buffer: Vec::new(),

            parent: self,
        }
    }
    /// Returns the current position of the cursor. Starts from (0, 0).
    pub fn position(&mut self) -> Result<Point, io::Error> {
        let temp = self.cursor_pos()?;

        Ok(Point {
            x: temp.0 - 1,

            y: temp.1 - 1,
        })
    }
    /// Returns the the lower-right corner point of the terminal.
    pub fn size(&self) -> Result<Point, io::Error> {
        let temp = termion::terminal_size()?;

        Ok(Point {
            x: temp.0 - 1,

            y: temp.1 - 1,
        })
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
/// Used for building commands. Use the append-method to string commands together and execute when
/// ready. Executing doesn't flush stdout however, so you need to do state.flush() afterwards, when
/// you want.
pub struct CommandBuilder<'a> {
    buffer: Vec<String>,

    parent: &'a mut State,
}

impl<'a> CommandBuilder<'a> {
    /// Append a new command. Commands must implement Display. Could also just be a string or a
    /// number, for example.
    pub fn append<D: Display>(&mut self, command: D) -> &mut Self {
        self.buffer.push(command.to_string());

        self
    }
    /// Execute the given command.
    pub fn execute(&mut self) -> Result<usize, io::Error> {
        self.parent.write(
            &self
                .buffer
                .iter()
                .flat_map(|string| string.as_bytes().to_vec())
                .collect::<Vec<u8>>(),
        )
    }
}
