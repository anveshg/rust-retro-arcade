//! Screen management: the [`Screen`] trait, the [`App`] state machine, and the
//! screen-transition types that wire them together.
//!
//! Rust concepts showcased: **traits** (like interfaces/abstract base classes),
//! **trait objects** (`Box<dyn Screen>`, runtime dispatch via vtable), **generics
//! with trait bounds** (`App<F: Fn(...)>`), **enums with associated data**
//! (`Transition::Goto(ScreenId)`), and `Option<T>` as a null-free "maybe a value."
use crate::input::Input;

/// Identifies which logical screen (game state) is currently active.
///
/// Rust `enum` variants are plain names — no hidden integer is assigned unless
/// you ask (e.g. `ScreenId::Menu as u8`). The `#[derive(...)]` line auto-
/// generates common trait impls: `Clone`/`Copy` for cheap duplication,
/// `PartialEq`/`Eq` for `==` comparisons, and `Debug` for `{:?}` formatting.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScreenId {
    /// The main menu screen.
    Menu,
    /// The how-to-play / instructions screen.
    Instructions,
    /// The credits screen.
    Credits,
    /// The Pong mini-game screen.
    Pong,
    /// The Pac-Man mini-game screen.
    Pacman,
    /// The shared game-over screen.
    GameOver,
}

/// A command a `Screen` returns to signal what should happen next.
///
/// Unlike C enums (which are just integers), Rust enums can carry data.
/// `Transition::Goto` bundles a `ScreenId` inside the variant — the compiler
/// ensures you cannot construct a `Goto` without supplying an id.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Transition {
    /// Switch to the screen identified by the enclosed `ScreenId`.
    ///
    /// This is a **data-carrying variant**: `Goto(ScreenId)` is like a tiny
    /// one-field tuple bundled with the variant name. A `match` arm can
    /// destructure it: `Transition::Goto(id) => use_id(id)`.
    Goto(ScreenId),
    /// Exit the application entirely.
    Quit,
}

/// The interface every game screen must satisfy.
///
/// **Traits** are Rust's version of interfaces or abstract base classes. A type
/// becomes a `Screen` by implementing all required methods below. The trait
/// itself carries no data — it only promises behavior. Any number of unrelated
/// types can implement the same trait independently.
pub trait Screen {
    /// Advance the screen's state by one frame and return any transition.
    ///
    /// `&mut self` gives **exclusive mutable access** to the implementor's
    /// fields. The `Option<Transition>` return is Rust's null-free "maybe":
    /// `Some(t)` means "apply this transition now"; `None` means "stay here."
    fn update(&mut self, input: &Input, dt: f32) -> Option<Transition>;
    /// Render the current frame.
    ///
    /// Takes `&self` (a shared, read-only borrow) rather than `&mut self`
    /// because drawing should never mutate game state — the compiler enforces
    /// this: a `&mut` method cannot be called through a `&self` reference.
    fn draw(&self);
    /// Return this screen's `ScreenId`.
    ///
    /// The `#[allow(dead_code)]` attribute suppresses the "never used" warning.
    /// This method is only called from unit tests, so the main binary never
    /// references it. Rather than delete it, we keep it for testability and
    /// acknowledge the situation explicitly with the attribute.
    #[allow(dead_code)] // exercised by unit tests
    fn id(&self) -> ScreenId;
}

/// The top-level application state machine.
///
/// **Generics with a trait bound**: `F: Fn(ScreenId) -> Box<dyn Screen>` means
/// "F is any callable that maps a `ScreenId` to a boxed screen." Every Rust
/// closure has its own unique anonymous type, so a generic parameter is the
/// only way to store one without erasing its concrete type via dynamic
/// dispatch. The compiler monomorphises (generates a specialised copy) of
/// `App` for the exact closure type used at the call site.
///
/// `Box<dyn Screen>` is a **trait object**: `dyn` means "dispatch method calls
/// at runtime through a vtable pointer," just like a C++ virtual call. This
/// lets `current` hold *any* `Screen` implementor chosen at runtime — the cost
/// is one extra pointer indirection per call.
pub struct App<F: Fn(ScreenId) -> Box<dyn Screen>> {
    /// Factory closure: given a `ScreenId`, constructs the matching screen.
    /// Stored as generic `F` so the compiler can inline the call.
    make: F,
    /// The currently displayed screen, heap-allocated through a trait object.
    current: Box<dyn Screen>,
    /// Polled by the game loop; set `true` when `Transition::Quit` is received.
    pub quit: bool,
}

