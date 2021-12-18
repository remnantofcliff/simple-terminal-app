use simple_terminal_app::{
    app,
    commands::{clear, color, cursor},
    event::{self, Key},
    scene::Scene,
    Point,
};

struct TestScene {
    color: u8,
}

impl Scene for TestScene {
    fn init(&mut self, state: &mut app::State) {
        state
            .command()
            .push("Hello!")
            .push(cursor::SteadyBlock)
            .execute();

        state.flush();
    }

    fn process_input(&mut self, state: &mut app::State, key_event: event::Key) {
        match key_event {
            Key::Esc => state.stop(),
            Key::Char('\n') => {
                let pos = state.cursor_position;

                state
                    .command()
                    .push(cursor::Goto(Point::new(0, pos.y + 1)))
                    .execute();
            }

            Key::Char('c') => state
                .command()
                .push(clear::All)
                .push(cursor::Goto(Point::new(0, 0)))
                .execute(),
            Key::Char('p') => {
                let pos = state.cursor_position;

                state.command().push(pos).execute();
            }

            Key::Char('H') => state.command().push(cursor::Show).execute(),
            Key::Char('h') => state.command().push(cursor::Hide).execute(),
            Key::Char('m') => state.command().push("message ").execute(),
            Key::Char('q') => state.change_scene(Box::new(QuitScene)),

            Key::Char('s') => {
                let size = state.size();

                state.command().push(size).execute();
            }

            _ => {}
        }

        state.flush();
    }

    fn update(&mut self, state: &mut app::State) {
        let size = state.size();

        let pos = state.cursor_position;

        let write_pos = Point::new(size.x - pos.to_string().len() as u16, size.y);

        state
            .command()
            .push(cursor::Save)
            .push(cursor::Goto(write_pos))
            .push(clear::CurrentLine)
            .push(color::Fg(color::AnsiValue(self.color)))
            .push(pos)
            .push(color::Fg(color::Reset))
            .push(cursor::Restore)
            .execute();

        state.flush();

        self.color = self.color.overflowing_add(1).0;
    }
}

struct QuitScene;

impl Scene for QuitScene {
    fn init(&mut self, state: &mut app::State) {
        state
            .command()
            .push(clear::All)
            .push(cursor::Goto(Point::new(0, 0)))
            .push("Press any key to exit. ")
            .execute();

        state.flush();
    }

    fn process_input(&mut self, state: &mut app::State, _key: Key) {
        state.command().push("See ya!").execute();

        state.flush();

        std::thread::sleep(std::time::Duration::from_secs(1));

        state.stop();
    }

    fn update(&mut self, _state: &mut app::State) {}
}

fn main() {
    app::start(Box::new(TestScene { color: 0 }), None);
}
