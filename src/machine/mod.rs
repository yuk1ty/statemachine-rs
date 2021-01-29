use std::marker::PhantomData;

pub mod builder;
pub mod error;

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
    State: Clone,
{
    fn current_state(&self) -> State {
        self.current_state.clone()
    }

    fn consume(&mut self, input: Input) -> State {
        let mut new_state = self.current_state.clone();
        new_state = (self.transition)(&new_state, &input);
        self.current_state = new_state;
        self.current_state()
    }

    fn peek(&mut self, input: Input) -> State {
        (self.transition)(&self.current_state, &input)
    }

    fn reset(&mut self) -> State {
        self.current_state = self.initial_state.clone();
        self.current_state()
    }

    fn set(&mut self, new_state: State) {
        self.current_state = new_state
    }
}

#[cfg(test)]
mod test {
    use std::marker::PhantomData;

    use super::BasicStateMachine;
    use super::StateMachine;

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
            current_state: Stations::Shibuya,
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
        let mut sm = BasicStateMachine {
            initial_state: Stations::Shibuya,
            current_state: Stations::Shibuya,
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
        let mut sm = BasicStateMachine {
            initial_state: Stations::Sangendyaya,
            current_state: Stations::Sangendyaya,
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
        let mut sm = BasicStateMachine {
            initial_state: Stations::Shibuya,
            current_state: Stations::Sangendyaya,
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
        let mut sm = BasicStateMachine {
            initial_state: Stations::Shibuya,
            current_state: Stations::Shibuya,
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
