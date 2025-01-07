use crossterm::event;
use ratatui::{prelude::*, widgets::Paragraph};
use ratatui_eventInput::{self, Input, Key};

fn main() {
    let mut _quiet = false;

    let mut terminal = ratatui::init();
    loop {
        let event = event::read().unwrap();
        let input: Input = (&event).into();
        if let Key::Char('q') = input.key {
            break;
        }
        terminal
            .draw(|frame| {
                Paragraph::new(format!("{:?}", input))
                    .centered()
                    .render(frame.area(), frame.buffer_mut())
            })
            .unwrap();
    }
    ratatui::restore();
}
