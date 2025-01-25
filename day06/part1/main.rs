// --- Day 6: Guard Gallivant ---
//
// The Historians use their fancy device again, this time to whisk you all away
// to the North Pole prototype suit manufacturing lab... in the year 1518! It
// turns out that having direct access to history is very convenient for a group
// of historians.
//
// You still have to be careful of time paradoxes, and so it will be important
// to avoid anyone from 1518 while The Historians search for the Chief.
// Unfortunately, a single guard is patrolling this part of the lab.
//
// Maybe you can work out where the guard will go ahead of time so that The
// Historians can search safely?
//
// You start by making a map (your puzzle input) of the situation. For example:
//
//    ....#.....
//    .........#
//    ..........
//    ..#.......
//    .......#..
//    ..........
//    .#..^.....
//    ........#.
//    #.........
//    ......#...
//
// The map shows the current position of the guard with ^ (to indicate the guard
// is currently facing up from the perspective of the map). Any obstructions -
// crates, desks, alchemical reactors, etc. - are shown as #.
//
// Lab guards in 1518 follow a very strict patrol protocol which involves
// repeatedly following these steps:
//
// * If there is something directly in front of you, turn right 90 degrees.
// * Otherwise, take a step forward.
//
// Following the above protocol, the guard moves up several times until she
// reaches an obstacle (in this case, a pile of failed suit prototypes):
//
//    ....#.....
//    ....^....#
//    ..........
//    ..#.......
//    .......#..
//    ..........
//    .#........
//    ........#.
//    #.........
//    ......#...
//
// Because there is now an obstacle in front of the guard, she turns right
// before continuing straight in her new facing direction:
//
//    ....#.....
//    ........>#
//    ..........
//    ..#.......
//    .......#..
//    ..........
//    .#........
//    ........#.
//    #.........
//    ......#...
//
// Reaching another obstacle (a spool of several very long polymers), she turns
// right again and continues downward:
//
//    ....#.....
//    .........#
//    ..........
//    ..#.......
//    .......#..
//    ..........
//    .#......v.
//    ........#.
//    #.........
//    ......#...
//
// This process continues for a while, but the guard eventually leaves the
// mapped area (after walking past a tank of universal solvent):
//
//    ....#.....
//    .........#
//    ..........
//    ..#.......
//    .......#..
//    ..........
//    .#........
//    ........#.
//    #.........
//    ......#v..
//
// By predicting the guard's route, you can determine which specific positions
// in the lab will be in the patrol path. Including the guard's starting
// position, the positions visited by the guard before leaving the area are
// marked with an X:
//
//    ....#.....
//    ....XXXXX#
//    ....X...X.
//    ..#.X...X.
//    ..XXXXX#X.
//    ..X.X.X.X.
//    .#XXXXXXX.
//    .XXXXXXX#.
//    #XXXXXXX..
//    ......#X..
//
// In this example, the guard will visit 41 distinct positions on your map.
//
// Predict the path of the guard. How many distinct positions will the guard
// visit before leaving the mapped area?
use std::io;
use std::io::Read;
use std::collections::HashSet;
use std::fmt::Debug;
use std::cmp::max;
use aoc2024::aoc::AocError;

fn main() -> Result<(), AocError> {
    let mut lab = read_lab_stdin()?;
    let mut positions = HashSet::new();
    loop {
        if guard_in_bounds(&lab) {
            positions.insert(lab.guard_position);
            move_lab(&mut lab);
        } else {
            break;
        }
    }
    println!("{:?}", positions.len());
    return Ok(());
}

/// A lab consists of a lab boundary, a set of obstructions, and a single guard.
#[derive(Debug, Eq, PartialEq)]
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
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North, East, South, West
}

/// (x, y) position in the grid.
type Position = (i32, i32);

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

}
