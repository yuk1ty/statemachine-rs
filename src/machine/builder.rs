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
