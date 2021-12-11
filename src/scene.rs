use termion::event::Key;

use crate::app::State;

pub trait Scene {
    /// Initializes the scene. This method is run once before process_input() starts running.
    fn init(&self, state: &mut State) -> Result<(), std::io::Error>;
    /// Gets called on every key-press.
    fn process_input(&self, key: Key, state: &mut State) -> Result<(), std::io::Error>;
}