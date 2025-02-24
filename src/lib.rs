#[cfg(feature = "crossterm")]
mod crossterm;

#[cfg(feature = "termion")]
mod termion;

#[cfg(feature = "termwiz")]
mod termwiz;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Input {
    pub key: Key,
    pub modifier: Modifier,
}

impl Input {
    pub fn new(key: Key, modifier: Modifier) -> Input {
        Input { key, modifier }
    }
    pub fn new_key(key: Key) -> Input {
        Input {
            key,
            modifier: Modifier::None,
        }
    }
    pub fn keys(keys: &[Key]) -> Vec<Input> {
        keys.iter().map(|key| Self::new_key(*key)).collect()
    }
    pub fn with_key(&mut self, key: Key) -> &mut Self {
        self.key = key;
        self
    }
    pub fn with_modifier(&mut self, modifier: Modifier) -> &mut Self {
        self.modifier = modifier;
        self
    }
}

impl Default for Input {
    fn default() -> Self {
        Self {
            key: Key::None,
            modifier: Modifier::None,
        }
    }
}

/// Input enum to represent all keys you may get
#[derive(Debug, Clone, Copy, Eq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    /// Character key
    /// like `a` key, `h` key
    Char(char),
    Esc,
    Backspace,
    Enter,
    Tab,
    CapsLock,
    // home island
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    Insert,
    /// Function keys
    Func(u8),
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    Menu,
    BackTab,
    // media
    MediaPlay,
    MediaPause,
    MediaPlayPause,
    MediaStop,
    MediaNext,
    MediaPrevious,
    RaisVolume,
    LowerVolume,
    MuteVolume,
    /// If a modifier is pressed on it's own
    Modifier(Modifier),
    /// Is `PartialEq` to any Key.
    /// Useful for when you don't care about the value of the key
    Any,
    /// Nothing was pressed
    /// Or it was not implemented
    None,
}

macro_rules! really_match {
    ($($kind:ident),+) => {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Self::Any => true,
                $(Self::$kind(a) => {
                    if let Self::$kind(b) = other {
                        a == b
                    } else {
                        matches!(other, Self::Any)
                    }
                })+
                _ => match other {
                    Self::Any => true,
                    _ => core::mem::discriminant(self) == core::mem::discriminant(other),
                },
            }
        }
    };
}

impl PartialEq for Key {
    really_match!(Char, Func, Modifier);
}

#[derive(Debug, Clone, Copy, Eq)]
pub enum Modifier {
    Shift(Side),
    Control(Side),
    Alt(Side),
    /// Command on macOS, Windows on Windows, Super on other platforms
    Super(Side),
    Meta(Side),
    Hyper(Side),
    /// Is `PartialEq` to any Modifier.
    /// Useful for when you don't care about the value of the modifier
    Any,
    /// No modifier
    None,
}
impl PartialEq for Modifier {
    really_match!(Shift, Control, Alt, Super, Meta, Hyper);
}

#[derive(Debug, Clone, Copy, Eq)]
/// If there is no side reported; Left will used as default
pub enum Side {
    Left,
    Right,
    /// Is `PartialEq` to any Side.
    /// Useful for when you don't want to pick a side
    Any,
}

impl PartialEq for Side {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Any => true,
            _ => match other {
                Self::Any => true,
                _ => core::mem::discriminant(self) == core::mem::discriminant(other),
            },
        }
    }
}
