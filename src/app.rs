use crate::{
    commands::{clear, cursor},
    scene::Scene,
    Point,
};

use std::{
    fmt::Display,
    io::{self, Stdout, Write},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use termion::{
    cursor::DetectCursorPos,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

/// Creates a new app and runs the scene from the first argument. `update_time_millis` can be
/// `None` if one doesn't want to use the update method of the scene and only update the scene on
/// input. Otherwise `update_time_millis` should be the time in milliseconds between calling the
/// update method.
pub fn start(mut scene: Box<dyn Scene + Send>, update_time_millis: Option<u64>) {
    let mut stdout = io::stdout()
        .into_raw_mode()
        .expect("Could not switch terminal to raw mode.");

    let mut state = State {
        cursor_position: cursor_pos(&mut stdout),
        next_scene: None,
        running: true,
        stdout,
    };

    state
        .command()
        .push(clear::All)
        .push(cursor::Goto(Point::new(0, 0)))
        .execute();

    state.flush();

    scene.init(&mut state);

    if let Some(update_time) = update_time_millis {
        let mut scene_mutex_input = Arc::new(Mutex::new(scene));

        let mut state_mutex_input = Arc::new(Mutex::new(state));

        let scene_mutex_update = scene_mutex_input.clone();

        let state_mutex_update = state_mutex_input.clone();

        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(update_time));

            scene_mutex_update
                .lock()
                .unwrap()
                .update(&mut state_mutex_update.lock().unwrap());
        });

        start_with_update(&mut scene_mutex_input, &mut state_mutex_input);
    } else {
        start_input_only(scene, &mut state);
    }
}

fn start_input_only(mut scene: Box<dyn Scene + Send>, state: &mut State) {
    for event in io::stdin().keys().map(|r| r.unwrap()) {
        scene.process_input(state, event);

        state.cursor_position = cursor_pos(&mut state.stdout);

        if !state.running {
            if let Some(next_scene) = state.next_scene.take() {
                state.running = true;

                scene = next_scene;

                scene.init(state);
            } else {
                break;
            }
        }
    }
}

fn start_with_update(
    scene_mutex: &mut Arc<Mutex<Box<dyn Scene + Send>>>,
    state_mutex: &mut Arc<Mutex<State>>,
) {
    for event in io::stdin().keys().map(|r| r.unwrap()) {
        let mut scene_lock = scene_mutex.lock().unwrap();

        let mut state_lock = state_mutex.lock().unwrap();

        scene_lock.process_input(&mut state_lock, event);

        state_lock.cursor_position = cursor_pos(&mut state_lock.stdout);

        if !state_lock.running {
            if let Some(next_scene) = state_lock.next_scene.take() {
                state_lock.running = true;

                *scene_lock = next_scene;

                scene_lock.init(&mut state_lock);
            } else {
                break;
            }
        }
    }
}

fn cursor_pos(stdout: &mut RawTerminal<Stdout>) -> Point {
    let temp = stdout.cursor_pos().expect("Could not get cursor position");
    Point {
        x: temp.0 - 1,
        y: temp.1 - 1,
    }
}

/// Represents the global state of the app.
pub struct State {
    pub cursor_position: Point,
    next_scene: Option<Box<dyn Scene + Send>>,
    running: bool,
    stdout: RawTerminal<Stdout>,
}

impl State {
    /// Stops this scene and starts a new scene.
    pub fn change_scene(&mut self, scene: Box<dyn Scene + Send>) {
        self.next_scene = Some(scene);

        self.stop();
    }

    /// Returns a `CommandBuilder` that can be used to build and execute commands.
    pub fn command(&mut self) -> CommandBuilder {
        CommandBuilder {
            buffer: Vec::new(),
            parent: self,
        }
    }

    /// Flush the buffer. Has to be called after executing a command for any change to take place
    /// in the terminal.
    pub fn flush(&mut self) {
        self.stdout.flush().expect("Could not flush stdout");
    }

    /// Returns the the lower-right corner point of the terminal.
    pub fn size(&self) -> Point {
        let (x, y) = termion::terminal_size().expect("Could not get terminal size");

        Point { x: x - 1, y: y - 1 }
    }

    /// Stops the scene from running.
    pub fn stop(&mut self) {
        self.running = false;
    }
}

/// Used for building commands. Use `push(...)` to string commands together and execute when
/// ready. Executing doesn't flush stdout however, so you need to do `state.flush()` afterwards, when
/// you want.
pub struct CommandBuilder<'a> {
    buffer: Vec<u8>,
    parent: &'a mut State,
}

impl<'a> CommandBuilder<'a> {
    /// Push a command. Commands must implement Display. Could also just be a string or a number,
    /// for example.
    pub fn push<D: Display>(&mut self, command: D) -> &mut Self {
        self.buffer.extend(command.to_string().as_bytes());

        self
    }

    /// Execute the given command.
    pub fn execute(&mut self) {
        self.parent
            .stdout
            .write_all(self.buffer.as_slice())
            .expect("Could not write buffer to stdout");
    }
}
