// --- Part Two ---
//
// Fortunately, the Elves are trying to order so much fence that they qualify
// for a bulk discount!
//
// Under the bulk discount, instead of using the perimeter to calculate the
// price, you need to use the number of sides each region has. Each straight
// section of fence counts as a side, regardless of how long it is.
//
// Consider this example again:
//
//    AAAA
//    BBCD
//    BBCC
//    EEEC
//
// The region containing type A plants has 4 sides, as does each of the regions
// containing plants of type B, D, and E. However, the more complex region
// containing the plants of type C has 8 sides!
//
// Using the new method of calculating the per-region price by multiplying the
// region's area by its number of sides, regions A through E have prices 16, 16,
// 32, 4, and 12, respectively, for a total price of 80.
//
// The second example above (full of type X and O plants) would have a total
// price of 436.
//
// Here's a map that includes an E-shaped region full of type E plants:
//
//    EEEEE
//    EXXXX
//    EEEEE
//    EXXXX
//    EEEEE
//
// The E-shaped region has an area of 17 and 12 sides for a price of 204.
// Including the two regions full of type X plants, this map has a total price
// of 236.
//
// This map has a total price of 368:
//
//    AAAAAA
//    AAABBA
//    AAABBA
//    ABBAAA
//    ABBAAA
//    AAAAAA
//
// It includes two regions full of type B plants (each with 4 sides) and a
// single region full of type A plants (with 4 sides on the outside and 8 more
// sides on the inside, a total of 12 sides). Be especially careful when
// counting the fence around regions like the one full of type A plants; in
// particular, each section of fence has an in-side and an out-side, so the
// fence does not connect across the middle of the region (where the two B
// regions touch diagonally). (The Elves would have used the Möbius Fencing
// Company instead, but their contract terms were too one-sided.)
//
// The larger example from before now has the following updated prices:
//
// * A region of R plants with price 12 * 10 = 120.
// * A region of I plants with price 4 * 4 = 16.
// * A region of C plants with price 14 * 22 = 308.
// * A region of F plants with price 10 * 12 = 120.
// * A region of V plants with price 13 * 10 = 130.
// * A region of J plants with price 11 * 12 = 132.
// * A region of C plants with price 1 * 4 = 4.
// * A region of E plants with price 13 * 8 = 104.
// * A region of I plants with price 14 * 16 = 224.
// * A region of M plants with price 5 * 6 = 30.
// * A region of S plants with price 3 * 6 = 18.
//
// Adding these together produces its new total price of 1206.
//
// What is the new total price of fencing all regions on your map?
use std::io;
use std::io::Read;
use aoc2024::aoc::AocError;

fn main() -> Result<(), AocError> {
    let mut plots = Plots::parse_stdin()?;
    let mut cost = 0;
    for field in plots.find_fields() {
        cost += field.sides * field.area;
    }
    println!("{:?}", cost);
    return Ok(());
}

/// A field consists of an area and the number of sides of the field.
#[derive(PartialEq, Eq, Debug, Clone)]
struct Field {

    /// The area of the field.
    area: u64,

    /// Number of sides in the field.
    sides: u64,

}

impl Field {

    /// Construct a new field.
    fn new() -> Self {
        return Field {
            area: 0,
            sides: 0,
        };
    }
}

/// A plot is part of a field and grows something.
#[derive(PartialEq, Eq, Debug)]
struct Plot {

    /// Which crop is grown.
    crop: u8,

    /// If this plot is a part of a field, this is the ID of that field.
    field_id: Option<usize>,

}

impl Plot {

    /// Construct a new plot.
    fn new(crop: u8) -> Self {
        Plot {
            crop: crop,
            field_id: None,
        }
    }

}

/// Represents a grid of fields, a field is just a character representing some
/// type of food.
#[derive(PartialEq, Eq, Debug)]
struct Plots {

    /// Fields in a 2D grid.
    plots: Vec<Vec<Plot>>,

    /// Maximum horizontal index.
    max_x: usize,

    /// Maximum vertical index.
    max_y: usize,

}

impl Plots {

