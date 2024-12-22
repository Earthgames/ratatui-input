#[cfg(feature = "crossterm")]
mod crossterm;

// #[cfg(feature = "termion")]
mod termion;

// #[cfg(feature = "termwiz")]
mod termwiz;

/// Input enum to represent all inputs a you may get.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    /// Character key
    /// like a key, h key
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
    // Function keys
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
    /// Nothing was pressed
    /// Or it was not implemented
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modifier {
    Shift(Side),
    Control(Side),
    Alt(Side),
    /// Command on macOS, Windows on Windows, Super on other platforms
    Super(Side),
    Meta(Side),
    Hyper(Side),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Left,
    Right,
}
