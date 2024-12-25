# ratatui-input

Unifies input handling from [crossterm](https://docs.rs/crossterm/latest/crossterm/), [termion](https://docs.rs/termion/latest/termion/) and [termwiz](https://docs.rs/termwiz/latest/termwiz/)
It is meant to be used by ratatui libraries to make input handling easier, or allow people using a library to specify what input to listen to

## basic usage

The different are all disabled by default, so need the features to use them:
```toml
ratatui-input = { version = "1.0", features = ["crossterm", "termion", "termwiz"] }
```

After that just use a function like this to handle the input:
```rust
use ratatui_input::{Input, Key};

pub fn handle<I: Into<Input>>(&mut self, input: I) {
    let input: Input = input.into()
    if input.key == Key::Right {
        println!("right")
    } else if input.key == Key::Left {
        println!("left")
    }
}
```


## Acknowledgments

This library does not support all inputs the backends give and is mostly based on the inputs that crossterm uses.
