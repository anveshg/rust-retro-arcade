//! The `screens` module bundles every full-screen "page" in the arcade.
//!
//! In Rust a module lives in its own file; `pub mod foo` here tells the
//! compiler to look for `src/screens/foo.rs` and compile it as a child module.
//! The `pub use` lines re-export each type so callers can write `screens::Menu`
//! instead of the longer `screens::menu::Menu` — flattening the public path.

pub mod credits;
pub mod gameover;
pub mod instructions;
pub mod menu;

pub use credits::Credits;
pub use gameover::GameOver;
pub use instructions::Instructions;
pub use menu::Menu;
