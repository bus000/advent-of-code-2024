// --- Part Two ---
//
// As you scan through the corrupted memory, you notice that some of the
// conditional statements are also still intact. If you handle some of the
// uncorrupted conditional statements in the program, you might be able to get
// an even more accurate result.
//
// There are two new instructions you'll need to handle:
//
//    * The do() instruction enables future mul instructions.
//    * The don't() instruction disables future mul instructions.
//
// Only the most recent do() or don't() instruction applies. At the beginning of
// the program, mul instructions are enabled.
//
// For example:
//
//     xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
//
// This corrupted memory is similar to the example from before, but this time
// the mul(5,5) and mul(11,8) instructions are disabled because there is a
// don't() instruction before them. The other mul instructions function
// normally, including the one at the end that gets re-enabled by a do()
// instruction.
//
// This time, the sum of the results is 48 (2*4 + 8*5).
//
// Handle the new instructions; what do you get if you add up all of the results
// of just the enabled multiplications?
use std::io;
use advent_of_code_2024::FiniteAutomata;
use advent_of_code_2024::FiniteAutomataError;
use advent_of_code_2024::StateRef;

fn main() {
    let machine = Machine::build_machine()
        .expect("Unexpected machine building error.");
    println!("{:?}", machine.active);
}

/// State machine that will parse input and compute sum of all activated
/// multiplications.
struct Machine {

    /// The current state of the parser.
    parser_state: FiniteAutomata,

    /// Reference of the state we end up in after finding an 'm'.
    mul_end: StateRef,

    /// Reference of the state we end up in after finishing parsing a mul()
    /// expression.
    mul_start: StateRef,

    /// Reference of the state we are in while parsing 0-9 in first number of
    /// mul expression.
    n1_state: StateRef,

    /// Reference of the state we are in while parsing 0-9 in second number of
    /// mul expression.
    n2_state: StateRef,

    /// Reference of the state we end up in after finishing parsing a do()
    /// expression.
    do_end: StateRef,

    /// Reference of the state we end up in after finishing parsing a dont()
    /// expression.
    dont_end: StateRef,

    /// Whether or not the multiplication is currently active.
    active: bool,

    /// Running sum of active multiplications.
    sum_of_muls: u32,

    /// If parsing the first number of a multiplication it is buffered here.
    n1: u32,

    /// If parsing the second number of a multiplication it is buffered here.
    n2: u32

}

impl Machine {

    /// Build machine that can parse mul(), do(), and don't() functions.
    fn build_machine() -> Result<Machine, FiniteAutomataError> {
        let mut automata = FiniteAutomata::new();
        let initial = automata.current_state();
        let mul_1 = automata.add_state();
        let mul_2 = automata.add_state();
        let mul_3 = automata.add_state();
        let mul_4 = automata.add_state();
        let mul_5 = automata.add_state();
        let mul_6 = automata.add_state();

        automata.add_transition(mul_1, &|c| c == b'u', mul_2)?;
        automata.add_transition(mul_2, &|c| c == b'l', mul_3)?;
        automata.add_transition(mul_3, &|c| c == b'(', mul_4)?;
        automata.add_transition(mul_4, &|c| c >= b'0' && c <= b'9', mul_4)?;
        automata.add_transition(mul_4, &|c| c == b',', mul_5)?;
        automata.add_transition(mul_5, &|c| c >= b'0' && c <= b'9', mul_5)?;
        automata.add_transition(mul_5, &|c| c == b')', mul_6)?;

        let do_dont_1 = automata.add_state();
        let do_dont_2 = automata.add_state();
        let do_1 = automata.add_state();
        let do_2 = automata.add_state();
        let dont_1 = automata.add_state();
        let dont_2 = automata.add_state();
        let dont_3 = automata.add_state();
        let dont_4 = automata.add_state();
        let dont_5 = automata.add_state();

        automata.add_transition(do_dont_1, &|c| c == b'o', do_dont_2)?;
        automata.add_transition(do_dont_2, &|c| c == b'(', do_1)?;
        automata.add_transition(do_dont_2, &|c| c == b'n', dont_1)?;
        automata.add_transition(do_1, &|c| c == b')', do_2)?;
        automata.add_transition(dont_1, &|c| c == b'\'', dont_2)?;
        automata.add_transition(dont_2, &|c| c == b't', dont_3)?;
        automata.add_transition(dont_3, &|c| c == b'(', dont_4)?;
        automata.add_transition(dont_4, &|c| c == b')', dont_5)?;

        // All states can start a new mut(), do() or don't().
        automata.add_transition_all(&|c| c == b'm', mul_1)?;
        automata.add_transition_all(&|c| c == b'd', do_dont_1)?;

        // And all states can transition to the initial state on anything that
        // doesn't match any other rule.
        automata.add_transition_all(&|_| true, initial)?;

        let machine = Machine {
            parser_state: automata,
            mul_end: mul_6,
            mul_start: mul_1,
            n1_state: mul_4,
            n2_state: mul_5,
            do_end: do_2,
            dont_end: dont_5,

            active: true,
            sum_of_muls: 0,
            n1: 0,
            n2: 0,
        };
        return Ok(machine);
    }

