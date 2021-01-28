use std::marker::PhantomData;

pub mod builder;

pub trait StateMachine<State, Input> {
    fn current_state(&self) -> State;
    fn consume(&mut self, input: Input) -> State;
    fn peek(&mut self, input: Input) -> State;
    fn reset(&mut self) -> State;
    fn set(&mut self, new_state: State);
}

pub struct BasicStateMachine<State, Input, Transition>
where
    Transition: FnMut(&State, &Input) -> State,
{
    pub initial_state: State,
    pub current_state: State,
    pub transition: Transition,
    pub _maker: PhantomData<Input>,
}

impl<State, Input, Transition> StateMachine<State, Input>
    for BasicStateMachine<State, Input, Transition>
where
    Transition: FnMut(&State, &Input) -> State,
    State: Copy,
{
    fn current_state(&self) -> State {
        self.current_state
    }

    fn consume(&mut self, input: Input) -> State {
        let mut new_state = self.current_state;
        new_state = (self.transition)(&new_state, &input);
        self.current_state = new_state;
        self.current_state
    }

    fn peek(&mut self, input: Input) -> State {
        (self.transition)(&self.current_state, &input)
    }

    fn reset(&mut self) -> State {
        self.current_state = self.initial_state;
        self.initial_state
    }

    fn set(&mut self, new_state: State) {
        self.current_state = new_state
    }
}
