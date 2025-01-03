/// A reference to a state in the finite state machine.
type StateRef = usize;

/// A finite automata that can be used for string matching.
///
/// Contains a set of legal states with transitions on them. Contains a
/// reference to the current state. A function FiniteAutomata.transition is used
/// to transition between states.
struct FiniteAutomata {

    /// Reference of the current state.
    current_state : StateRef,

    /// All states.
    states: Vec<State>,

}

impl FiniteAutomata {

    /// Create a new finite automata.
    ///
    /// Will set the current_state as the given initial_state.
    pub fn new() -> FiniteAutomata {
        let mut states = Vec::new();
        states.push(State::new(0));
        return FiniteAutomata {
            current_state: 0,
            states: states
        };
    }

    /// Add a state to a finite automata.
    pub fn add_state(&mut self) -> StateRef {
        let state_ref = self.states.len();
        let state = State::new(state_ref);
        self.states.push(state);
        return state_ref;
    }

    /// Add a transition to a state.
    ///
    /// Will add a transition to state resulting_state_ref from state state_ref
    /// if the predicate p matches a byte. The transition is added at the end of
    /// the list of transitions on the state_ref state. If another transition is
    /// matching before this one it will not be taken.
    ///
    /// # Errors
    ///
    /// Will give error FiniteAutomataError.MissingState if either state_ref or
    /// resulting_state_ref could not be found on the finite automata.
    pub fn add_transition(&mut self, state_ref : StateRef,
        p: Box<dyn Fn(u8) -> bool + 'static>, resulting_state_ref: StateRef)
        -> Result<(), FiniteAutomataError> {

        let state_count = self.states.len();
        if state_ref >= state_count || resulting_state_ref >= state_count {
            return Err(FiniteAutomataError::MissingState);
        }
        return match self.find_state_mut(state_ref) {
            None => {
                Err(FiniteAutomataError::MissingState)
            },
            Some(state) => {
                state.add_transition(p, resulting_state_ref);
                Ok(())
            }
        }
    }

    /// Transition from the current state on the given byte.
    ///
    /// Will apply first matching transition to enter a new state.
    ///
    /// # Errors
    ///
    /// Will give error FiniteAutomataError.MissingTransition if no transition
    /// matches the given byte.
    pub fn transition(&mut self, byte: u8) -> Result<(), FiniteAutomataError> {
        let state = self.states.get(self.current_state)
            .expect("The current state can never refer to a missing state.");
        for transition in state.transitions.iter() {
            if (transition.predicate)(byte) {
                self.current_state = transition.result_state_ref;
                return Ok(());
            }
        }

        return Err(FiniteAutomataError::MissingTransition);
    }

    /// Find the state with the given state_ref.
    fn find_state(&self, state_ref: StateRef) -> Option<&State> {
        return self.states.get(state_ref);
    }

    /// Find the state with the given state_ref.
    fn find_state_mut(&mut self, state_ref: StateRef)
        -> Option<&mut State> {

        return self.states.get_mut(state_ref);
    }

}

/// A state has a reference and a list of transitions to other states.
struct State {

    /// Unique name of the state.
    state_ref: StateRef,

    /// Transitions to other states.
    transitions: Vec<Transition>

}

impl State {

    /// Construct a new state with the given name.
    pub fn new(state_ref: StateRef) -> State {
        return State {
            state_ref: state_ref,
            transitions: Vec::new()
        };
    }

    /// Add another transition to this state.
    ///
    /// The transition is added to the end of the list of transitions and will
    /// only be tried if all other transitions fail matching.
    pub fn add_transition(&mut self, p: Box<dyn Fn(u8) -> bool + 'static>,
        result_state_ref: StateRef) {

        let transition = Transition {
            predicate: p,
            result_state_ref: result_state_ref
        };
        self.transitions.push(transition);
    }

}

/// A transition is a predicate matching a byte and a result state if matching.
struct Transition {

    /// Predicate matching whether the transition should be taken.
    predicate : Box<dyn Fn(u8) -> bool + 'static>,

    /// The state to transition to on success.
    result_state_ref : StateRef

}

/// Errors returned by finite automatas.
#[derive(Debug, PartialEq)]
enum FiniteAutomataError {