    /// Construct a new plots struct.
    fn new(plots: Vec<Vec<Plot>>) -> Result<Plots, AocError> {
        let max_y = plots.len();
        let max_x = if max_y == 0 { 0 } else { plots[0].len() };
        for line in &plots {
            if line.len() != max_x {
                return Err(AocError::ParseInputError(
                    "Expected all lines to be equal length.".into()));
            }
        }

        return Ok(Plots {
            plots: plots,
            max_x: max_x - 1,
            max_y: max_y - 1,
        });
    }

    /// Parse plots from the reader given.
    fn parse(reader: &mut dyn Read) -> Result<Plots, AocError> {
        let mut buffer = [0; 1024];
        let mut lines = Vec::new();
        let mut line = Vec::new();

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => {
                    if line.len() > 0 {
                        lines.push(line);
                    }
                    return Plots::new(lines);
                },
                Ok(bytes_read) => {
                    for i in 0..bytes_read {
                        let c = buffer[i];
                        if c == b'\n' && line.len() > 0 {
                            lines.push(line);
                            line = Vec::new();
                        } else if c != b'\n' {
                            line.push(Plot::new(c));
                        }
                    }
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }

    /// Read plots map from stdin.
    fn parse_stdin() -> Result<Plots, AocError> {
        let stdin = io::stdin();
        let mut stdin_lock = stdin.lock();
        return Plots::parse(&mut stdin_lock);
    }

    /// Loop through all plots and annotate each one as belonging to a field.
    ///
    /// Returns the number of fields found.
    fn annotate_plots(&mut self) -> usize {
        let mut field_n = 0;
        for y in 0..self.max_y + 1 {
            for x in 0..self.max_x + 1 {
                if self.plots[y][x].field_id.is_none() {
                    let crop = self.plots[y][x].crop;
                    self.annotate_plot_rec(x, y, field_n, crop);
                    field_n += 1;
                }
            }
        }

        return field_n;
    }

    /// Annotate the given plot position as field field_id if crop matches. Do
    /// so recursively for plot neighbours in order to color the whole field.
    fn annotate_plot_rec(&mut self, x: usize, y: usize, field_id: usize,
            crop: u8) {

        if self.plots[y][x].field_id.is_none() && self.plots[y][x].crop == crop {
            self.plots[y][x].field_id = Some(field_id);
            for (n_x, n_y) in self.neighbour_positions(x, y) {
                self.annotate_plot_rec(n_x, n_y, field_id, crop);
            }
        }
    }

    /// Annotate all plots as part of a field and return the list of computed
    /// fields.
    fn find_fields(&mut self) -> Vec<Field> {
        let field_count = self.annotate_plots();
        if field_count == 0 {
            return Vec::new();
        }
        let mut fields = vec![Field::new(); field_count];

        for y in 0..self.max_y + 1 {
            for x in 0..self.max_x + 1 {
                let field_id = self.plots[y][x].field_id
                    .expect("Should have been annotated before.");
                let field: &mut Field = &mut fields[field_id];
                field.area += 1;
                field.sides += self.count_corners(x, y);
            }
        }

        return fields;
    }

    /// Count the number of corners in the given position.
    fn count_corners(&self, x: usize, y: usize) -> u64 {
        let field_id = self.plots[y][x].field_id.unwrap();
        let mut corners = 0;
        if (x == 0 || !self.is_field((x - 1, y), field_id)) &&
            (y == 0 || !self.is_field((x, y - 1), field_id)) {
            corners += 1;
        }
        if !self.is_field((x + 1, y), field_id) &&
            (y == 0 || !self.is_field((x, y - 1), field_id)) {
            corners += 1;
        }
        if (x == 0 || !self.is_field((x - 1, y), field_id)) &&
            !self.is_field((x, y + 1), field_id) {
            corners += 1;
        }
        if !self.is_field((x + 1, y), field_id) &&
            !self.is_field((x, y + 1), field_id) {
            corners += 1;
        }
        if (x > 0 && self.is_field((x - 1, y), field_id)) &&
            (y > 0 && self.is_field((x, y - 1), field_id)) &&
            (x == 0 || y == 0 || !self.is_field((x - 1, y - 1), field_id)) {
            corners += 1;
        }
        if self.is_field((x + 1, y), field_id) &&
            (y > 0 && self.is_field((x, y - 1), field_id)) &&
            (y == 0 || !self.is_field((x + 1, y - 1), field_id)) {
            corners += 1;
        }
        if (x > 0 && self.is_field((x - 1, y), field_id)) &&
            self.is_field((x, y + 1), field_id) &&
            (x == 0 || !self.is_field((x - 1, y + 1), field_id)) {
            corners += 1;
        }
        if self.is_field((x + 1, y), field_id) &&
            self.is_field((x, y + 1), field_id) &&
            !self.is_field((x + 1, y + 1), field_id) {
            corners += 1;
        }

        return corners;
    }

    /// Test whether the given position matches the given field ID.
    fn is_field(&self, pos: (usize, usize), field_id: usize) -> bool {
        let (x, y) = pos;
        if x > self.max_x || y > self.max_y {
            return false;
        } else {
            return Some(field_id) == self.plots[y][x].field_id;
        }
    }

    /// Find positions of neighbouring plots.
    fn neighbour_positions(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::with_capacity(4);
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if x < self.max_x {
            neighbours.push((x + 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if y < self.max_y {
            neighbours.push((x, y + 1));
        }

        return neighbours;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that parsing plots works as expected.
    #[test]
    fn test_parse_valid() {
        let input = concat![
            "AAABB\n",
            "AABBB\n",
            "AABCC\n",
            "CCCCC\n",
            "CCCCC\n",
            "CCCCC\n",
            "AAAAB\n",
        ];
        let plots = Plots::parse(&mut input.as_bytes()).unwrap();

        assert_eq!(plots.max_x, 4);
        assert_eq!(plots.max_y, 6);
        assert_eq!(plots.plots[0], vec![
            Plot {
                crop: b'A',
                field_id: None,
            },
            Plot {
                crop: b'A',
                field_id: None,
            },
            Plot {
                crop: b'A',
                field_id: None,
            },
            Plot {
                crop: b'B',
                field_id: None,
            },
            Plot {
                crop: b'B',
                field_id: None,
            },
        ]);
        assert_eq!(plots.plots[3], vec![
            Plot {
                crop: b'C',
                field_id: None,
            },
            Plot {
                crop: b'C',
                field_id: None,
            },
            Plot {
                crop: b'C',
                field_id: None,
            },
            Plot {
                crop: b'C',
                field_id: None,
            },
            Plot {
                crop: b'C',
                field_id: None,
            },
        ]);
    }

    /// Test that parsing where one line is shorter gives error.
    #[test]
    fn test_parse_too_short_line() {
        let input = concat![
            "AAABB\n",
            "AABBB\n",
            "AABC\n",
            "CCCCC\n",
            "CCCCC\n",
            "CCCCC\n",
            "AAAAB\n",
        ];
        let err = Plots::parse(&mut input.as_bytes());
        assert!(err.is_err());
    }

    /// Test that parsing where one line is longer gives error.
    #[test]
    fn test_parse_too_long_line() {
        let input = concat![
            "AAABB\n",
            "AABBB\n",
            "AABCB\n",
            "CCCCC\n",
            "CCCCC\n",
            "CCCCC\n",
            "AAAABC\n",
        ];
        let err = Plots::parse(&mut input.as_bytes());
        assert!(err.is_err());
    }

    /// Test that finding fields works as expected.
    #[test]
    fn test_find_fields() {
        let input = concat![
            "AAABB\n",
            "AABBB\n",
            "AABCC\n",
            "CCDCC\n",
            "CCCCC\n",
            "CCCCC\n",
            "AAAAB\n",
        ];
        let mut plots = Plots::parse(&mut input.as_bytes()).unwrap();
        let fields = plots.find_fields();
        assert_eq!(fields.len(), 6);
        assert_eq!(fields[0].area, 7);
        assert_eq!(fields[0].sides, 6);
        assert_eq!(fields[1].area, 6);
        assert_eq!(fields[1].sides, 8);
        assert_eq!(fields[2].area, 16);
        assert_eq!(fields[2].sides, 8);
        assert_eq!(fields[3].area, 1);
        assert_eq!(fields[3].sides, 4);
        assert_eq!(fields[4].area, 4);
        assert_eq!(fields[4].sides, 4);
        assert_eq!(fields[5].area, 1);
        assert_eq!(fields[5].sides, 4);
    }

}
