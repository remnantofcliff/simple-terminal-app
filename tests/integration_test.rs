use simple_terminal_app::{app::App, clear, cursor, event::Key, scene::Scene};
use std::io::Write;

struct TestScene;

impl Scene for TestScene {
    fn init(&mut self, state: &mut simple_terminal_app::app::State) -> Result<(), std::io::Error> {
        write!(state, "{}", cursor::SteadyBlock)?;

        state.flush()?;

        Ok(())
    }

    fn process_input(
        &mut self,
        key_event: termion::event::Key,
        state: &mut simple_terminal_app::app::State,
    ) -> Result<(), std::io::Error> {
        match key_event {
            Key::Esc => state.stop(),

            Key::Char('c') => {
                write!(state, "{}{}", clear::All, cursor::Goto(1, 1))?;
            }

            Key::Char('p') => {
                let pos = state.position()?;

                write!(state, "{:?}", pos)?;
            }

            Key::Char('H') => {
                write!(state, "{}", cursor::Show)?;
            }

            Key::Char('h') => {
                write!(state, "{}", cursor::Hide)?;
            }

            Key::Char('s') => {
                let size = state.size()?;

                write!(state, "{:?}", size)?;
            }

            _ => {}
        }

        state.flush()?;

        Ok(())
    }
}

#[test]
fn playground() {
    App::start(Box::new(TestScene)).unwrap();
}
