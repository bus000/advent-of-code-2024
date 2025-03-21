// --- Part Two ---
//
//The reindeer spends a few minutes reviewing your hiking trail map before
//realizing something, disappearing for a few minutes, and finally returning
//with yet another slightly-charred piece of paper.
//
// The paper describes a second way to measure a trailhead called its rating. A
// trailhead's rating is the number of distinct hiking trails which begin at
// that trailhead. For example:
//
//    .....0.
//    ..4321.
//    ..5..2.
//    ..6543.
//    ..7..4.
//    ..8765.
//    ..9....
//
// The above map has a single trailhead; its rating is 3 because there are
// exactly three distinct hiking trails which begin at that position:
//
//    .....0.   .....0.   .....0.
//    ..4321.   .....1.   .....1.
//    ..5....   .....2.   .....2.
//    ..6....   ..6543.   .....3.
//    ..7....   ..7....   .....4.
//    ..8....   ..8....   ..8765.
//    ..9....   ..9....   ..9....
//
// Here is a map containing a single trailhead with rating 13:
//
//    ..90..9
//    ...1.98
//    ...2..7
//    6543456
//    765.987
//    876....
//    987....
//
// This map contains a single trailhead with rating 227 (because there are 121
// distinct hiking trails that lead to the 9 on the right edge and 106 that lead
// to the 9 on the bottom edge):
//
//    012345
//    123456
//    234567
//    345678
//    4.6789
//    56789.
//
// Here's the larger example from before:
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
// Considering its trailheads in reading order, they have ratings of 20, 24, 10,
// 4, 1, 4, 5, 8, and 5. The sum of all trailhead ratings in this larger example
// topographic map is 81.
//
// You're not sure how, but the reindeer seems to have crafted some tiny flags
// out of toothpicks and bits of paper and is using them to mark trailheads on
// your topographic map. What is the sum of the ratings of all trailheads?
use std::io;
use std::io::Read;
use std::fmt::Debug;
use aoc2024::aoc::AocError;

fn main() -> Result<(), AocError> {
    let map = read_topographical_map_stdin()?;
    let score: u64 = map
        .positions()
        .filter(|h| h.height == 0)
        .map(|h| map.find_trail_counts(h))
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

    /// Find the number of distinct trails starting from the given location and
    /// ending at a 9.
    fn find_trail_counts(&self, current: Height) -> u64 {
        if current.height == 9 {
            return 1;
        }
        let mut trails = 0;
        if let Some(left) = self.left(current) {
            if current.can_walk(&left) {
                trails += self.find_trail_counts(left);
            }
        }
        if let Some(right) = self.right(current) {
            if current.can_walk(&right) {
                trails += self.find_trail_counts(right);
            }
        }
        if let Some(up) = self.up(current) {
            if current.can_walk(&up) {
                trails += self.find_trail_counts(up);
            }
        }
        if let Some(down) = self.down(current) {
            if current.can_walk(&down) {
                trails += self.find_trail_counts(down);
            }
        }

        return trails;
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

    /// Test that counting trails returns the correct count.
    #[test]
    fn test_find_trail_counts_1() {
        let input = concat![
            ".....0.\n",
            "..4321.\n",
            "..5..2.\n",
            "..6543.\n",
            "..7..4.\n",
            "..8765.\n",
            "..9....\n",
        ];
        let map = read_topographical_map(&mut input.as_bytes()).unwrap();
        let start = map.height(5, 0).unwrap();
        assert_eq!(map.find_trail_counts(start), 3);
    }

    /// Test that counting trails returns the correct count.
    #[test]
    fn test_find_trail_counts_2() {
        let input = concat![
            ".....0.\n",
            "..4321.\n",
            "..5..2.\n",
            "..6543.\n",
            "..7..4.\n",
            "..8765.\n",
            "..9....\n",
        ];
        let map = read_topographical_map(&mut input.as_bytes()).unwrap();
        let start = map.height(5, 2).unwrap();
        assert_eq!(map.find_trail_counts(start), 2);
    }

    /// Test that counting trails returns the correct count.
    #[test]
    fn test_find_trail_counts_3() {
        let input = concat![
            "..90..9\n",
            "...1.98\n",
            "...2..7\n",
            "6543456\n",
            "765.987\n",
            "876....\n",
            "987....\n",
        ];
        let map = read_topographical_map(&mut input.as_bytes()).unwrap();
        let start = map.height(3, 0).unwrap();
        assert_eq!(map.find_trail_counts(start), 13);
    }

    /// Test that counting trails returns the correct count.
    #[test]
    fn test_find_trail_counts_4() {
        let input = concat![
            "012345\n",
            "123456\n",
            "234567\n",
            "345678\n",
            "4.6789\n",
            "56789.\n",
        ];
        let map = read_topographical_map(&mut input.as_bytes()).unwrap();
        let start = map.height(0, 0).unwrap();
        assert_eq!(map.find_trail_counts(start), 227);
    }

}
