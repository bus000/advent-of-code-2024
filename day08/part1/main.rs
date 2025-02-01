// --- Day 8: Resonant Collinearity ---
//
// You find yourselves on the roof of a top-secret Easter Bunny installation.
//
// While The Historians do their thing, you take a look at the familiar huge
// antenna. Much to your surprise, it seems to have been reconfigured to emit a
// signal that makes people 0.1% more likely to buy Easter Bunny brand Imitation
// Mediocre Chocolate as a Christmas gift! Unthinkable!
//
// Scanning across the city, you find that there are actually many such
// antennas. Each antenna is tuned to a specific frequency indicated by a single
// lowercase letter, uppercase letter, or digit. You create a map (your puzzle
// input) of these antennas. For example:
//
//    ............
//    ........0...
//    .....0......
//    .......0....
//    ....0.......
//    ......A.....
//    ............
//    ............
//    ........A...
//    .........A..
//    ............
//    ............
//
// The signal only applies its nefarious effect at specific antinodes based on
// the resonant frequencies of the antennas. In particular, an antinode occurs
// at any point that is perfectly in line with two antennas of the same
// frequency - but only when one of the antennas is twice as far away as the
// other. This means that for any pair of antennas with the same frequency,
// there are two antinodes, one on either side of them.
//
// So, for these two antennas with frequency a, they create the two antinodes
// marked with #:
//
//    ..........
//    ...#......
//    ..........
//    ....a.....
//    ..........
//    .....a....
//    ..........
//    ......#...
//    ..........
//    ..........
//
// Adding a third antenna with the same frequency creates several more
// antinodes. It would ideally add four antinodes, but two are off the right
// side of the map, so instead it adds only two:
//
//    ..........
//    ...#......
//    #.........
//    ....a.....
//    ........a.
//    .....a....
//    ..#.......
//    ......#...
//    ..........
//    ..........
//
// Antennas with different frequencies don't create antinodes; A and a count as
// different frequencies. However, antinodes can occur at locations that contain
// antennas. In this diagram, the lone antenna with frequency capital A creates
// no antinodes but has a lowercase-a-frequency antinode at its location:
//
//    ..........
//    ...#......
//    #.........
//    ....a.....
//    ........a.
//    .....a....
//    ..#.......
//    ......A...
//    ..........
//    ..........
//
// The first example has antennas with two different frequencies, so the
// antinodes they create look like this, plus an antinode overlapping the
// topmost A-frequency antenna:
//
//    ......#....#
//    ...#....0...
//    ....#0....#.
//    ..#....0....
//    ....0....#..
//    .#....A.....
//    ...#........
//    #......#....
//    ........A...
//    .........A..
//    ..........#.
//    ..........#.
//
// Because the topmost A-frequency antenna overlaps with a 0-frequency antinode,
// there are 14 total unique locations that contain an antinode within the
// bounds of the map.
//
// Calculate the impact of the signal. How many unique locations within the
// bounds of the map contain an antinode?
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
                let antinode_1 = (x1 + diff_x, y1 + diff_y);
                let antinode_2 = (x2 - diff_x, y2 - diff_y);
                if self.in_bounds(antinode_1) {
                    antinodes.insert(antinode_1);
                }
                if self.in_bounds(antinode_2) {
                    antinodes.insert(antinode_2);
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

        assert_eq!(antenna_map.signal_impact(), 14);
    }

}
