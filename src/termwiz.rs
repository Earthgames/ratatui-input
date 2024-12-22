use termwiz::input::{InputEvent, KeyCode, Modifiers};

use crate::Input;

use super::{Key, Modifier, Side};

impl From<&InputEvent> for Input {
    /// Convert termwiz [`InputEvent`](https://docs.rs/termwiz/latest/termwiz/input/enum.InputEvent.html) to [`Input`].
    ///
    /// **Note:** This implementation is only available when the `termwiz` feature is enabled.
    fn from(value: &InputEvent) -> Self {
        match value {
            InputEvent::Key(key) => {
                let result_key = key.key.into();
                let modifier = key.modifiers.into();
                Input::new(result_key, modifier)
            }
            _ => Input::default(),
        }
    }
}

impl From<KeyCode> for Key {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::DownArrow => Key::Down,
            KeyCode::UpArrow => Key::Up,
            KeyCode::LeftArrow => Key::Left,
            KeyCode::RightArrow => Key::Right,
            KeyCode::Char(c) => Key::Char(c),
            KeyCode::Hyper => Key::Modifier(Modifier::Hyper(Side::Left)),
            KeyCode::Super => Key::Modifier(Modifier::Super(Side::Left)),
            KeyCode::Meta => Key::Modifier(Modifier::Meta(Side::Left)),
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Tab => Key::Tab,
            KeyCode::Enter => Key::Enter,
            KeyCode::Shift => Key::Modifier(Modifier::Shift(Side::Left)),
            KeyCode::LeftShift => Key::Modifier(Modifier::Shift(Side::Left)),
            KeyCode::RightShift => Key::Modifier(Modifier::Shift(Side::Right)),
            KeyCode::Escape => Key::Esc,
            KeyCode::Control => Key::Modifier(Modifier::Control(Side::Left)),
            KeyCode::LeftControl => Key::Modifier(Modifier::Control(Side::Left)),
            KeyCode::RightControl => Key::Modifier(Modifier::Control(Side::Right)),
            KeyCode::Alt => Key::Modifier(Modifier::Alt(Side::Right)),
            KeyCode::LeftAlt => Key::Modifier(Modifier::Alt(Side::Right)),
            KeyCode::RightAlt => Key::Modifier(Modifier::Alt(Side::Right)),
            KeyCode::Menu => Key::Menu,
            KeyCode::LeftMenu => Key::Menu,
            KeyCode::RightMenu => Key::Menu,
            KeyCode::Pause => Key::Pause,
            KeyCode::CapsLock => Key::CapsLock,
            KeyCode::PageUp => Key::PageUp,
            KeyCode::PageDown => Key::PageDown,
            KeyCode::End => Key::End,
            KeyCode::Home => Key::Home,
            KeyCode::PrintScreen => Key::PrintScreen,
            KeyCode::Insert => Key::Insert,
            KeyCode::Delete => Key::Delete,
            KeyCode::LeftWindows => Key::Modifier(Modifier::Super(Side::Left)),
            KeyCode::RightWindows => Key::Modifier(Modifier::Super(Side::Right)),
            KeyCode::Function(f) => Key::Func(f),
            KeyCode::NumLock => Key::NumLock,
            KeyCode::ScrollLock => Key::ScrollLock,
            KeyCode::VolumeMute => Key::MuteVolume,
            KeyCode::VolumeDown => Key::LowerVolume,
            KeyCode::VolumeUp => Key::RaisVolume,
            KeyCode::MediaNextTrack => Key::MediaNext,
            KeyCode::MediaPrevTrack => Key::MediaPrevious,
            KeyCode::MediaStop => Key::MediaStop,
            KeyCode::MediaPlayPause => Key::MediaPlayPause,
            KeyCode::KeyPadHome => Key::Home,
            KeyCode::KeyPadEnd => Key::End,
            KeyCode::KeyPadPageUp => Key::PageUp,
            KeyCode::KeyPadPageDown => Key::Down,
            _ => todo!(),
        }
    }
}

impl From<Modifiers> for Modifier {
    fn from(value: termwiz::input::Modifiers) -> Self {
        match value {
            Modifiers::SUPER => Modifier::Super(Side::Left),
            Modifiers::CTRL => Modifier::Control(Side::Left),
            Modifiers::LEFT_CTRL => Modifier::Control(Side::Left),
            Modifiers::RIGHT_CTRL => Modifier::Control(Side::Right),
            Modifiers::ALT => Modifier::Alt(Side::Left),
            Modifiers::LEFT_ALT => Modifier::Alt(Side::Left),
            Modifiers::RIGHT_ALT => Modifier::Alt(Side::Right),
            Modifiers::SHIFT => Modifier::Shift(Side::Left),
            Modifiers::LEFT_SHIFT => Modifier::Shift(Side::Left),
            Modifiers::RIGHT_SHIFT => Modifier::Shift(Side::Right),
            _ => Modifier::None,
        }
    }
}
