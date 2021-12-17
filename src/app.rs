use crate::{commands::cursor, scene::Scene, Point};
use std::{
    fmt::Display,
    io::{self, stdin, Stdout, Write},
    sync::{self, Arc, Mutex},
    thread,
    time::Duration,
};
use termion::{
    clear,
    cursor::DetectCursorPos,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

enum AppEvent {
    Flush,
    RequestPos,
    Shutdown,
    Write(Vec<u8>),
}

enum StateEvent {
    Point(u16, u16),
}
/// Creates a new app and runs the scene from the first argument.
pub fn start(mut scene: Box<dyn Scene + Send>, input_and_update_time_millis: Option<(u64, u64)>) {
    let stdout = &mut io::stdout()
        .into_raw_mode()
        .expect("Could not switch terminal to raw mode.");

    let (state_transmitter, app_receiver) = sync::mpsc::channel();

    let (app_transmitter, state_receiver) = sync::mpsc::channel();

    let mut state = State {
        cursor_position: Point::new(0, 0),
        next_scene: None,
        running: true,
        rx: state_receiver,
        tx: state_transmitter,
    };

    state
        .command()
        .append(clear::All)
        .append(cursor::Goto(Point::new(0, 0)))
        .execute();

    stdout.flush().expect("Could not flush stdout");

    scene.init(&mut state);

    if let Some(input_update_time) = input_and_update_time_millis {
        assert!(input_update_time.0 > 0);
        let scene_mutex_input = Arc::new(Mutex::new(scene));

        let state_mutex_input = Arc::new(Mutex::new(state));

        let scene_mutex_update = scene_mutex_input.clone();

        let state_mutex_update = state_mutex_input.clone();

        thread::Builder::new()
            .name("Input thread".to_string())
            .stack_size(1)
            .spawn(move || loop {
                thread::sleep(Duration::from_millis(input_update_time.0));

                for event in stdin().keys().map(|r| r.unwrap()) {
                    let mut scene_lock = scene_mutex_input.lock().unwrap();

                    let mut state_lock = state_mutex_input.lock().unwrap();

                    scene_lock.process_input(&mut state_lock, event);

                    state_lock.send_command(AppEvent::RequestPos);

                    state_lock.cursor_position = match state_lock.rx.recv().unwrap() {
                        StateEvent::Point(x, y) => Point { x: x - 1, y: y - 1 },
                    };

                    if !state_lock.running {
                        if let Some(next_scene) = state_lock.next_scene.take() {
                            state_lock.running = true;

                            *scene_lock = next_scene;

                            scene_lock.init(&mut state_lock);
                        } else {
                            state_lock.send_command(AppEvent::Shutdown);
                        }
                    }
                }
            })
            .expect("Failed to create input thread");

        thread::Builder::new()
            .name("Update thread".to_string())
            .stack_size(1)
            .spawn(move || loop {
                thread::sleep(Duration::from_millis(input_update_time.1));

                scene_mutex_update
                    .lock()
                    .unwrap()
                    .update(&mut state_mutex_update.lock().unwrap());
            })
            .expect("Could not create update thread.");

        main_thread_loop(app_receiver, stdout, app_transmitter);
    } else {
        thread::Builder::new()
            .name("Input thread".to_string())
            .stack_size(1)
            .spawn(move || {
                for event in stdin().keys().map(|r| r.unwrap()) {
                    scene.process_input(&mut state, event);

                    if !state.running {
                        if let Some(next_scene) = state.next_scene.take() {
                            state.running = true;

                            scene = next_scene;

                            scene.init(&mut state);
                        } else {
                            break;
                        }
                    }
                }
            })
            .unwrap();

        main_thread_loop(app_receiver, stdout, app_transmitter);
    }
}

fn main_thread_loop(
    receiver: sync::mpsc::Receiver<AppEvent>,
    stdout: &mut RawTerminal<Stdout>,
    transmitter: sync::mpsc::Sender<StateEvent>,
) {
    for event in receiver.iter() {
        match event {
            AppEvent::Flush => stdout.flush().expect("Could not flush stdout"),
            AppEvent::RequestPos => {
                let pos = stdout.cursor_pos().expect("Could not get cursor position.");
                transmitter.send(StateEvent::Point(pos.0, pos.1)).unwrap();
            }
            AppEvent::Shutdown => break,
            AppEvent::Write(buffer) => stdout
                .write_all(buffer.as_slice())
                .expect("Could not write buffer to stdout"),
        }
    }
}
/// Represents the global state of the app.
pub struct State {
    cursor_position: Point,
    next_scene: Option<Box<dyn Scene + Send>>,
    running: bool,
    rx: sync::mpsc::Receiver<StateEvent>,
    tx: sync::mpsc::Sender<AppEvent>,
}

impl State {
    /// Stops this scene and starts a new scene.
    pub fn change_scene(&mut self, scene: Box<dyn Scene + Send>) {
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
    /// Flush the buffer. Has to be called after executing a command for any change to take place
    /// in the terminal.
    pub fn flush(&mut self) {
        self.send_command(AppEvent::Flush);
    }
    /// Returns the current position of the cursor. Starts from (0, 0).
    pub fn position(&mut self) -> Point {
        self.cursor_position
    }
    /// Returns the the lower-right corner point of the terminal.
    pub fn size(&self) -> Point {
        let (x, y) = termion::terminal_size().expect("Could not get terminal size");

        Point { x, y }
    }
    /// Stops the scene from running.
    pub fn stop(&mut self) {
        self.running = false;
    }

    fn send_command(&mut self, command: AppEvent) {
        self.tx.send(command).unwrap();
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
    pub fn execute(&mut self) {
        self.parent.send_command(AppEvent::Write(
            self.buffer
                .iter()
                .flat_map(|string| string.as_bytes().to_vec())
                .collect::<Vec<u8>>(),
        ));
    }
}