    /// Transition the state of the machine based on the input byte.
    fn transition(&mut self, byte: u8) {
        self.parser_state.transition(byte)
            .expect("All states have a match rule going to the initial state.");
        let current_state = self.parser_state.current_state();
        if current_state == self.mul_end && self.active {
            self.sum_of_muls = self.sum_of_muls + self.n1 * self.n2;
        } else if current_state == self.mul_start {
            self.n1 = 0;
            self.n2 = 0;
        } else if current_state == self.do_end {
            self.active = true;
        } else if current_state == self.dont_end {
            self.active = false;
        } else if current_state == self.n1_state && byte != b'(' {
            let n = (byte - b'0') as u32;
            self.n1 = self.n1 * 10 + n;
        } else if current_state == self.n2_state && byte != b',' {
            let n = (byte - b'0') as u32;
            self.n2 = self.n2 * 10 + n;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that we can parse mul() expressions.
    #[test]
    pub fn test_build_machine_simple_mul() {
        let mut machine = Machine::build_machine()
            .expect("No machine building errors.");

        machine.transition(b'm');
        machine.transition(b'u');
        machine.transition(b'l');
        machine.transition(b'(');
        machine.transition(b'2');
        machine.transition(b',');
        machine.transition(b'5');
        machine.transition(b'0');
        machine.transition(b')');

        assert_eq!(machine.sum_of_muls, 100);
    }

    /// Test that we can parse mul() expressions that doesn't begin right away.
    #[test]
    pub fn test_build_machine_mul_starting_different_place() {
        let mut machine = Machine::build_machine()
            .expect("No machine building errors.");

        machine.transition(b'm');
        machine.transition(b'u');
        machine.transition(b'l');
        machine.transition(b'(');
        machine.transition(b'g');
        machine.transition(b'a');
        machine.transition(b'r');
        machine.transition(b'b');

        machine.transition(b'm');
        machine.transition(b'u');
        machine.transition(b'l');
        machine.transition(b'(');
        machine.transition(b'2');
        machine.transition(b',');
        machine.transition(b'5');
        machine.transition(b'0');
        machine.transition(b')');

        assert_eq!(machine.sum_of_muls, 100);
    }

    /// Test that we can parse do() and don't() expressions.
    #[test]
    pub fn test_build_machine_do_and_dont() {
        let mut machine = Machine::build_machine()
            .expect("No machine building errors.");

        machine.transition(b'm');
        assert_eq!(machine.active, true);
        machine.transition(b'u');
        assert_eq!(machine.active, true);
        machine.transition(b'l');
        assert_eq!(machine.active, true);
        machine.transition(b'(');
        assert_eq!(machine.active, true);
        machine.transition(b'd');
        assert_eq!(machine.active, true);
        machine.transition(b'o');
        assert_eq!(machine.active, true);
        machine.transition(b'n');
        assert_eq!(machine.active, true);
        machine.transition(b'\'');
        assert_eq!(machine.active, true);
        machine.transition(b't');
        assert_eq!(machine.active, true);
        machine.transition(b'(');
        assert_eq!(machine.active, true);
        machine.transition(b')');
        assert_eq!(machine.active, false);
        machine.transition(b'd');
        assert_eq!(machine.active, false);
        machine.transition(b'o');
        assert_eq!(machine.active, false);
        machine.transition(b'(');
        assert_eq!(machine.active, false);
        machine.transition(b'd');
        assert_eq!(machine.active, false);
        machine.transition(b'o');
        assert_eq!(machine.active, false);
        machine.transition(b'(');
        assert_eq!(machine.active, false);
        machine.transition(b')');
        assert_eq!(machine.active, true);
    }

    /// Test that a larger example computes the correct sum.
    #[test]
    pub fn test_build_machine_large_example() {
        let mut machine = Machine::build_machine()
            .expect("No machine building errors.");
        let input = concat![
            "asdfamul(mul(2,4)dont()mul(3,1)asdfdon't()mul(100,100)dafdodo(do(",
            ")mul(7,7)asfd"
        ];

        for byte in input.bytes() {
            machine.transition(byte);
        }

        assert_eq!(machine.sum_of_muls, 60);
    }

}
