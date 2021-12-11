use simple_terminal_app::{app::App, cursor, event::Key, scene::Scene};
use std::io::Write;

struct TestScene;

impl Scene for TestScene {
    fn init(&self, state: &mut simple_terminal_app::app::State) -> Result<(), std::io::Error> {
        write!(state.stdout, "{}", cursor::SteadyBlock)?;

        state.stdout.flush()?;

        Ok(())
    }

    fn process_input(
        &self,
        key_event: termion::event::Key,
        state: &mut simple_terminal_app::app::State,
    ) -> Result<(), std::io::Error> {
        match key_event {
            Key::Esc => state.running = false,

            Key::Char('p') => {
                let pos = state.position()?;

                write!(state.stdout, "{:?}", pos)?;
            }

            Key::Char('H') => {
                write!(state.stdout, "{}", cursor::Show)?;
            }

            Key::Char('h') => {
                write!(state.stdout, "{}", cursor::Hide)?;
            }

            Key::Char('s') => {
                let size = state.size()?;

                write!(state.stdout, "{:?}", size)?;
            }

            _ => {}
        }

        state.stdout.flush()?;

        Ok(())
    }
}

#[test]
fn state_test() {
    App::start(Box::new(TestScene)).unwrap();
}