    /// Error given when referencing a state that does not exist.
    MissingState,

    /// Error given when given a byte that doesn't match any transition in the
    /// current state.
    MissingTransition

}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that adding a state works as expected.
    #[test]
    fn test_add_state() {
        let mut automata = FiniteAutomata::new();
        let state_1 = automata.add_state();
        let state_2 = automata.add_state();
        assert_eq!(0, automata.current_state);
        assert_eq!(1, state_1);
        assert_eq!(2, state_2);
        assert_eq!(3, automata.states.len());
    }

    /// Test that adding state transitions works as expected.
    #[test]
    pub fn test_add_transition() {
        let mut automata = FiniteAutomata::new();
        let state_1 = automata.current_state;
        let state_2 = automata.add_state();
        let predicate = Box::new(&|c| c == 0x78);
        automata.add_transition(state_1, predicate, state_2)
            .expect("Both states exist, this should be fine.");

        assert_eq!(0, automata.current_state);
        assert_eq!(1, automata.states[0].transitions.len());
        assert_eq!(0, automata.states[1].transitions.len());
    }

    /// Test that adding state transitions to states that doesn't exist will
    /// give an error.
    #[test]
    pub fn test_add_transition_to_missing_state() {
        let mut automata = FiniteAutomata::new();
        let state_1 = automata.current_state;
        automata.add_state();
        let predicate = Box::new(&|c| c == 0x78);
        let error = automata.add_transition(state_1, predicate, 44);

        assert_eq!(Err(FiniteAutomataError::MissingState), error);
    }

    /// Test that adding state transitions from states that doesn't exist will
    /// give an error.
    #[test]
    pub fn test_add_transition_from_missing_state() {
        let mut automata = FiniteAutomata::new();
        automata.current_state;
        let state_2 = automata.add_state();
        let predicate = Box::new(&|c| c == 0x78);
        let error = automata.add_transition(44, predicate, state_2);

        assert_eq!(Err(FiniteAutomataError::MissingState), error);
    }

    /// Test that transitioning on a matching byte will switch state.
    #[test]
    pub fn test_transition() {
        let mut automata = FiniteAutomata::new();
        let state_1 = automata.current_state;
        let state_2 = automata.add_state();
        let predicate = Box::new(&|c| c == 0x78);
        automata.add_transition(state_1, predicate, state_2)
            .expect("Both states exist, this should be fine.");

        assert_eq!(state_1, automata.current_state);
        automata.transition(b'x').expect("We have a rule matching 'x'.");
        assert_eq!(state_2, automata.current_state);
    }

    /// Test that transitioning on a non matching byte will error.
    #[test]
    pub fn test_transition_missing() {
        let mut automata = FiniteAutomata::new();
        let state_1 = automata.current_state;
        let state_2 = automata.add_state();
        let predicate = Box::new(&|c| c == 0x78);
        automata.add_transition(state_1, predicate, state_2)
            .expect("Both states exist, this should be fine.");

        assert_eq!(state_1, automata.current_state);
        let error = automata.transition(b'y');
        assert_eq!(error, Err(FiniteAutomataError::MissingTransition));
    }

    /// Test that the first matching transition is used.
    #[test]
    pub fn test_transition_first_matching_rule_is_used() {
        let mut automata = FiniteAutomata::new();
        let state_1 = automata.current_state;
        let state_2 = automata.add_state();
        let state_3 = automata.add_state();

        automata.add_transition(state_1, Box::new(&|c| c == 0x78), state_2)
            .expect("Both states exist, this should be fine.");
        automata.add_transition(state_1, Box::new(&|c| true), state_3)
            .expect("Both states exist, this should be fine.");
        automata.add_transition(state_2, Box::new(&|c| true), state_1)
            .expect("Both states exist, this should be fine.");

        assert_eq!(state_1, automata.current_state);
        automata.transition(b'x').expect("We have a rule matching 'x'.");
        assert_eq!(state_2, automata.current_state);
        automata.transition(b'x').expect("We have a rule matching 'x'.");
        assert_eq!(state_1, automata.current_state);
        automata.transition(b'y').expect("We have a rule matching 'y'.");
        assert_eq!(state_3, automata.current_state);
    }

}
