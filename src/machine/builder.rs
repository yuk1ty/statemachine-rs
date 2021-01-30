use std::{cell::RefCell, marker::PhantomData};

use super::{error::StateMachineError, BasicStateMachine, StateMachine, StateWrapper};

/// This builder enables us to assemble StateMachine
/// (like [`crate::machine::BasicStateMachine`]) more easily.
pub struct StateMachineBuilder<State, Input, Transition>
where
    Transition: Fn(&State, &Input) -> State,
    State: Clone,
{
    initial_state: Option<State>,
    current_state: Option<State>,
    transition: Option<Transition>,
    _marker: PhantomData<Input>,
}

impl<State, Input, Transition> StateMachineBuilder<State, Input, Transition>
where
    Transition: Fn(&State, &Input) -> State,
    State: Clone,
{
    /// Starts the builder.
    pub fn start() -> Self {
        Self::default()
    }

    /// Sets particular initial state to the state machine.
    pub fn initial_state(mut self, state: State) -> Self {
        self.initial_state = Some(state);
        self
    }

    /// Sets particular state to the current state.
    pub fn current_state(mut self, state: State) -> Self {
        self.current_state = Some(state);
        self
    }

    /// Sets particular transition algorithm to the state machine.
    pub fn transition(mut self, next: Transition) -> Self {
        self.transition = Some(next);
        self
    }

    /// To finish the builder. If it fails, returns [`crate::machine::error::StateMachineError`].
    pub fn build(self) -> Result<impl StateMachine<State, Input>, Box<dyn std::error::Error>> {
        match (self.initial_state, self.transition) {
            (Some(initial_state), Some(transition)) => Ok(BasicStateMachine {
                initial_state: initial_state.clone(),
                current_state: {
                    // If `current_state` in this builder is still `None`,
                    // sets `initial_state` as the current state forcibly.
                    let current_state = self.current_state;
                    match current_state {
                        Some(state) => RefCell::new(StateWrapper::new(state)),
                        None => RefCell::new(StateWrapper::new(initial_state)),
                    }
                },
                transition,
                _maker: self._marker,
            }),
            (None, _) => Err(Box::new(StateMachineError::MissingField(
                "initial_state".to_string(),
            ))),
            (_, None) => Err(Box::new(StateMachineError::MissingField(
                "transition".to_string(),
            ))),
        }
    }
}

impl<State, Input, Transition> Default for StateMachineBuilder<State, Input, Transition>
where
    Transition: Fn(&State, &Input) -> State,
    State: Clone,
{
    fn default() -> Self {
        StateMachineBuilder {
            initial_state: None,
            current_state: None,
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
        let sm = StateMachineBuilder::start()
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
            .build()
            .unwrap();

        assert_eq!(Stations::Sangendyaya, sm.consume(Train::Express));
    }
}
