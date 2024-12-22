use std::collections::HashMap;

/// A finite automata that can be used for string matching.
///
/// Contains a set of legal states with transitions on them. Contains a
/// reference to the current state. A function FiniteAutomata.transition is used
/// to transition between states.
struct FiniteAutomata<'a> {

    /// Reference of the current state.
    current_state : &'a str,

    /// All states.
    states: HashMap<&'a str, State<'a>>

}

impl FiniteAutomata<'_> {

    /// Create a new finite automata.
    ///
    /// Will set the current_state as the given initial_state.
    pub fn new<'a>(initial_state: &'a str) -> FiniteAutomata<'a> {
        let mut states = HashMap::new();
        states.insert(initial_state, State::new(initial_state));
        return FiniteAutomata {
            current_state: initial_state,
            states: states
        };
    }

    /// Add a state to a finite automata.
    ///
    /// # Errors
    ///
    /// Will give error DuplicateState if given a state reference that already
    /// exist.
    pub fn add_state<'a>(&'a mut self, state_ref: &'a str)
        -> Result<(), FiniteAutomataError> {

        if self.states.contains_key(state_ref) {
            return Err(FiniteAutomataError::DuplicateState);
        }

        let state = State::new(state_ref);
        self.states.insert(state_ref, state);
        return Ok(());
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
    pub fn add_transition(&mut self, state_ref : &str, p: &dyn Fn(u8) -> bool,
        resulting_state_ref: &str) -> Result<(), FiniteAutomataError> {

        if !self.states.contains_key(resulting_state_ref) {
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
    pub fn find_state(&self, state_ref: &str) -> Option<&State> {
        return self.states.get(state_ref);
    }

    /// Find the state with the given state_ref.
    fn find_state_mut(&mut self, state_ref: &str) -> Option<&mut State> {
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
struct State<'a> {

    /// Unique name of the state.
    state_ref: &'a str,

    /// Transitions to other states.
    transitions: Vec<Transition<'a>>

}

impl State<'_> {

    /// Construct a new state with the given name.
    pub fn new<'a>(state_ref : &str) -> State {
        return State {
            state_ref: state_ref,
            transitions: Vec::new()
        };
    }

    /// Add another transition to this state.
    ///
    /// The transition is added to the end of the list of transitions and will
    /// only be tried if all other transitions fail matching.
    pub fn add_transition(&mut self, p: &dyn Fn(u8) -> bool,
        result_state_ref: &str) {

        let transition = Transition {
            predicate: Box::new(p),
            result_state_ref: result_state_ref
        };
        self.transitions.push(transition);
    }

}

/// A transition is a predicate matching a byte and a result state if matching.
struct Transition<'a> {

    /// Predicate matching whether the transition should be taken.
    predicate : Box<dyn Fn(u8) -> bool>,

    /// The state to transition to on success.
    result_state_ref : &'a str

}

/// Errors returned by finite automatas.
enum FiniteAutomataError {

    /// Error given when trying to add a state to an automata that already
    /// exist.
    DuplicateState,

    /// Error given when referencing a state that does not exist.
    MissingState,

    /// Error given when given a byte that doesn't match any transition in the
    /// current state.
    MissingTransition

}
