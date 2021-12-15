use simple_terminal_app::{
    app::App,
    commands::{clear, cursor},
    event::Key,
    scene::Scene,
    Point,
};
use std::io::Write;

struct TestScene;

impl Scene for TestScene {
    fn init(&mut self, state: &mut simple_terminal_app::app::State) -> Result<(), std::io::Error> {
        state
            .command()
            .append("Hello!")
            .append(cursor::SteadyBlock)
            .execute()?;

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
                state
                    .command()
                    .append(clear::All)
                    .append(cursor::Goto(Point::new(0, 0)))
                    .execute()?;
            }

            Key::Char('p') => {
                let pos = state.position()?;

                state.command().append(pos).execute()?;
            }

            Key::Char('H') => {
                state.command().append(cursor::Show).execute()?;
            }

            Key::Char('h') => {
                state.command().append(cursor::Hide).execute()?;
            }

            Key::Char('s') => {
                let size = state.size()?;

                state.command().append(size).execute()?;
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
