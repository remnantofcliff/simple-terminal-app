use std::io::Write;
use termion::{clear, cursor};

use crate::app::State;

/// Clears the screen and sets the cursor position back to (1, 1) but doesn't flush the stdout buffer.
pub fn clear(state: &mut State) -> Result<(), std::io::Error> {
    write!(state.stdout, "{}{}", clear::All, cursor::Goto(1, 1))?;

    Ok(())
}
