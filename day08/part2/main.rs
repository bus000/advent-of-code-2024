// --- Part Two ---
//
// Watching over your shoulder as you work, one of The Historians asks if you
// took the effects of resonant harmonics into your calculations.
//
// Whoops!
//
// After updating your model, it turns out that an antinode occurs at any grid
// position exactly in line with at least two antennas of the same frequency,
// regardless of distance. This means that some of the new antinodes will occur
// at the position of each antenna (unless that antenna is the only one of its
// frequency).
//
// So, these three T-frequency antennas now create many antinodes:
//
//    T....#....
//    ...T......
//    .T....#...
//    .........#
//    ..#.......
//    ..........
//    ...#......
//    ..........
//    ....#.....
//    ..........
//
// In fact, the three T-frequency antennas are all exactly in line with two
// antennas, so they are all also antinodes! This brings the total number of
// antinodes in the above example to 9.
//
// The original example now has 34 antinodes, including the antinodes that
// appear on every antenna:
//
//    ##....#....#
//    .#.#....0...
//    ..#.#0....#.
//    ..##...0....
//    ....0....#..
//    .#...#A....#
//    ...#..#.....
//    #....#.#....
//    ..#.....A...
//    ....#....A..
//    .#........#.
//    ...#......##
//
// Calculate the impact of the signal using this updated model. How many unique
// locations within the bounds of the map contain an antinode?
use std::io;
use std::io::Read;
use std::fmt::Debug;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use aoc2024::aoc::AocError;

fn main() -> Result<(), AocError> {
    let antenna_map = read_antennas_stdin()?;
    println!("{}", antenna_map.signal_impact());
    return Ok(());
}

/// A position is an index in the 2D grid.
type Position = (i32, i32);

/// A map containing antennas.
#[derive(Debug, PartialEq, Eq)]
struct AntennaMap {

    /// Mapping from frequency to set of antennas of that frequency.
    antennas: HashMap<u8, HashSet<Position>>,

    /// The maximum x value that are inside the map.
    x_max: i32,

    /// The maximum y value that are inside the map.
    y_max: i32

}

impl AntennaMap {

    /// Compute the impact of the signal.
    ///
    /// The impact is the number of antinodes of the antennas.
    fn signal_impact(&self) -> usize {
        let mut antinodes = HashSet::new();

        for (_, antennas) in self.antennas.iter() {
            for ((x1, y1), (x2, y2)) in antennas.iter().tuple_combinations() {
                let diff_x = x1 - x2;
                let diff_y = y1 - y2;
                for antinode in self.line((*x1, *y1), (diff_x, diff_y)) {
                    antinodes.insert(antinode);
                }
            }
        }

        return antinodes.len();
    }

    /// Check whether a position is inside the antenna map.
    fn in_bounds(&self, position: Position) -> bool {
        let (x, y) = position;
        return x <= self.x_max && x >= 0 && y <= self.y_max && y >= 0;
    }

    /// Compute a line starting at the start point going in both directions
    /// applying the diff given. The vector might contain duplicate points.
    fn line(&self, start: Position, diff: Position) -> Vec<Position> {
        let mut line = Vec::new();
        let (mut x, mut y) = start;
        let (diff_x, diff_y) = diff;
        while self.in_bounds((x, y)) {
            line.push((x, y));
            x = x + diff_x;
            y = y + diff_y;
        } 

        (x, y) = start;
        while self.in_bounds((x, y)) {
            line.push((x, y));
            x = x - diff_x;
            y = y - diff_y;
        } 

        return line;
    }

}

/// Read an antenna map from stdin.
fn read_antennas_stdin() -> Result<AntennaMap, AocError> {
    let stdin = io::stdin();
    let mut lock = stdin.lock();
    return read_antennas(&mut lock);
}

/// Read antennas from the given reader.
fn read_antennas(reader: &mut dyn Read) -> Result<AntennaMap, AocError> {
    let mut x = 0;
    let mut y = 0;
    let mut antennas: HashMap<u8, HashSet<Position>> = HashMap::new();
    let mut buffer = [0; 1024];
    let mut max_x = 0;

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        for i in 0..bytes_read {
            match buffer[i] {
                b'.' => {
                    x = x + 1;
                },
                b'\n' => {
                    max_x = max(x, max_x);
                    x = 0;
                    y = y + 1;
                },
                b => {
                    antennas.entry(b).or_default().insert((x, y));
                    x = x + 1;
                }
            }
        }
    }

    return Ok(AntennaMap {
        antennas: antennas,
        x_max: (max_x - 1).try_into().unwrap(),
        y_max: (y - 1).try_into().unwrap()
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that reading antenna map works.
    #[test]
    fn test_read_antennas() {
        let input = concat![
            "............\n",
            "........0...\n",
            ".....0......\n",
            ".......0....\n",
            "....0.......\n",
            "......A.....\n",
            "............\n",
            "............\n",
            "........A...\n",
            ".........A..\n",
            "............\n",
            "............\n"
        ];
        let antennas = read_antennas(&mut input.as_bytes()).unwrap();

        assert_eq!(antennas.antennas.len(), 2);
        assert_eq!(antennas.antennas[&b'0'],
            HashSet::from([(8, 1), (5, 2), (7, 3), (4, 4)]));
        assert_eq!(antennas.antennas[&b'A'],
            HashSet::from([(6, 5), (8, 8), (9, 9)]));
        assert_eq!(antennas.x_max, 11);
        assert_eq!(antennas.y_max, 11);
    }

    /// Test that computing the signal impact works.
    #[test]
    fn test_signal_impact() {
        let mut antennas = HashMap::new();
        antennas.insert(b'0', HashSet::from([(8, 1), (5, 2), (7, 3), (4, 4)]));
        antennas.insert(b'A', HashSet::from([(6, 5), (8, 8), (9, 9)]));
        let antenna_map = AntennaMap {
            antennas: antennas,
            x_max: 11,
            y_max: 11
        };

        assert_eq!(antenna_map.signal_impact(), 34);
    }

}