impl<F: Fn(ScreenId) -> Box<dyn Screen>> App<F> {
    /// Construct a new `App`, immediately building the starting screen.
    ///
    /// `Self` is shorthand for the implementing type (`App<F>`). The `make`
    /// closure is called once here to seed `current`.
    pub fn new(make: F, start: ScreenId) -> Self {
        let current = make(start);
        App {
            make,
            current,
            quit: false,
        }
    }

    /// Return the `ScreenId` of the currently active screen.
    /// Only called from tests; see the `#[allow(dead_code)]` note on
    /// `Screen::id` for why the attribute is present.
    #[allow(dead_code)] // exercised by unit tests
    pub fn current_id(&self) -> ScreenId {
        self.current.id()
    }

    /// Drive one frame of logic: ask the active screen to update itself, then
    /// act on any `Transition` it returns.
    ///
    /// `&mut self` grants exclusive access — the borrow checker prevents any
    /// other live reference to this `App` while this method is running.
    pub fn update(&mut self, input: &Input, dt: f32) {
        // `if let Some(t)` combines "does the Option hold a value?" with
        // "bind that value to `t`" in one expression — no null-pointer risk,
        // no unwrap that could panic.
        if let Some(t) = self.current.update(input, dt) {
            // `match` is exhaustive: the compiler rejects this unless every
            // `Transition` variant is handled. Adding a new variant turns
            // every unhandled match site into a compile error automatically.
            match t {
                Transition::Goto(id) => self.current = (self.make)(id),
                Transition::Quit => self.quit = true,
            }
        }
    }

    /// Delegate rendering to the active screen. `&self` — no mutation allowed.
    pub fn draw(&self) {
        self.current.draw();
    }
}

/// Unit tests compiled only during `cargo test` — absent from release builds.
///
/// `#[cfg(test)]` is a conditional-compilation attribute: the whole module is
/// stripped from non-test builds, so test helpers never bloat the binary.
#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal `Screen` stub: emits `emit` on the first `update`, then `None`.
    struct Fake {
        /// The id this fake screen reports via `Screen::id`.
        id: ScreenId,
        /// Transition to fire on the first update (consumed via `Option::take`).
        emit: Option<Transition>,
    }
    impl Screen for Fake {
        /// Consume and return `emit` (if any), leaving `None` in its place.
        fn update(&mut self, _i: &Input, _dt: f32) -> Option<Transition> {
            // `Option::take` swaps `self.emit` with `None` and returns the old
            // value — a tidy one-shot event flag with no extra state variable.
            self.emit.take()
        }
        /// No-op: visual output is not tested.
        fn draw(&self) {}
        /// Return the stored `ScreenId`.
        fn id(&self) -> ScreenId {
            self.id
        }
    }

    /// Test factory: `Menu` emits `Goto(Pong)`; every other screen is inert.
    fn make(id: ScreenId) -> Box<dyn Screen> {
        // Menu emits a Goto(Pong); every other screen is inert.
        let emit = match id {
            ScreenId::Menu => Some(Transition::Goto(ScreenId::Pong)),
            _ => None,
        };
        Box::new(Fake { id, emit })
    }

    /// Verify that `App::new` lands on the screen passed as `start`.
    #[test]
    fn starts_on_requested_screen() {
        let app = App::new(make, ScreenId::Menu);
        assert_eq!(app.current_id(), ScreenId::Menu);
    }

    /// Verify that a `Goto` transition replaces `current` with the new screen.
    #[test]
    fn goto_transition_swaps_screen() {
        let mut app = App::new(make, ScreenId::Menu);
        app.update(&Input::default(), 0.0);
        assert_eq!(app.current_id(), ScreenId::Pong);
        assert!(!app.quit);
    }

    /// Verify that a `Quit` transition sets `app.quit` to `true`.
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
