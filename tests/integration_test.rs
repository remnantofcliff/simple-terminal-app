use simple_terminal_app::{app::App, scene::Scene};
use std::io::{Write, stdout};
use termion::event::Key;
use termion::cursor::DetectCursorPos;

struct TestScene;

impl Scene for TestScene {
    fn init(&self, state: &mut simple_terminal_app::app::State) -> Result<(), std::io::Error> {
        write!(state.stdout, "123")?;
        state.stdout.flush()?;
        println!("{:?}", state.stdout.cursor_pos()?);
        Ok(())
    }

    fn process_input(
        &self,
        key_event: termion::event::Key,
        state: &mut simple_terminal_app::app::State,
    ) -> Result<(), std::io::Error> {
        match key_event {
            Key::Esc => state.running = false,
            Key::Char('s') => {
                let pos = state.stdout.cursor_pos()?;
                write!(state.stdout, "{:?}", pos)?;
                stdout().flush()?;
            }
            _ => {}

        }
        Ok(())
    }
}

#[test]
fn get_cursor_pos_test() {
    App::start(Box::new(TestScene)).unwrap();
}
