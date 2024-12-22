use crossterm::event::{Event, KeyCode};

use super::{Key, Modifier, Side};

impl From<&Event> for Key {
    fn from(value: &Event) -> Self {
        if let Event::Key(key) = value {
            if matches!(
                key.kind,
                crossterm::event::KeyEventKind::Press | crossterm::event::KeyEventKind::Repeat
            ) {
                return key.code.into();
            }
        }

        Key::None
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
                crossterm::event::MediaKeyCode::Play => Key::MediaPlay,
                crossterm::event::MediaKeyCode::Pause => Key::MediaPause,
                crossterm::event::MediaKeyCode::PlayPause => Key::MediaPlayPause,
                crossterm::event::MediaKeyCode::Stop => Key::MediaStop,
                crossterm::event::MediaKeyCode::TrackNext => Key::MediaNext,
                crossterm::event::MediaKeyCode::TrackPrevious => Key::MediaPrevious,
                crossterm::event::MediaKeyCode::LowerVolume => Key::LowerVolume,
                crossterm::event::MediaKeyCode::RaiseVolume => Key::RaisVolume,
                crossterm::event::MediaKeyCode::MuteVolume => Key::MuteVolume,
                _ => Key::None,
            },
            KeyCode::Modifier(modifier) => match modifier {
                crossterm::event::ModifierKeyCode::LeftShift => {
                    Key::Modifier(Modifier::Shift(Side::Left))
                }
                crossterm::event::ModifierKeyCode::LeftControl => {
                    Key::Modifier(Modifier::Control(Side::Left))
                }
                crossterm::event::ModifierKeyCode::LeftAlt => {
                    Key::Modifier(Modifier::Alt(Side::Left))
                }
                crossterm::event::ModifierKeyCode::LeftSuper => {
                    Key::Modifier(Modifier::Super(Side::Left))
                }
                crossterm::event::ModifierKeyCode::LeftHyper => {
                    Key::Modifier(Modifier::Hyper(Side::Left))
                }
                crossterm::event::ModifierKeyCode::LeftMeta => {
                    Key::Modifier(Modifier::Meta(Side::Left))
                }
                crossterm::event::ModifierKeyCode::RightShift => {
                    Key::Modifier(Modifier::Shift(Side::Right))
                }
                crossterm::event::ModifierKeyCode::RightControl => {
                    Key::Modifier(Modifier::Control(Side::Right))
                }
                crossterm::event::ModifierKeyCode::RightAlt => {
                    Key::Modifier(Modifier::Alt(Side::Right))
                }
                crossterm::event::ModifierKeyCode::RightSuper => {
                    Key::Modifier(Modifier::Super(Side::Right))
                }
                crossterm::event::ModifierKeyCode::RightHyper => {
                    Key::Modifier(Modifier::Hyper(Side::Right))
                }
                crossterm::event::ModifierKeyCode::RightMeta => {
                    Key::Modifier(Modifier::Meta(Side::Right))
                }
                _ => Key::None,
            },
            _ => Key::None,
        }
    }
}
