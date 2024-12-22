use termion::event::Event;

use super::Key;

// impl From<&Event> for Key {
//     /// Convert termion [`Event`](https://docs.rs/termion/latest/termion/event/enum.Event.html) to [`Input`].
//     ///
//     /// **Note:** This implementation is only available when the `termion` feature is enabled.
//     fn from(value: &Event) -> Self {
// }

impl From<Event::Key> for Key {
    fn from(value: Event::Key) -> Self {}
}
