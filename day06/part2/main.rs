// --- Part Two ---
//
// While The Historians begin working around the guard's patrol route, you
// borrow their fancy device and step outside the lab. From the safety of a
// supply closet, you time travel through the last few months and record the
// nightly status of the lab's guard post on the walls of the closet.
//
// Returning after what seems like only a few seconds to The Historians, they
// explain that the guard's patrol area is simply too large for them to safely
// search the lab without getting caught.
//
// Fortunately, they are pretty sure that adding a single new obstruction won't
// cause a time paradox. They'd like to place the new obstruction in such a way
// that the guard will get stuck in a loop, making the rest of the lab safe to
// search.
//
// To have the lowest chance of creating a time paradox, The Historians would
// like to know all of the possible positions for such an obstruction. The new
// obstruction can't be placed at the guard's starting position - the guard is
// there right now and would notice.
//
// In the above example, there are only 6 different positions where a new
// obstruction would cause the guard to get stuck in a loop. The diagrams of
// these six situations use O to mark the new obstruction, | to show a position
// where the guard moves up/down, - to show a position where the guard moves
// left/right, and + to show a position where the guard moves both up/down and
// left/right.
//
// Option one, put a printing press next to the guard's starting position:
//
//    ....#.....
//    ....+---+#
//    ....|...|.
//    ..#.|...|.
//    ....|..#|.
//    ....|...|.
//    .#.O^---+.
//    ........#.
//    #.........
//    ......#...
//
// Option two, put a stack of failed suit prototypes in the bottom right
// quadrant of the mapped area:
//
//    ....#.....
//    ....+---+#
//    ....|...|.
//    ..#.|...|.
//    ..+-+-+#|.
//    ..|.|.|.|.
//    .#+-^-+-+.
//    ......O.#.
//    #.........
//    ......#...
// Option three, put a crate of chimney-squeeze prototype fabric next to the
// standing desk in the bottom right quadrant:
//
//    ....#.....
//    ....+---+#
//    ....|...|.
//    ..#.|...|.
//    ..+-+-+#|.
//    ..|.|.|.|.
//    .#+-^-+-+.
//    .+----+O#.
//    #+----+...
//    ......#...
//
// Option four, put an alchemical retroencabulator near the bottom left corner:
//
//    ....#.....
//       ....+---+#
//       ....|...|.
//       ..#.|...|.
//       ..+-+-+#|.
//       ..|.|.|.|.
//       .#+-^-+-+.
//       ..|...|.#.
//       #O+---+...
//       ......#...
//
// Option five, put the alchemical retroencabulator a bit to the right instead:
//
//    ....#.....
//    ....+---+#
//    ....|...|.
//    ..#.|...|.
//    ..+-+-+#|.
//    ..|.|.|.|.
//    .#+-^-+-+.
//    ....|.|.#.
//    #..O+-+...
//    ......#...
//
// Option six, put a tank of sovereign glue right next to the tank of universal
// solvent:
//
//    ....#.....
//    ....+---+#
//    ....|...|.
//    ..#.|...|.
//    ..+-+-+#|.
//    ..|.|.|.|.
//    .#+-^-+-+.
//    .+----++#.
//    #+----++..
//    ......#O..
//
// It doesn't really matter what you choose to use as an obstacle so long as you
// and The Historians can put it into position without the guard noticing. The
// important thing is having enough options that you can find one that minimizes
// time paradoxes, and in this example, there are 6 different positions you
// could choose.
//
// You need to get the guard stuck in a loop by adding a single new obstruction.
// How many different positions could you choose for this obstruction?
use std::io;
use std::io::Read;
use std::collections::HashSet;
use std::fmt::Debug;
use std::cmp::max;
use aoc2024::aoc::AocError;

fn main() -> Result<(), AocError> {
    let mut lab = read_lab_stdin()?;
    let obstructions = find_obstructions_producing_cycles(&mut lab);
    println!("{:?}", obstructions.len());
    return Ok(());
}

