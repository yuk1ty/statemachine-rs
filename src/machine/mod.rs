use std::{cell::RefCell, marker::PhantomData};

pub mod builder;
pub mod error;

/// The trait provides several methos to implement state machine.
/// `BasicStateMachine` is good example to implement it.
/// Of course, you can build your own state machine by using this trait.
pub trait StateMachine<State, Input> {
    fn current_state(&self) -> State;
    fn consume(&self, input: Input) -> State;
    fn peek(&self, input: Input) -> State;
    fn reset(&self) -> State;
    fn set(&self, new_state: State);
}

pub(crate) struct StateWrapper<State: Clone>(State);

impl<State> StateWrapper<State>
where
    State: Clone,
{
    pub fn new(state: State) -> Self {
        StateWrapper(state)
    }

    pub fn get(&self) -> State {
        self.0.clone()
    }

    pub fn set(&mut self, state: State) {
        self.0 = state;
    }
}

/// The basic state machine implementation.
/// It holds `initial_state`, `current_state`, `transition` function.
pub struct BasicStateMachine<State, Input, Transition>
where
    Transition: Fn(&State, &Input) -> State,
    State: Clone,
{
    /// `initial_state` is literary initial state of state machine.
    /// The field doesn't update the whole life of the state machine.
    /// That is, it always returns the initial state of the machine.
    initial_state: State,
    /// `current_state` is the current state of the state machine.
    /// It transit to the next state via `transition`.
    current_state: RefCell<StateWrapper<State>>,
    /// `transition` is the definition of state transition.
    /// See an example of StateMachine::transit, you can grasp how
    /// to define the transition.
    transition: Transition,
    _maker: PhantomData<Input>,
}

impl<State, Input, Transition> StateMachine<State, Input>
    for BasicStateMachine<State, Input, Transition>
where
    Transition: Fn(&State, &Input) -> State,
    State: Clone,
{
    fn current_state(&self) -> State {
        self.current_state.borrow().get()
    }

    fn consume(&self, input: Input) -> State {
        let new_state = (self.transition)(&self.current_state.borrow().0, &input);
        self.current_state.borrow_mut().set(new_state);
        self.current_state()
    }

    fn peek(&self, input: Input) -> State {
        (self.transition)(&self.current_state.borrow().0, &input)
    }

    fn reset(&self) -> State {
        self.current_state
            .borrow_mut()
            .set(self.initial_state.clone());
        self.current_state()
    }

    fn set(&self, new_state: State) {
        self.current_state.borrow_mut().set(new_state)
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, marker::PhantomData};

    use super::StateMachine;
    use super::{BasicStateMachine, StateWrapper};

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

    #[test]
    fn test_current_state() {
        let sm = BasicStateMachine {
            initial_state: Stations::Shibuya,
            current_state: RefCell::new(StateWrapper::new(Stations::Shibuya)),
            transition: |station, train| match (station, train) {
                (Stations::Shibuya, Train::Local) => Stations::IkejiriOhashi,
                _ => unreachable!(),
            },
            _maker: PhantomData::<Train>::default(),
        };

        assert_eq!(Stations::Shibuya, sm.current_state());
    }

    #[test]
    fn test_consume() {
        let sm = BasicStateMachine {
            initial_state: Stations::Shibuya,
            current_state: RefCell::new(StateWrapper::new(Stations::Shibuya)),
            transition: |station, train| match (station, train) {
                (Stations::Shibuya, Train::Local) => Stations::IkejiriOhashi,
                (Stations::Shibuya, Train::Express) => Stations::Sangendyaya,
                (Stations::IkejiriOhashi, Train::Local) => Stations::Sangendyaya,
                (Stations::Sangendyaya, Train::Local) => Stations::KomazawaDaigaku,
                (Stations::Sangendyaya, Train::Express) => Stations::FutakoTamagawa,
                (Stations::KomazawaDaigaku, Train::Local) => Stations::Sakurashinmachi,
                (Stations::Sakurashinmachi, Train::Local) => Stations::Yoga,
                _ => unreachable!(),
            },
            _maker: PhantomData::<Train>::default(),
        };

        assert_eq!(Stations::IkejiriOhashi, sm.consume(Train::Local));
    }

    #[test]
    fn test_peek() {
        let sm = BasicStateMachine {
            initial_state: Stations::Sangendyaya,
            current_state: RefCell::new(StateWrapper::new(Stations::Sangendyaya)),
            transition: |station, train| match (station, train) {
                (Stations::Shibuya, Train::Local) => Stations::IkejiriOhashi,
                (Stations::Shibuya, Train::Express) => Stations::Sangendyaya,
                (Stations::IkejiriOhashi, Train::Local) => Stations::Sangendyaya,
                (Stations::Sangendyaya, Train::Local) => Stations::KomazawaDaigaku,
                (Stations::Sangendyaya, Train::Express) => Stations::FutakoTamagawa,
                (Stations::KomazawaDaigaku, Train::Local) => Stations::Sakurashinmachi,
                (Stations::Sakurashinmachi, Train::Local) => Stations::Yoga,
                _ => unreachable!(),
            },
            _maker: PhantomData::<Train>::default(),
        };

        assert_eq!(Stations::FutakoTamagawa, sm.peek(Train::Express));
        assert_eq!(Stations::Sangendyaya, sm.current_state());
    }

    #[test]
    fn test_reset() {
        let sm = BasicStateMachine {
            initial_state: Stations::Shibuya,
            current_state: RefCell::new(StateWrapper::new(Stations::Sangendyaya)),
            transition: |station, train| match (station, train) {
                (Stations::Shibuya, Train::Local) => Stations::IkejiriOhashi,
                (Stations::Shibuya, Train::Express) => Stations::Sangendyaya,
                (Stations::IkejiriOhashi, Train::Local) => Stations::Sangendyaya,
                (Stations::Sangendyaya, Train::Local) => Stations::KomazawaDaigaku,
                (Stations::Sangendyaya, Train::Express) => Stations::FutakoTamagawa,
                (Stations::KomazawaDaigaku, Train::Local) => Stations::Sakurashinmachi,
                (Stations::Sakurashinmachi, Train::Local) => Stations::Yoga,
                _ => unreachable!(),
            },
            _maker: PhantomData::<Train>::default(),
        };

        assert_eq!(Stations::FutakoTamagawa, sm.consume(Train::Express));
        assert_eq!(Stations::Shibuya, sm.reset());
    }

    #[test]
    fn test_set() {
        let sm = BasicStateMachine {
            initial_state: Stations::Shibuya,
            current_state: RefCell::new(StateWrapper::new(Stations::Shibuya)),
            transition: |station, train| match (station, train) {
                (Stations::Shibuya, Train::Local) => Stations::IkejiriOhashi,
                _ => unreachable!(),
            },
            _maker: PhantomData::<Train>::default(),
        };

        assert_eq!(Stations::Shibuya, sm.current_state());
        sm.set(Stations::Yoga);
        assert_eq!(Stations::Yoga, sm.current_state())
    }
}
