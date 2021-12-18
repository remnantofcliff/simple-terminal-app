use crate::{app::State, event::Key};

pub trait Scene {
    /// Initializes the scene. This method is run once before `process_input(...)` starts running.
    fn init(&mut self, state: &mut State);

    /// Gets called on every key-press.
    fn process_input(&mut self, state: &mut State, key: Key);

    /// Gets called independent of user input.
    fn update(&mut self, state: &mut State);
}