/// Input of the problem.
#[derive(Debug, Eq, PartialEq, Clone)]
struct Input {

    /// The lab.
    lab: Lab,

    /// The guards position in/out of the lab.
    guard: Guard

}

/// A lab consists of a lab boundary and a set of obstructions.
#[derive(Debug, Eq, PartialEq, Clone)]
struct Lab {

    /// The maximum x value before leaving the lab.
    x_max: usize,

    /// The maximum y value before leaving the lab.
    y_max: usize,

    /// List of obstructions to the guards path.
    obstructions: HashSet<Position>,

    /// An extra obstruction to prevent mutation of the above set.
    extra_obstruction: Option<Position>

}

/// A guard has a position and a direction of movement.
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Guard {

    /// The current position of the guard.
    guard_position: Position,

    /// The guards current direction of movement.
    guard_direction: Direction,

}

/// Directions that can be moved in.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Direction {
    North, East, South, West
}

/// (x, y) position in the grid.
type Position = (i32, i32);

/// An action taken by the guard.
enum Action {
    Turn, Move
}

/// Find a list of possible obstructions that will generate a loop.
///
/// Runs the given lab simulation, in each location the guard visits we will
/// place an obstruction and run another simulation. If the guard loops the
/// position is added to the returned set.
fn find_obstructions_producing_cycles(input: &mut Input) -> HashSet<Position> {
    let lab = &mut input.lab;
    let initial_guard = &input.guard;
    let mut guard = initial_guard.clone();
    let mut obstructions = HashSet::new();
    loop {
        let action = move_guard(&lab, &mut guard);
        if !in_bounds(&lab, guard.guard_position) {
            return obstructions;
        }
        match action {
            Action::Move => {
                lab.extra_obstruction = Some(guard.guard_position);
                if detect_loop(&lab, &mut initial_guard.clone()) {
                    obstructions.insert(guard.guard_position);
                }
                lab.extra_obstruction = None;
            }
            Action::Turn => { },
        }
    }
}

/// Detects whether or not there is a loop in the guards path.
fn detect_loop(lab: &Lab, guard: &mut Guard) -> bool {
    let mut turns = HashSet::new();
    while in_bounds(&lab, guard.guard_position) {
        match move_guard(&lab, guard) {
            Action::Turn => {
                if turns.contains(guard) {
                    return true;
                } else {
                    turns.insert(guard.clone());
                }
            },
            Action:: Move => { }
        }
    }

    return false;
}

/// Move the guard in the lab according to the rules.
///
/// Will return the action taken by the guard.
fn move_guard(lab: &Lab, guard: &mut Guard) -> Action {
    let new_pos = move_position(&guard.guard_position, guard.guard_direction);
    if lab.extra_obstruction == Some(new_pos) ||
        lab.obstructions.contains(&new_pos) {

        guard.guard_direction = turn_right(guard.guard_direction);
        return Action::Turn;
    } else {
        guard.guard_position = new_pos;
        return Action::Move;
    }
}

/// Compute new position if taking a step in the direction given.
fn move_position(position: &Position, direction: Direction) -> Position {
    let (x, y) = position;
    match direction {
        Direction::North => (*x, y - 1),
        Direction::East => (x + 1, *y),
        Direction::South => (*x, y + 1),
        Direction::West => (x - 1, *y)
    }
}

/// Turn right of the given direction.
fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North
    }
}

/// Return true if the guard is inside the bounds of the lab and false
/// otherwise.
fn in_bounds(lab: &Lab, position: Position) -> bool {
    let (x, y) = position;
    let x_max = lab.x_max.try_into().expect("An i32 to be big enough for this map.");
    let y_max = lab.y_max.try_into().expect("An i32 to be big enough for this map.");
    return x >= 0 && x <= x_max && y >= 0 && y <= y_max;
}

