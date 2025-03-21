// --- Day 10: Hoof It ---
//
// You all arrive at a Lava Production Facility on a floating island in the sky.
// As the others begin to search the massive industrial complex, you feel a
// small nose boop your leg and look down to discover a reindeer wearing a hard
// hat.
//
// The reindeer is holding a book titled "Lava Island Hiking Guide". However,
// when you open the book, you discover that most of it seems to have been
// scorched by lava! As you're about to ask how you can help, the reindeer
// brings you a blank topographic map of the surrounding area (your puzzle
// input) and looks up at you excitedly.
//
// Perhaps you can help fill in the missing hiking trails?
//
// The topographic map indicates the height at each position using a scale from
// 0 (lowest) to 9 (highest). For example:
//
//    0123
//    1234
//    8765
//    9876
//
// Based on un-scorched scraps of the book, you determine that a good hiking
// trail is as long as possible and has an even, gradual, uphill slope. For all
// practical purposes, this means that a hiking trail is any path that starts at
// height 0, ends at height 9, and always increases by a height of exactly 1 at
// each step. Hiking trails never include diagonal steps - only up, down, left,
// or right (from the perspective of the map).
//
// You look up from the map and notice that the reindeer has helpfully begun to
// construct a small pile of pencils, markers, rulers, compasses, stickers, and
// other equipment you might need to update the map with hiking trails.
//
// A trailhead is any position that starts one or more hiking trails - here,
// these positions will always have height 0. Assembling more fragments of
// pages, you establish that a trailhead's score is the number of 9-height
// positions reachable from that trailhead via a hiking trail. In the above
// example, the single trailhead in the top left corner has a score of 1 because
// it can reach a single 9 (the one in the bottom left).
//
// This trailhead has a score of 2:
//
//    ...0...
//    ...1...
//    ...2...
//    6543456
//    7.....7
//    8.....8
//    9.....9
//
// (The positions marked . are impassable tiles to simplify these examples; they
// do not appear on your actual topographic map.)
//
// This trailhead has a score of 4 because every 9 is reachable via a hiking
// trail except the one immediately to the left of the trailhead:
//
//    ..90..9
//    ...1.98
//    ...2..7
//    6543456
//    765.987
//    876....
//    987....
//
// This topographic map contains two trailheads; the trailhead at the top has a
// score of 1, while the trailhead at the bottom has a score of 2:
//
//    10..9..
//    2...8..
//    3...7..
//    4567654
//    ...8..3
//    ...9..2
//    .....01
//
// Here's a larger example:
//
//    89010123
//    78121874
//    87430965
//    96549874
//    45678903
//    32019012
//    01329801
//    10456732
//
// This larger example has 9 trailheads. Considering the trailheads in reading
// order, they have scores of 5, 6, 5, 3, 1, 3, 5, 3, and 5. Adding these scores
// together, the sum of the scores of all trailheads is 36.
//
// The reindeer gleefully carries over a protractor and adds it to the pile.
// What is the sum of the scores of all trailheads on your topographic map?
use std::io;
use std::io::Read;
use std::fmt::Debug;
use aoc2024::aoc::AocError;
use std::collections::HashSet;

fn main() -> Result<(), AocError> {
    let map = read_topographical_map_stdin()?;
    let score: usize = map
        .positions()
        .filter(|h| h.height == 0)
        .map(|h| map.find_trail_ends(h).len())
        .sum();
    println!("{:?}", score);
    return Ok(());
}

/// A height in the array.
#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
struct Height {

    /// X position.
    x: usize,

    /// Y position.
    y: usize,

    /// The height at the position.
    height: u8,

}

impl Height {

    /// Check whether the height difference in walkable.
    ///
    /// A height difference is walkable only when the source is lower than the
    /// destination and the difference is exactly 1.
    fn can_walk(&self, other: &Height) -> bool {
        return self.height < other.height && other.height - self.height == 1;
    }

}

/// A topographical map.
#[derive(PartialEq, Eq, Debug, Clone)]
struct TopMap {

    /// Topographical heights with height * width elements. Indexed as
    /// heights[x + y * (x_max + 1)].
    heights: Vec<u8>,

