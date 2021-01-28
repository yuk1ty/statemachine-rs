# statemachine

A lightweight state machine library which has no dependency.

## Usage

Let's have a look at the following example.

```rust
use statemachine::machine::builder::StateMachineBuilder;
use statemachine::machine::StateMachine;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Stations {
    Shibuya,
    IkejiriOhashi,
    Sangendyaya,
    KomazawaDaigaku,
    Sakurashinmachi,
    Yoga,
    FutakoTamagawa,
}

enum Train {
    Local,
    Express,
}

fn main() {
    let mut sm = StateMachineBuilder::start()
            .initial_state(Stations::Shibuya)
            .transition(|station, train| match (station, train) {
                (Stations::Shibuya, Train::Local) => Stations::IkejiriOhashi,
                (Stations::Shibuya, Train::Express) => Stations::Sangendyaya,
                (Stations::IkejiriOhashi, Train::Local) => Stations::Sangendyaya,
                (Stations::Sangendyaya, Train::Local) => Stations::KomazawaDaigaku,
                (Stations::Sangendyaya, Train::Express) => Stations::FutakoTamagawa,
                (Stations::KomazawaDaigaku, Train::Local) => Stations::Sakurashinmachi,
                (Stations::Sakurashinmachi, Train::Local) => Stations::Yoga,
                _ => unreachable!(),
            })
            .build();

    assert_eq!(Stations::Sangendyaya, sm.consume(Train::Express));
}
```
