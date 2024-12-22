use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, MediaKeyCode, ModifierKeyCode};

use crate::Input;

use super::{Key, Modifier, Side};

impl From<&Event> for Input {
    fn from(value: &Event) -> Self {
        if let Event::Key(key) = value {
            if matches!(
                key.kind,
                KeyEventKind::Press | crossterm::event::KeyEventKind::Repeat
            ) {
                let result_key: Key = key.code.into();
                let modifier = key.modifiers.into();
                return Input::new(result_key, modifier);
            }
        }

        Input::default()
    }
}

impl From<KeyCode> for Key {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::Char(c) => Key::Char(c),
            KeyCode::Up => Key::Up,
            KeyCode::Down => Key::Down,
            KeyCode::Left => Key::Left,
            KeyCode::Right => Key::Right,
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Enter => Key::Enter,
            KeyCode::Home => Key::Home,
            KeyCode::End => Key::End,
            KeyCode::PageUp => Key::PageUp,
            KeyCode::PageDown => Key::PageDown,
            KeyCode::Tab => Key::Tab,
            KeyCode::BackTab => Key::BackTab,
            KeyCode::Delete => Key::Delete,
            KeyCode::Insert => Key::Insert,
            KeyCode::F(n) => Key::Func(n),
            KeyCode::Esc => Key::Esc,
            KeyCode::CapsLock => Key::CapsLock,
            KeyCode::ScrollLock => Key::ScrollLock,
            KeyCode::NumLock => Key::NumLock,
            KeyCode::PrintScreen => Key::PrintScreen,
            KeyCode::Pause => Key::Pause,
            KeyCode::Menu => Key::Menu,
            KeyCode::Media(media) => match media {
                MediaKeyCode::Play => Key::MediaPlay,
                MediaKeyCode::Pause => Key::MediaPause,
                MediaKeyCode::PlayPause => Key::MediaPlayPause,
                MediaKeyCode::Stop => Key::MediaStop,
                MediaKeyCode::TrackNext => Key::MediaNext,
                MediaKeyCode::TrackPrevious => Key::MediaPrevious,
                MediaKeyCode::LowerVolume => Key::LowerVolume,
                MediaKeyCode::RaiseVolume => Key::RaisVolume,
                MediaKeyCode::MuteVolume => Key::MuteVolume,
                _ => Key::None,
            },
            KeyCode::Modifier(modifier) => Key::Modifier(modifier.into()),
            _ => Key::None,
        }
    }
}

impl From<ModifierKeyCode> for Modifier {
    fn from(value: ModifierKeyCode) -> Self {
        match value {
            ModifierKeyCode::LeftShift => Modifier::Shift(Side::Left),
            ModifierKeyCode::LeftControl => Modifier::Control(Side::Left),
            ModifierKeyCode::LeftAlt => Modifier::Alt(Side::Left),
            ModifierKeyCode::LeftSuper => Modifier::Super(Side::Left),
            ModifierKeyCode::LeftHyper => Modifier::Hyper(Side::Left),
            ModifierKeyCode::LeftMeta => Modifier::Meta(Side::Left),
            ModifierKeyCode::RightShift => Modifier::Shift(Side::Right),
            ModifierKeyCode::RightControl => Modifier::Control(Side::Right),
            ModifierKeyCode::RightAlt => Modifier::Alt(Side::Right),
            ModifierKeyCode::RightSuper => Modifier::Super(Side::Right),
            ModifierKeyCode::RightHyper => Modifier::Hyper(Side::Right),
            ModifierKeyCode::RightMeta => Modifier::Meta(Side::Right),
            _ => Modifier::None,
        }
    }
}

impl From<KeyModifiers> for Modifier {
    fn from(value: KeyModifiers) -> Self {
        match value {
            KeyModifiers::SHIFT => Modifier::Shift(Side::Left),
            KeyModifiers::CONTROL => Modifier::Control(Side::Left),
            KeyModifiers::ALT => Modifier::Alt(Side::Left),
            KeyModifiers::META => Modifier::Meta(Side::Left),
            KeyModifiers::SUPER => Modifier::Super(Side::Left),
            KeyModifiers::HYPER => Modifier::Hyper(Side::Left),
            KeyModifiers::NONE => Modifier::None,
            _ => Modifier::None,
        }
    }
}
