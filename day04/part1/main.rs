// --- Day 4: Ceres Search ---
//
// "Looks like the Chief's not here. Next!" One of The Historians pulls out a
// device and pushes the only button on it. After a brief flash, you recognize
// the interior of the Ceres monitoring station!
//
// As the search for the Chief continues, a small Elf who lives on the station
// tugs on your shirt; she'd like to know if you could help her with her word
// search (your puzzle input). She only has to find one word: XMAS.
//
// This word search allows words to be horizontal, vertical, diagonal, written
// backwards, or even overlapping other words. It's a little unusual, though, as
// you don't merely need to find one instance of XMAS - you need to find all of
// them. Here are a few ways XMAS might appear, where irrelevant characters have
// been replaced with .:
//
//    ..X...
//    .SAMX.
//    .A..A.
//    XMAS.S
//    .X....
//
// The actual word search will be full of letters instead. For example:
//
//    MMMSXXMASM
//    MSAMXMSMSA
//    AMXSXMAAMM
//    MSAMASMSMX
//    XMASAMXAMM
//    XXAMMXXAMA
//    SMSMSASXSS
//    SAXAMASAAA
//    MAMMMXMMMM
//    MXMXAXMASX
//
// In this word search, XMAS occurs a total of 18 times; here's the same word
// search again, but where letters not involved in any XMAS have been replaced
// with .:
//
//    ....XXMAS.
//    .SAMXMS...
//    ...S..A...
//    ..A.A.MS.X
//    XMASAMX.MM
//    X.....XA.A
//    S.S.S.S.SS
//    .A.A.A.A.A
//    ..M.M.M.MM
//    .X.X.XMASX
//
// Take a look at the little Elf's word search. How many times does XMAS appear?
use std::io;
use std::process::ExitCode;
use aoc2024::finite_automata::FiniteAutomata;
use aoc2024::finite_automata::FiniteAutomataError;
use aoc2024::finite_automata::StateRef;

fn main() -> ExitCode {
    match read_input() {
        Ok(input) => {
            println!("{:?}", count_xmas(input));
        },
        Err(err) => {
            eprintln!("Could not read input: {:?}", err);
            return ExitCode::from(1);
        }
    }

    return ExitCode::from(0);
}

/// Count instances of the word "XMAS" appearing horizontally, vertically or
/// diagonally either forwards or backwards in the given 2D array.
fn count_xmas(array: Vec<Vec<u8>>) -> u32 {
    let rows = array.len();
    if rows == 0 {
        return 0;
    }
    let columns = array[0].len();
    let diagonals_half = rows + columns - 1;
    let diagonals = diagonals_half * 2;

    let size = rows + columns + diagonals;
    let mut machines = Vec::with_capacity(size);
    for _ in 0..size {
        machines.push(Machine::build_machine());
    }

    for i in 0..array.len() {
        for j in 0..array[i].len() {
            let row = &mut machines[i];
            row.transition(array[i][j]);

            let column = &mut machines[rows + j];
            column.transition(array[i][j]);

            // Diagonals going from left to right can be uniquely identified by
            // their sum of indices. Try it on paper.
            let diag_1 = &mut machines[rows + columns + i + j];
            diag_1.transition(array[i][j]);

            // Diagonals going from right to left can be identified by
            // subtracting the 2 indices. We add (columns - 1) make indices
            // start at 0.
            let index = columns + i - j - 1;
            let diag_2 = &mut machines[rows + columns + diagonals_half + index];
            diag_2.transition(array[i][j]);
        }
    }

    let mut sum = 0;
    for machine in machines {
        sum = sum + machine.count;
    }
    return sum;
}

/// Read a 2D array of bytes from stdin in full.
fn read_input() -> Result<Vec<Vec<u8>>, io::Error> {
    let mut array = vec![];

    for line in io::stdin().lines() {
        match line {
            Ok(string) => {
                array.push(string.bytes().collect());
            },
            Err(err) => {
                return Err(err);
            }
        }
    }

    return Ok(array);
}

/// State machine that will parse XMAS and SAMX and count occurences.
struct Machine {

    /// The current state of the parser.
    parser_state: FiniteAutomata,

    /// Reference of the state we end up in after reading 'XMAS'.
    xmas_end: StateRef,

    /// Reference of the state we end up in after reading 'SAMX'.
    samx_end: StateRef,

    /// Current count of 'XMAS' and 'SAMX' found.
    count: u32,

}

impl Machine {

    /// Build machine that can find 'XMAS' and 'SAMX'.
    fn build_machine() -> Machine {
        Machine::build_machine_helper()
            .expect("Unexpected machine building error.")
    }

    /// Build machine that can find 'XMAS' and 'SAMX'.
    fn build_machine_helper() -> Result<Machine, FiniteAutomataError> {
        let mut automata = FiniteAutomata::new();
        let initial = automata.current_state();
        let xmas1 = automata.add_state();
        let xmas2 = automata.add_state();
        let xmas3 = automata.add_state();
        let xmas4 = automata.add_state();
        let samx1 = automata.add_state();
        let samx2 = automata.add_state();
        let samx3 = automata.add_state();
        let samx4 = automata.add_state();

        automata.add_transition(xmas1, &|c| c == b'M', xmas2)?;
        automata.add_transition(xmas2, &|c| c == b'A', xmas3)?;
        automata.add_transition(xmas3, &|c| c == b'S', xmas4)?;
        automata.add_transition(xmas4, &|c| c == b'A', samx2)?;

        automata.add_transition(samx1, &|c| c == b'A', samx2)?;
        automata.add_transition(samx2, &|c| c == b'M', samx3)?;
        automata.add_transition(samx3, &|c| c == b'X', samx4)?;
        automata.add_transition(samx4, &|c| c == b'M', xmas2)?;

        // All states can start a new 'XMAS' or 'SAMX'.
        automata.add_transition_all(&|c| c == b'X', xmas1)?;
        automata.add_transition_all(&|c| c == b'S', samx1)?;

        // And all states can transition to the initial state on anything that
        // doesn't match any other rule.
        automata.add_transition_all(&|_| true, initial)?;

        let machine = Machine {
            parser_state: automata,
            xmas_end: xmas4,
            samx_end: samx4,
            count: 0
        };
        return Ok(machine);
    }

