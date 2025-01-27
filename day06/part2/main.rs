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
    let obstructions = find_obstructions(&mut lab);
    println!("{:?}", obstructions.len());
    return Ok(());
}

/// A lab consists of a lab boundary, a set of obstructions, and a single guard.
#[derive(Debug, Eq, PartialEq, Clone)]
struct Lab {

    /// The maximum x value before leaving the lab.
    x_max: usize,

    /// The maximum y value before leaving the lab.
    y_max: usize,

    /// The current position of the guard.
    guard_position: Position,

    /// The guards current direction of movement.
    guard_direction: Direction,

    /// List of obstructions to the guards path.
    obstructions: HashSet<Position>

}

/// Directions that can be moved in.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Direction {
    North, East, South, West
}

/// (x, y) position in the grid.
type Position = (i32, i32);

/// Find a list of possible obstructions that will generate a loop.
///
/// Runs the given lab simulation, in each location the guard visits we will
/// place an obstruction and run another simulation. If the guard loops the
/// position is added to the returned set.
fn find_obstructions(lab: &mut Lab) -> HashSet<Position> {
    let original = lab.clone();
    let mut obstructions = HashSet::new();
    loop {
        if !guard_in_bounds(&lab) {
            return obstructions;
        } else {
            move_lab(lab);
            let mut with_obstruction = original.clone();
            with_obstruction.obstructions.insert(lab.guard_position);
            if run_simulation(&mut with_obstruction) {
                obstructions.insert(lab.guard_position);
            }
        }
    }
}

/// Simulate the given lab until done.
///
/// Runs the given lab until the guard has either left the lab or has looped to
/// a previous location. Returns true if the guard looped and false otherwise.
fn run_simulation(lab: &mut Lab) -> bool {
    let mut guard_states = HashSet::new();
    loop {
        if guard_states.contains(&(lab.guard_position, lab.guard_direction)) {
            return true;
        } else if !guard_in_bounds(&lab) {
            return false;
        } else {
            guard_states.insert((lab.guard_position, lab.guard_direction));
            move_lab(lab);
        }
    }
}

/// Move the guard in the lab according to the rules.
fn move_lab(lab: &mut Lab) {
    let new_pos = move_position(&lab.guard_position, lab.guard_direction);
    if lab.obstructions.contains(&new_pos) {
        lab.guard_direction = turn_right(lab.guard_direction);
    } else {
        lab.guard_position = new_pos;
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
fn guard_in_bounds(lab: &Lab) -> bool {
    let (x, y) = lab.guard_position;
    let x_max = lab.x_max.try_into().expect("An i32 to be big enough for this map.");
    let y_max = lab.y_max.try_into().expect("An i32 to be big enough for this map.");
    return x >= 0 && x <= x_max && y >= 0 && y <= y_max;
}

/// Read a lab state from stdin or fail if given something useless.
fn read_lab_stdin() -> Result<Lab, AocError> {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    return read_lab(&mut stdin_lock);
}

/// Read a lab state from the given reader or fail if given something useless.
fn read_lab(reader: &mut dyn Read) -> Result<Lab, AocError> {
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
    return Ok(Lab {
        x_max: (max_x - 1).try_into().unwrap(),
        y_max: (y - 1).try_into().unwrap(),
        guard_position: guard_position,
        guard_direction: Direction::North,
        obstructions: obstructions
    });
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
        let lab = read_lab(&mut input.as_bytes()).unwrap();
        assert_eq!(lab.x_max, 9);
        assert_eq!(lab.y_max, 9);
        assert_eq!(lab.guard_position, (4, 6));
        assert_eq!(lab.guard_direction, Direction::North);
        assert_eq!(lab.obstructions, HashSet::from([
            (4, 0), (9, 1), (2, 3), (7, 4), (1, 6), (8, 7), (0, 8), (6, 9)
        ]));
    }

    /// Test that moving a lab a step forwards work.
    #[test]
    fn test_move_lab_1() {
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
        let mut lab = read_lab(&mut input.as_bytes()).unwrap();
        move_lab(&mut lab);
        assert_eq!(lab.x_max, 9);
        assert_eq!(lab.y_max, 9);
        assert_eq!(lab.guard_position, (4, 5));
        assert_eq!(lab.guard_direction, Direction::North);
        assert_eq!(lab.obstructions, HashSet::from([
            (4, 0), (9, 1), (2, 3), (7, 4), (1, 6), (8, 7), (0, 8), (6, 9)
        ]));
    }

    /// Test that moving a lab a step forwards work.
    #[test]
    fn test_move_lab_2() {
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
        let mut lab = read_lab(&mut input.as_bytes()).unwrap();
        move_lab(&mut lab);
        assert_eq!(lab.x_max, 9);
        assert_eq!(lab.y_max, 9);
        assert_eq!(lab.guard_position, (4, 1));
        assert_eq!(lab.guard_direction, Direction::East);
        assert_eq!(lab.obstructions, HashSet::from([
            (4, 0), (9, 1), (2, 3), (7, 4), (1, 6), (8, 7), (0, 8), (6, 9)
        ]));
    }

    /// Test that verifying position of guard works.
    #[test]
    fn test_guard_in_bounds_1() {
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
        let lab = read_lab(&mut input.as_bytes()).unwrap();
        assert!(guard_in_bounds(&lab));
    }

    /// Test that verifying position of guard works.
    #[test]
    fn test_guard_in_bounds_2() {
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
        let lab = read_lab(&mut input.as_bytes()).unwrap();
        assert!(guard_in_bounds(&lab));
    }

    /// Test that verifying position of guard works.
    #[test]
    fn test_guard_in_bounds_3() {
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
        let mut lab = read_lab(&mut input.as_bytes()).unwrap();
        move_lab(&mut lab);
        assert!(!guard_in_bounds(&lab));
    }

    /// Test that running a simulation of a guard that leaves the lab returns
    /// false.
    #[test]
    fn test_run_simulation_1() {
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
        let mut lab = read_lab(&mut input.as_bytes()).unwrap();
        assert!(!run_simulation(&mut lab));
    }

    /// Test that running a simulation of a guard that doesn't leave the lab
    /// returns true.
    #[test]
    fn test_run_simulation_2() {
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
        let mut lab = read_lab(&mut input.as_bytes()).unwrap();
        assert!(run_simulation(&mut lab));
    }

    /// Test that finding obstructions returns the correct list.
    #[test]
    fn test_find_obstructions() {
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
        let mut lab = read_lab(&mut input.as_bytes()).unwrap();
        let obstructions = find_obstructions(&mut lab);
        assert_eq!(obstructions.len(), 6);
    }

}
