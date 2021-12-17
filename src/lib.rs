/// Module that contains the app and state. The app can be started from app::App::start.
pub mod app;
/// Module that contains the Scene-trait that is needed for creating scenes for the app.
pub mod scene;
/// Contains all the terminal commands as structs. Command.to_string() generates the ANSI escape code.
pub mod commands {
    /// Manipulating the cursor.
    pub mod cursor {

        pub use termion::cursor::{
            BlinkingBar, BlinkingBlock, BlinkingUnderline, Down, Hide, HideCursor, Left, Restore,
            Right, Save, Show, SteadyBar, SteadyBlock, SteadyUnderline, Up,
        };

        use crate::Point;

        use std::fmt::Display;

        pub struct Goto(pub Point);

        impl Display for Goto {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", termion::cursor::Goto(self.0.x + 1, self.0.y + 1))
            }
        }
    }
    /// Module for manipulating terminal colors.
    pub mod color {
        pub use termion::color::{
            AnsiValue, Bg, Black, Blue, Cyan, Fg, Green, LightBlack, LightBlue, LightCyan,
            LightGreen, LightMagenta, LightRed, LightWhite, LightYellow, Magenta, Red, Reset,
            White, Yellow,
        };
    }

    pub use termion::{clear, style};
}
/// A point in the terminal. Starts from (0, 0)
#[derive(Clone, Copy)]
pub struct Point {
    pub x: u16,

    pub y: u16,
}
impl Point {
    // Creates a new point. Upper-left corner is (0, 0) and lower-right is state.size()
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

use std::fmt::Display;

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}


pub use termion::event;
