use crate::input::Input;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScreenId {
    Menu,
    Instructions,
    Credits,
    Pong,
    Pacman,
    GameOver,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Transition {
    Goto(ScreenId),
    Quit,
}

pub trait Screen {
    fn update(&mut self, input: &Input, dt: f32) -> Option<Transition>;
    fn draw(&self);
    #[allow(dead_code)] // exercised by unit tests
    fn id(&self) -> ScreenId;
}

pub struct App<F: Fn(ScreenId) -> Box<dyn Screen>> {
    make: F,
    current: Box<dyn Screen>,
    pub quit: bool,
}

impl<F: Fn(ScreenId) -> Box<dyn Screen>> App<F> {
    pub fn new(make: F, start: ScreenId) -> Self {
        let current = make(start);
        App {
            make,
            current,
            quit: false,
        }
    }

    #[allow(dead_code)] // exercised by unit tests
    pub fn current_id(&self) -> ScreenId {
        self.current.id()
    }

    pub fn update(&mut self, input: &Input, dt: f32) {
        if let Some(t) = self.current.update(input, dt) {
            match t {
                Transition::Goto(id) => self.current = (self.make)(id),
                Transition::Quit => self.quit = true,
            }
        }
    }

    pub fn draw(&self) {
        self.current.draw();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fake {
        id: ScreenId,
        emit: Option<Transition>,
    }
    impl Screen for Fake {
        fn update(&mut self, _i: &Input, _dt: f32) -> Option<Transition> {
            self.emit.take()
        }
        fn draw(&self) {}
        fn id(&self) -> ScreenId {
            self.id
        }
    }

    fn make(id: ScreenId) -> Box<dyn Screen> {
        // Menu emits a Goto(Pong); every other screen is inert.
        let emit = match id {
            ScreenId::Menu => Some(Transition::Goto(ScreenId::Pong)),
            _ => None,
        };
        Box::new(Fake { id, emit })
    }

    #[test]
    fn starts_on_requested_screen() {
        let app = App::new(make, ScreenId::Menu);
        assert_eq!(app.current_id(), ScreenId::Menu);
    }

    #[test]
    fn goto_transition_swaps_screen() {
        let mut app = App::new(make, ScreenId::Menu);
        app.update(&Input::default(), 0.0);
        assert_eq!(app.current_id(), ScreenId::Pong);
        assert!(!app.quit);
    }

    #[test]
    fn quit_transition_sets_flag() {
        fn make_quit(id: ScreenId) -> Box<dyn Screen> {
            Box::new(Fake {
                id,
                emit: Some(Transition::Quit),
            })
        }
        let mut app = App::new(make_quit, ScreenId::Menu);
        app.update(&Input::default(), 0.0);
        assert!(app.quit);
    }
}
