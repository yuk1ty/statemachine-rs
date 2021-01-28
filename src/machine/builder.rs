use std::marker::PhantomData;

use super::{BasicStateMachine, StateMachine};

pub struct StateMachineBuilder<State, Input, Transition>
where
    Transition: FnMut(&State, &Input) -> State,
    State: Copy,
{
    initial_state: Option<State>,
    transition: Option<Transition>,
    _marker: PhantomData<Input>,
}

impl<State, Input, Transition> StateMachineBuilder<State, Input, Transition>
where
    Transition: FnMut(&State, &Input) -> State,
    State: Copy,
{
    pub fn start() -> Self {
        Self::default()
    }

    pub fn initial_state(mut self, state: State) -> Self {
        self.initial_state = Some(state);
        self
    }

    pub fn transition(mut self, next: Transition) -> Self {
        self.transition = Some(next);
        self
    }

    pub fn build(self) -> impl StateMachine<State, Input> {
        BasicStateMachine {
            initial_state: self.initial_state.unwrap(),
            current_state: self.initial_state.unwrap(),
            transition: self.transition.unwrap(),
            _maker: self._marker,
        }
    }
}

impl<State, Input, Transition> Default for StateMachineBuilder<State, Input, Transition>
where
    Transition: FnMut(&State, &Input) -> State,
    State: Copy,
{
    fn default() -> Self {
        StateMachineBuilder {
            initial_state: None,
            transition: None,
            _marker: PhantomData::<Input>::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::machine::StateMachine;

    use super::StateMachineBuilder;

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    enum Train {
        Local,
        Express,
    }

    #[test]
    fn test_build() {
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
}