    /// Transition the state of the machine based on the input byte.
    fn transition(&mut self, byte: u8) {
        self.parser_state.transition(byte)
            .expect("All states have a match rule going to the initial state.");
        let current_state = self.parser_state.current_state();
        if current_state == self.xmas_end || current_state == self.samx_end {
            self.count = self.count + 1;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that counting horizontal XMAS works.
    #[test]
    fn test_count_xmas_horizontal() {
        let input = vec![
            "..XMAS".bytes().collect(),
            "......".bytes().collect(),
            ".A..A.".bytes().collect(),
            "X.AS.S".bytes().collect(),
            ".X....".bytes().collect()
        ];
        let xmass = count_xmas(input);

        assert_eq!(xmass, 1);
    }

    /// Test that counting double horizontal XMAS + SAMX works.
    #[test]
    fn test_count_xmas_double_horizontal() {
        let input = vec![
            "..XMASAMX".bytes().collect(),
            ".........".bytes().collect(),
            "X.AS.S...".bytes().collect(),
            "X.AS.S...".bytes().collect(),
            "X.AS.S...".bytes().collect(),
            "X.AS.S...".bytes().collect(),
            "X.AS.S...".bytes().collect(),
            "X.AS.S...".bytes().collect(),
            ".X.......".bytes().collect()
        ];
        let xmass = count_xmas(input);

        assert_eq!(xmass, 2);
    }

    /// Test that counting horizontal SAMX works.
    #[test]
    fn test_count_samx_horizontal() {
        let input = vec![
            "..SAMX".bytes().collect(),
            "......".bytes().collect(),
            ".A..A.".bytes().collect(),
            "X.AS.S".bytes().collect(),
            ".X....".bytes().collect()
        ];
        let xmass = count_xmas(input);

        assert_eq!(xmass, 1);
    }

    /// Test that counting vertical SAMX works.
    #[test]
    fn test_count_samx_vertical() {
        let input = vec![
            "..XS.S".bytes().collect(),
            "...A..".bytes().collect(),
            ".A.MA.".bytes().collect(),
            "X.AX.S".bytes().collect(),
            ".X....".bytes().collect()
        ];
        let xmass = count_xmas(input);

        assert_eq!(xmass, 1);
    }

    /// Test that counting diagonal down XMAS works.
    #[test]
    fn test_count_xmas_diagonal_down() {
        let input = vec![
            "..XS.S".bytes().collect(),
            "...M..".bytes().collect(),
            ".A.MA.".bytes().collect(),
            "X.AX.S".bytes().collect(),
            ".X....".bytes().collect()
        ];
        let xmass = count_xmas(input);

        assert_eq!(xmass, 1);
    }

    /// Test that counting diagonal up SAMX works.
    #[test]
    fn test_count_samx_diagonal_up() {
        let input = vec![
            "...X.S".bytes().collect(),
            "..M...".bytes().collect(),
            ".A.M..".bytes().collect(),
            "S.A..S".bytes().collect(),
            ".X....".bytes().collect()
        ];
        let xmass = count_xmas(input);

        assert_eq!(xmass, 1);
    }

    /// Test that counting diagonal up XMAS works.
    #[test]
    fn test_count_xmas_diagonal_up() {
        let input = vec![
            "...X.S".bytes().collect(),
            ".....S".bytes().collect(),
            ".A.MA.".bytes().collect(),
            "S.AM.S".bytes().collect(),
            ".XX...".bytes().collect()
        ];
        let xmass = count_xmas(input);

        assert_eq!(xmass, 1);
    }

    /// Test that counting 'XMAS' and 'SAMX' produces correct count.
    #[test]
    fn test_count_xmas_1() {
        let input = vec![
            "..X...".bytes().collect(),
            ".SAMX.".bytes().collect(),
            ".A..A.".bytes().collect(),
            "XMAS.S".bytes().collect(),
            ".X....".bytes().collect()
        ];
        let xmass = count_xmas(input);

        assert_eq!(xmass, 4);
    }

    /// Test that counting 'XMAS' and 'SAMX' produces correct count.
    #[test]
    fn test_count_xmas_2() {
        let input = vec![
            "MMMSXXMASM".bytes().collect(),
            "MSAMXMSMSA".bytes().collect(),
            "AMXSXMAAMM".bytes().collect(),
            "MSAMASMSMX".bytes().collect(),
            "XMASAMXAMM".bytes().collect(),
            "XXAMMXXAMA".bytes().collect(),
            "SMSMSASXSS".bytes().collect(),
            "SAXAMASAAA".bytes().collect(),
            "MAMMMXMMMM".bytes().collect(),
            "MXMXAXMASX".bytes().collect()
        ];
        let xmass = count_xmas(input);

        assert_eq!(xmass, 18);
    }

}