/// Read a lab state from stdin or fail if given something useless.
fn read_lab_stdin() -> Result<Input, AocError> {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    return read_lab(&mut stdin_lock);
}

/// Read a lab state from the given reader or fail if given something useless.
fn read_lab(reader: &mut dyn Read) -> Result<Input, AocError> {
    let mut x = 0;
    let mut y = 0;
    let mut obstructions = HashSet::new();
    let mut guard = None;
    let mut buffer = [0; 1024];
    let mut max_x = 0;

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => {
                for i in 0..bytes_read {
                    match buffer[i] {
                        b'#' => {
                            obstructions.insert((x, y));
                            x = x + 1;
                        },
                        b'^' => {
                            guard = Some((x, y));
                            x = x + 1;
                        },
                        b'\n' => {
                            max_x = max(x, max_x);
                            x = 0;
                            y = y + 1;
                        },
                        _ => {
                            x = x + 1;
                        }
                    }
                }
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    let guard_position = guard
        .ok_or(AocError::UnexpectedError("Did not find a guard.".to_string()))?;
    return Ok(Input {
        lab: Lab {
            x_max: (max_x - 1).try_into().unwrap(),
            y_max: (y - 1).try_into().unwrap(),
            obstructions: obstructions,
            extra_obstruction: None
        },
        guard: Guard {
            guard_position: guard_position,
            guard_direction: Direction::North
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that reading a lab returns error if not given a guard position.
    #[test]
    fn test_read_lab_1() {
        let input = concat![
            "....#.....\n",
            ".........#\n",
            "..........\n",
            "..#.......\n",
            ".......#..\n",
            "..........\n",
            ".#........\n",
            "........#.\n",
            "#.........\n",
            "......#...\n"
        ];
        let error = read_lab(&mut input.as_bytes()).err().unwrap();
        assert_eq!(
            AocError::UnexpectedError("Did not find a guard.".to_string()),
            error);
    }

    /// Test that reading a lab works when a guard position is given.
    #[test]
    fn test_read_lab_2() {
        let input = concat![
            "....#.....\n",
            ".........#\n",
            "..........\n",
            "..#.......\n",
            ".......#..\n",
            "..........\n",
            ".#..^.....\n",
            "........#.\n",
            "#.........\n",
            "......#...\n"
        ];
        let input = read_lab(&mut input.as_bytes()).unwrap();
        let lab = input.lab;
        let guard = input.guard;
        assert_eq!(lab.x_max, 9);
        assert_eq!(lab.y_max, 9);
        assert_eq!(guard.guard_position, (4, 6));
        assert_eq!(guard.guard_direction, Direction::North);
        assert_eq!(lab.obstructions, HashSet::from([
            (4, 0), (9, 1), (2, 3), (7, 4), (1, 6), (8, 7), (0, 8), (6, 9)
        ]));
    }

    /// Test that moving a lab a step forwards work.
    #[test]
    fn test_move_guard_1() {
        let input = concat![
            "....#.....\n",
            ".........#\n",
            "..........\n",
            "..#.......\n",
            ".......#..\n",
            "..........\n",
            ".#..^.....\n",
            "........#.\n",
            "#.........\n",
            "......#...\n"
        ];
        let mut input = read_lab(&mut input.as_bytes()).unwrap();
        let lab = input.lab;
        let guard = &mut input.guard;
        move_guard(&lab, guard);
        assert_eq!(lab.x_max, 9);
        assert_eq!(lab.y_max, 9);
        assert_eq!(guard.guard_position, (4, 5));
        assert_eq!(guard.guard_direction, Direction::North);
        assert_eq!(lab.obstructions, HashSet::from([
            (4, 0), (9, 1), (2, 3), (7, 4), (1, 6), (8, 7), (0, 8), (6, 9)
        ]));
    }

    /// Test that moving a lab a step forwards work.
    #[test]
    fn test_move_guard_2() {
        let input = concat![
            "....#.....\n",
            "....^....#\n",
            "..........\n",
            "..#.......\n",
            ".......#..\n",
            "..........\n",
            ".#........\n",
            "........#.\n",
            "#.........\n",
            "......#...\n"
        ];
        let mut input = read_lab(&mut input.as_bytes()).unwrap();
        let lab = input.lab;
        let guard = &mut input.guard;
        move_guard(&lab, guard);
        assert_eq!(lab.x_max, 9);
        assert_eq!(lab.y_max, 9);
        assert_eq!(guard.guard_position, (4, 1));
        assert_eq!(guard.guard_direction, Direction::East);
        assert_eq!(lab.obstructions, HashSet::from([
            (4, 0), (9, 1), (2, 3), (7, 4), (1, 6), (8, 7), (0, 8), (6, 9)
        ]));
    }

    /// Test that verifying position of guard works.
    #[test]
    fn test_in_bounds_1() {
        let input = concat![
            "....#.....\n",
            "....^....#\n",
            "..........\n",
            "..#.......\n",
            ".......#..\n",
            "..........\n",
            ".#........\n",
            "........#.\n",
            "#.........\n",
            "......#...\n"
        ];
        let input = read_lab(&mut input.as_bytes()).unwrap();
        assert!(in_bounds(&input.lab, (4, 1)));
    }

    /// Test that verifying position of guard works.
    #[test]
    fn test_in_bounds_2() {
        let input = concat![
            "....#.....\n",
            ".........#\n",
            "..........\n",
            "..#.......\n",
            ".......#..\n",
            "..........\n",
            ".#........\n",
            "........#.\n",
            "#.........\n",
            "^.....#...\n"
        ];
        let input = read_lab(&mut input.as_bytes()).unwrap();
        assert!(in_bounds(&input.lab, (0, 9)));
    }

    /// Test that verifying position of guard works.
    #[test]
    fn test_in_bounds_3() {
        let input = concat![
            "^...#.....\n",
            ".........#\n",
            "..........\n",
            "..#.......\n",
            ".......#..\n",
            "..........\n",
            ".#........\n",
            "........#.\n",
            "#.........\n",
            "......#...\n"
        ];
        let input = read_lab(&mut input.as_bytes()).unwrap();
        assert!(!in_bounds(&input.lab, (0, -1)));
    }

    /// Test that running a simulation of a guard that leaves the lab returns
    /// false.
    #[test]
    fn test_detect_loop_1() {
        let input = concat![
            "....#.....\n",
            ".........#\n",
            "..........\n",
            "..#.......\n",
            ".......#..\n",
            "..........\n",
            ".#..^.....\n",
            "........#.\n",
            "#.........\n",
            "......#...\n"
        ];
        let mut input = read_lab(&mut input.as_bytes()).unwrap();
        assert!(!detect_loop(&input.lab, &mut input.guard));
    }

    /// Test that running a simulation of a guard that doesn't leave the lab
    /// returns true.
    #[test]
    fn test_detect_loop_2() {
        let input = concat![
            "....#.....\n",
            ".........#\n",
            "..........\n",
            "..#.......\n",
            ".......#..\n",
            "..........\n",
            ".#..^.....\n",
            "......#.#.\n",
            "#.........\n",
            "......#...\n"
        ];
        let mut input = read_lab(&mut input.as_bytes()).unwrap();
        assert!(detect_loop(&input.lab, &mut input.guard));
    }

    /// Test that finding obstructions returns the correct list.
    #[test]
    fn test_find_obstructions_producing_cycles() {
        let input = concat![
            "....#.....\n",
            ".........#\n",
            "..........\n",
            "..#.......\n",
            ".......#..\n",
            "..........\n",
            ".#..^.....\n",
            "........#.\n",
            "#.........\n",
            "......#...\n"
        ];
        let mut input = read_lab(&mut input.as_bytes()).unwrap();
        let obstructions = find_obstructions_producing_cycles(&mut input);
        assert_eq!(obstructions.len(), 6);
    }

}