    /// The number of columns in input.
    x_max: usize,

    /// The number of lines in input.
    y_max: usize,

}

impl TopMap {

    /// Return an iterator that iterates through all the heights in the map.
    fn positions(&self) -> impl Iterator<Item=Height> + '_ {
        return self.heights.iter()
            .enumerate()
            .map(|(i, h)| Height {
                x: i % (self.x_max + 1),
                y: i / (self.x_max + 1),
                height: *h
            });
    }

    /// Find trail ends of trails that start in the given position.
    fn find_trail_ends(&self, current: Height) -> HashSet<(usize, usize)> {
        if current.height == 9 {
            return HashSet::from([(current.x, current.y)]);
        }
        let mut trail_ends = HashSet::new();
        if let Some(left) = self.left(current) {
            if current.can_walk(&left) {
                for end in self.find_trail_ends(left) {
                    trail_ends.insert(end);
                }
            }
        }
        if let Some(right) = self.right(current) {
            if current.can_walk(&right) {
                for end in self.find_trail_ends(right) {
                    trail_ends.insert(end);
                }
            }
        }
        if let Some(up) = self.up(current) {
            if current.can_walk(&up) {
                for end in self.find_trail_ends(up) {
                    trail_ends.insert(end);
                }
            }
        }
        if let Some(down) = self.down(current) {
            if current.can_walk(&down) {
                for end in self.find_trail_ends(down) {
                    trail_ends.insert(end);
                }
            }
        }

        return trail_ends;
    }

    /// Get the height of the given position in the topographical map.
    fn height(&self, x: usize, y: usize) -> Option<Height> {
        return self.heights.get(x + (self.x_max + 1) * y)
            .copied()
            .map(|h| Height {x: x, y: y, height: h});
    }

    /// Return the height to the left of the given height.
    fn left(&self, height: Height) -> Option<Height> {
        if height.x == 0 {
            return None;
        } else {
            return self.height(height.x - 1, height.y);
        }
    }

    /// Return the height to the right of the given height.
    fn right(&self, height: Height) -> Option<Height> {
        if height.x == self.x_max {
            return None;
        } else {
            return self.height(height.x + 1, height.y);
        }
    }

    /// Return the position above the current position.
    fn up(&self, height: Height) -> Option<Height> {
        if height.y == 0 {
            return None;
        } else {
            return self.height(height.x, height.y - 1);
        }
    }

    /// Return the position below the current position.
    fn down(&self, height: Height) -> Option<Height> {
        if height.y == self.y_max {
            return None;
        } else {
            return self.height(height.x, height.y + 1);
        }
    }


}

/// Read topographical map from stdin.
fn read_topographical_map_stdin() -> Result<TopMap, AocError> {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    return read_topographical_map(&mut stdin_lock);
}

