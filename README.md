[![Workflow Status](https://github.com/yuk1ty/statemachine-rs/workflows/Rust/badge.svg)](https://github.com/yuk1ty/statemachine-rs/actions?query=workflow%3A%22Rust%22)

# statemachine-rs

A zero dependency crate to implement state machine.

### Usage

Let's have a look at the following simple example. This example shows the state machine
can transit its number (it called `current_state` in this machine)
from given string ("next") and then, it produces outputs.

```rust
use statemachine_rs::machine::{
    builder::{BasicStateMachineBuilder, StateMachineBuilder},
    StateMachine,
};

fn main() {
    let sm = BasicStateMachineBuilder::start()
        .initial_state(1)
        .transition(|state, input| match (state, input) {
            (1, "next") => 2,
            (2, "next") => 3,
            _ => unreachable!(),
        })
        .build()
        .unwrap();

    assert_eq!(1, sm.current_state());
    sm.consume("next");
    assert_eq!(2, sm.current_state());
}
```

You can assemble your state machine by using `statemachine_rs::machine::builder::BasicStateMachineBuilder`.
`BasicStateMachineBuilder::initial_state()` initializes the initial state of its machine.
`BasicStateMachineBuilder::transition()` defines the transition model.

Of cource we can use `enum`s for representing states and inputs. Let's have a look at another example.

The following example describes if you press the button, the state turns to be `On`. Otherwise, `Off`.

```rust
use statemachine_rs::machine::{
    builder::{BasicStateMachineBuilder, StateMachineBuilder},
    StateMachine,
};

#[derive(Clone, Debug, PartialEq)]
enum ButtonState {
    On,
    Off,
}

enum Input {
    Press,
}

fn main() {
    let sm = BasicStateMachineBuilder::start()
        .initial_state(ButtonState::Off)
        .transition(|state, input| match (state, input) {
            (ButtonState::On, Input::Press) => ButtonState::Off,
            (ButtonState::Off, Input::Press) => ButtonState::On,
        })
        .build()
        .unwrap();

    assert_eq!(ButtonState::Off, sm.current_state());
    sm.consume(Input::Press);
    assert_eq!(ButtonState::On, sm.current_state());
}
```

### License

MIT

### Contribution

All contributions are welcome.

If you have an idea to improve this crate, create new issue or submit new pull request.

License: MIT
