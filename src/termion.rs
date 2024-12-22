use termion::{
    event::{self, Event},
    input,
};

use crate::{Input, Key, Modifier, Side};

// impl From<&Event> for Key {
//     /// Convert termion [`Event`](https://docs.rs/termion/latest/termion/event/enum.Event.html) to [`Input`].
//     ///
//     /// **Note:** This implementation is only available when the `termion` feature is enabled.
//     fn from(value: &Event) -> Self {
// }

impl From<Event> for Input {
    fn from(value: Event) -> Self {
        if let Event::Key(key) = value {
            let mut modifier = Modifier::None;
            let result_key = match key {
                event::Key::Backspace => Key::Backspace,
                event::Key::Left => Key::Left,
                event::Key::ShiftLeft => {
                    modifier = Modifier::Shift(Side::Left);
                    Key::Left
                }
                event::Key::AltLeft => {
                    modifier = Modifier::Alt(Side::Left);
                    Key::Left
                }
                event::Key::CtrlLeft => {
                    modifier = Modifier::Control(Side::Left);
                    Key::Left
                }
                event::Key::Right => Key::Right,
                event::Key::ShiftRight => {
                    modifier = Modifier::Shift(Side::Left);
                    Key::Right
                }
                event::Key::AltRight => {
                    modifier = Modifier::Alt(Side::Left);
                    Key::Right
                }
                event::Key::CtrlRight => {
                    modifier = Modifier::Control(Side::Left);
                    Key::Right
                }
                event::Key::Up => Key::Up,
                event::Key::ShiftUp => {
                    modifier = Modifier::Shift(Side::Left);
                    Key::Up
                }
                event::Key::AltUp => {
                    modifier = Modifier::Alt(Side::Left);
                    Key::Up
                }
                event::Key::CtrlUp => {
                    modifier = Modifier::Control(Side::Left);
                    Key::Up
                }
                event::Key::Down => Key::Down,
                event::Key::ShiftDown => {
                    modifier = Modifier::Shift(Side::Left);
                    Key::Down
                }
                event::Key::AltDown => {
                    modifier = Modifier::Alt(Side::Left);
                    Key::Down
                }
                event::Key::CtrlDown => {
                    modifier = Modifier::Control(Side::Left);
                    Key::Down
                }
                event::Key::Home => Key::Home,
                event::Key::CtrlHome => {
                    modifier = Modifier::Control(Side::Left);
                    Key::Home
                }
                event::Key::End => Key::End,
                event::Key::CtrlEnd => {
                    modifier = Modifier::Control(Side::Left);
                    Key::End
                }
                event::Key::PageUp => Key::PageUp,
                event::Key::PageDown => Key::PageDown,
                event::Key::BackTab => Key::BackTab,
                event::Key::Delete => Key::Delete,
                event::Key::Insert => Key::Insert,
                event::Key::F(f) => Key::Func(f),
                event::Key::Char(c) => Key::Char(c),
                event::Key::Alt(c) => {
                    modifier = Modifier::Alt(Side::Left);
                    Key::Char(c)
                }
                event::Key::Ctrl(c) => {
                    modifier = Modifier::Control(Side::Left);
                    Key::Char(c)
                }
                event::Key::Esc => Key::Esc,
                _ => Key::None,
            };
            Input::new(result_key, modifier)
        } else {
            Input::default()
        }
    }
}