/// Read topographical map from given source.
fn read_topographical_map(reader: &mut dyn Read) -> Result<TopMap, AocError> {
    let mut buffer = [0; 1024];
    let mut lines = 0;
    let mut columns = 0;
    let mut max_columns = 0;
    let mut heights = Vec::new();

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => {
                return Ok(TopMap {
                    heights: heights,
                    x_max: max_columns - 2,
                    y_max: lines - 1,
                });
            },
            Ok(bytes_read) => {
                for i in 0..bytes_read {
                    let c = buffer[i];
                    columns += 1;
                    if c == b'\n' && max_columns == 0 {
                        max_columns = columns;
                        columns = 0;
                        lines += 1;
                    } else if c == b'\n' && max_columns != columns {
                        return Err(AocError::ParseInputError(
                            "Expected all lines to be equal length.".to_string()));
                    } else if c == b'\n' {
                        columns = 0;
                        lines += 1;
                    } else if c == b'.' {
                        heights.push(u8::MAX);
                    } else if c >= b'0' && c <= b'9' {
                        heights.push(buffer[i] - b'0');
                    } else {
                        return Err(AocError::ParseInputError(
                            "Expected only numbers, dots and newlines.".to_string()));
                    }
                }
            },
            Err(e) => {
                eprintln!("Could not read stdin: {:?}", e);
                return Err(e.into());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that reading a map works as expected.
    #[test]
    fn test_read_topographical_map_1() {
        let input = concat![
            "0123\n",
            "1234\n",
            "8765\n",
            "9876\n",
        ];
        let map = read_topographical_map(&mut input.as_bytes()).unwrap();
        assert_eq!(map.heights, vec![
            0, 1, 2, 3,
            1, 2, 3, 4,
            8, 7, 6, 5,
            9, 8, 7, 6]);
        assert_eq!(map.x_max, 3);
        assert_eq!(map.y_max, 3);
    }

    /// Test that if the input is not square we will error.
    #[test]
    fn test_read_topographical_map_2() {
        let input = concat![
            "012\n",
            "1234\n",
            "8765\n",
            "9876\n",
        ];
        let error = read_topographical_map(&mut input.as_bytes()).err().unwrap();
        assert_eq!(error, AocError::ParseInputError(
            "Expected all lines to be equal length.".to_string()));
    }

    /// Test that letters are not allowed.
    #[test]
    fn test_read_topographical_map_3() {
        let input = concat![
            "0123\n",
            "1234\n",
            "876k\n",
            "9876\n",
        ];
        let error = read_topographical_map(&mut input.as_bytes()).err().unwrap();
        assert_eq!(error, AocError::ParseInputError(
            "Expected only numbers, dots and newlines.".to_string()));
    }

    /// Test that dots will be read as the maximum height.
    #[test]
    fn test_read_topographical_map_4() {
        let input = concat![
            "...0...\n",
            "...1...\n",
            "...2...\n",
            "6543456\n",
            "7.....7\n",
            "8.....8\n",
            "9.....9\n",
        ];
        let map = read_topographical_map(&mut input.as_bytes()).unwrap();
        assert_eq!(map.heights, vec![
            255, 255, 255,   0, 255, 255, 255,
            255, 255, 255,   1, 255, 255, 255,
            255, 255, 255,   2, 255, 255, 255,
              6,   5,   4,   3,   4,   5,   6,
              7, 255, 255, 255, 255, 255,   7,
              8, 255, 255, 255, 255, 255,   8,
              9, 255, 255, 255, 255, 255,   9,
        ]);
        assert_eq!(map.x_max, 6);
        assert_eq!(map.y_max, 6);
    }

    /// Test that finding different heights works as expected.
    #[test]
    fn test_heights() {
        let input = concat![
            "...0...\n",
            "...1...\n",
            "...2...\n",
            "6543456\n",
            "7.....7\n",
            "8.....8\n",
            "9.....9\n",
        ];
        let map = read_topographical_map(&mut input.as_bytes()).unwrap();
        assert_eq!(map.height(0, 0), Some(Height {x: 0, y: 0, height: 255}));
        assert_eq!(map.height(3, 0), Some(Height {x: 3, y: 0, height: 0}));
        assert_eq!(map.height(3, 1), Some(Height {x: 3, y: 1, height: 1}));
        assert_eq!(map.height(3, 2), Some(Height {x: 3, y: 2, height: 2}));
        assert_eq!(map.height(2, 2), Some(Height {x: 2, y: 2, height: 255}));
        assert_eq!(map.height(4, 2), Some(Height {x: 4, y: 2, height: 255}));
        assert_eq!(map.height(6, 6), Some(Height {x: 6, y: 6, height: 9}));
        assert_eq!(map.height(7, 6), None);
    }

    /// Test that finding trail ends when multiple is available will return them
    /// both.
    #[test]
    fn test_find_trail_ends_1() {
        let input = concat![
            "...0...\n",
            "...1...\n",
            "...2...\n",
            "6543456\n",
            "7.....7\n",
            "8.....8\n",
            "9.....9\n",
        ];
        let map = read_topographical_map(&mut input.as_bytes()).unwrap();
        let start = map.height(3, 0).unwrap();
        assert_eq!(map.find_trail_ends(start), HashSet::from([(0, 6), (6, 6)]));
    }

}
