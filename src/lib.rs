mod buffer;
mod camera;
mod events;
mod options;
mod plugin;
mod pos;
mod primitives;
mod render;
mod systems;
mod util;

pub use buffer::*;
pub use camera::*;
pub use events::TermState;
pub use options::*;
pub use plugin::*;
pub use pos::*;
pub use primitives::*;
pub use render::*;
pub use util::*;

/// (Re-export of crossterm's [`Event`](https://docs.rs/crossterm/0.18.0/crossterm/event/enum.Event.html))
pub use crossterm::event::Event as TermEvent;
/// (Re-export of crossterm's [`KeyCode`](https://docs.rs/crossterm/0.18.0/crossterm/event/enum.KeyCode.html))
pub use crossterm::event::KeyCode as TermKeyCode;
/// (Re-export of crossterm's [`KeyEvent`](https://docs.rs/crossterm/0.18.0/crossterm/event/struct.KeyEvent.html))
pub use crossterm::event::KeyEvent as TermKeyEvent;
/// (Re-export of crossterm's [`KeyModifiers`](https://docs.rs/crossterm/0.18.0/crossterm/event/struct.KeyModifiers.html))
pub use crossterm::event::KeyModifiers as TermKeyModifiers;
/// (Re-export of crossterm's [`Color`](https://docs.rs/crossterm/0.18.0/crossterm/style/enum.Color.html))
pub use crossterm::style::Color as TermColor;
/// (Re-export of crossterm's [`ContentStyle`](https://docs.rs/crossterm/0.18.0/crossterm/style/struct.ContentStyle.html))
pub use crossterm::style::ContentStyle;
