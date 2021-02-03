use std::{cell::RefCell, marker::PhantomData};

pub mod builder;
pub mod error;

/// The trait is representing the basic operation for the state machine.
/// It includes getting its current state, transition to the next state,
/// resetting its current state to initial state and setting particular state forcibly.
/// [`BasicStateMachine`] is a good example to implement it.
/// Of course, you can build your own state machine by using this trait.
pub trait StateMachine<State, Input> {
    /// Returns the current state of the state machine.
    ///
    /// # Example
    /// ```
    /// use statemachine_rs::machine::{
    ///     builder::BasicStateMachineBuilder, builder::StateMachineBuilder, StateMachine,
    /// };
    ///
    /// #[derive(Clone, Debug, PartialEq)]
    /// enum ButtonState {
    ///     On,
    ///     Off,
    /// }
    ///
    /// #[allow(dead_code)]
    /// enum Input {
    ///     Press,
    /// }
    ///
    /// let sm = BasicStateMachineBuilder::start()
    ///     .initial_state(ButtonState::Off)
    ///     .transition(|state, input| match (state, input) {
    ///         (ButtonState::On, Input::Press) => ButtonState::Off,
    ///         (ButtonState::Off, Input::Press) => ButtonState::On,
    ///     })
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(ButtonState::Off, sm.current_state());
    /// ```
    fn current_state(&self) -> State;
    /// Returns the result of state transition according to `input` and
    /// the definition of transition function.
    ///
    /// # Example
    /// ```
    /// use statemachine_rs::machine::{
    ///     builder::BasicStateMachineBuilder, builder::StateMachineBuilder, StateMachine,
    /// };
    ///
    /// #[derive(Clone, Debug, PartialEq)]
    /// enum ButtonState {
    ///     On,
    ///     Off,
    /// }
    ///
    /// enum Input {
    ///     Press,
    /// }
    ///
    /// let sm = BasicStateMachineBuilder::start()
    ///     .initial_state(ButtonState::Off)
    ///     .transition(|state, input| match (state, input) {
    ///         (ButtonState::On, Input::Press) => ButtonState::Off,
    ///         (ButtonState::Off, Input::Press) => ButtonState::On,
    ///     })
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(ButtonState::Off, sm.current_state());
    /// assert_eq!(ButtonState::On, sm.consume(Input::Press));
    /// ```
    fn consume(&self, input: Input) -> State;
    /// Returns the next state from the current state but the state machine
    /// retains in its current state.
    ///
    /// # Example
    /// ```
    /// use statemachine_rs::machine::{
    ///     builder::BasicStateMachineBuilder, builder::StateMachineBuilder, StateMachine,
    /// };
    ///
    /// #[derive(Clone, Debug, PartialEq)]
    /// enum ButtonState {
    ///     On,
    ///     Off,
    /// }
    ///
    /// enum Input {
    ///     Press,
    /// }
    ///
    /// let sm = BasicStateMachineBuilder::start()
    ///     .initial_state(ButtonState::Off)
    ///     .transition(|state, input| match (state, input) {
    ///         (ButtonState::On, Input::Press) => ButtonState::Off,
    ///         (ButtonState::Off, Input::Press) => ButtonState::On,
    ///     })
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(ButtonState::Off, sm.current_state());
    /// assert_eq!(ButtonState::On, sm.peek(Input::Press));
    /// assert_eq!(ButtonState::Off, sm.current_state());
    /// ```
    fn peek(&self, input: Input) -> State;
    /// Resets the current state to the initial state.
    ///
    /// # Example
    /// ```
    /// use statemachine_rs::machine::{
    ///     builder::BasicStateMachineBuilder, builder::StateMachineBuilder, StateMachine,
    /// };
    ///
    /// #[derive(Clone, Debug, PartialEq)]
    /// enum ButtonState {
    ///     On,
    ///     Off,
    /// }
    ///
    /// enum Input {
    ///     Press,
    /// }
    ///
    /// let sm = BasicStateMachineBuilder::start()
    ///     .initial_state(ButtonState::Off)
    ///     .transition(|state, input| match (state, input) {
    ///         (ButtonState::On, Input::Press) => ButtonState::Off,
    ///         (ButtonState::Off, Input::Press) => ButtonState::On,
    ///     })
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(ButtonState::Off, sm.current_state());
    /// assert_eq!(ButtonState::On, sm.consume(Input::Press));
    /// assert_eq!(ButtonState::Off, sm.reset());
    /// ```
    fn reset(&self) -> State;
    /// Set a new state forcibly to the current state.
    ///
    /// # Example
    /// ```
    /// use statemachine_rs::machine::{
    ///     builder::BasicStateMachineBuilder, builder::StateMachineBuilder, StateMachine,
    /// };
    ///
    /// #[derive(Clone, Debug, PartialEq)]
    /// enum ButtonState {
    ///     On,
    ///     Off,
    ///     Disable,
    /// }
    ///
    /// enum Input {
    ///     Press,
    /// }
    ///
    /// let sm = BasicStateMachineBuilder::start()
    ///     .initial_state(ButtonState::Off)
    ///     .transition(|state, input| match (state, input) {
    ///         (ButtonState::On, Input::Press) => ButtonState::Off,
    ///         (ButtonState::Off, Input::Press) => ButtonState::On,
    ///         (ButtonState::Disable, Input::Press) => ButtonState::Disable,
    ///     })
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(ButtonState::Off, sm.current_state());
    /// sm.set(ButtonState::Disable);
    /// assert_eq!(ButtonState::Disable, sm.consume(Input::Press));
    /// ```
    fn set(&self, new_state: State);
}

/// [`StateWrapper`] is a struct for interior mutability.
/// It enables to acquire the control of switching mutable/imutable
/// with [`std::cell::RefCell`].
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
    Transition: Fn(&State, Input) -> State,
    State: Clone,
{
    /// `initial_state` is literally an initial state of the state machine.
    /// The field isn't updated the whole life of its state machine.
    /// That is, it always returns its initial state of its machine.
    initial_state: State,
    /// `current_state` is the current state of the state machine.
    /// It transit to the next state via `transition`.
    current_state: RefCell<StateWrapper<State>>,
    /// `transition` is the definition of state transition.
    /// See an example of [`StateMachine::consume()`], you can grasp how
    /// to define the transition.
    transition: Transition,
    _maker: PhantomData<Input>,
}

impl<State, Input, Transition> StateMachine<State, Input>
    for BasicStateMachine<State, Input, Transition>
where
    Transition: Fn(&State, Input) -> State,
    State: Clone,
{
    fn current_state(&self) -> State {
        self.current_state.borrow().get()
    }

    fn consume(&self, input: Input) -> State {
        let new_state = (self.transition)(&self.current_state.borrow().0, input);
        self.current_state.borrow_mut().set(new_state);
        self.current_state()
    }

    fn peek(&self, input: Input) -> State {
        (self.transition)(&self.current_state.borrow().0, input)
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
