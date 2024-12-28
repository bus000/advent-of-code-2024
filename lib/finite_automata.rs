use std::collections::HashMap;

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
        p: &'static dyn Fn(u8) -> bool, resulting_state_ref: StateRef)
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

    /// Find the state with the given state_ref.
    pub fn find_state(&self, state_ref: StateRef) -> Option<&State> {
        return self.states.get(state_ref);
    }

    /// Find the state with the given state_ref.
    fn find_state_mut(&mut self, state_ref: StateRef) -> Option<&mut State> {
        return self.states.get_mut(state_ref);
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
        let state = self.find_state(self.current_state)
            .expect("The current state can never refer to a missing state.");
        for transition in state.transitions.iter() {
            if (transition.predicate)(byte) {
                self.current_state = transition.result_state_ref;
                return Ok(());
            }
        }

        return Err(FiniteAutomataError::MissingTransition);
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
    pub fn add_transition(&mut self, p: &'static dyn Fn(u8) -> bool,
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
    predicate : &'static dyn Fn(u8) -> bool,

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

    // /// Test that adding transition bl
    //pub fn add_transition(&mut self, state_ref : &str,
        //p: &'static dyn Fn(u8) -> bool, resulting_state_ref: &'static str)

}
