// --- Day 12: Garden Groups ---
//
// Why not search for the Chief Historian near the gardener and his massive
// farm? There's plenty of food, so The Historians grab something to eat while
// they search.
//
// You're about to settle near a complex arrangement of garden plots when some
// Elves ask if you can lend a hand. They'd like to set up fences around each
// region of garden plots, but they can't figure out how much fence they need to
// order or how much it will cost. They hand you a map (your puzzle input) of
// the garden plots.
//
// Each garden plot grows only a single type of plant and is indicated by a
// single letter on your map. When multiple garden plots are growing the same
// type of plant and are touching (horizontally or vertically), they form a
// region. For example:
//
//    AAAA
//    BBCD
//    BBCC
//    EEEC
//
// This 4x4 arrangement includes garden plots growing five different types of
// plants (labeled A, B, C, D, and E), each grouped into their own region.
//
// In order to accurately calculate the cost of the fence around a single
// region, you need to know that region's area and perimeter.
//
// The area of a region is simply the number of garden plots the region
// contains. The above map's type A, B, and C plants are each in a region of
// area 4. The type E plants are in a region of area 3; the type D plants are in
// a region of area 1.
//
// Each garden plot is a square and so has four sides. The perimeter of a region
// is the number of sides of garden plots in the region that do not touch
// another garden plot in the same region. The type A and C plants are each in a
// region with perimeter 10. The type B and E plants are each in a region with
// perimeter 8. The lone D plot forms its own region with perimeter 4.
//
// Visually indicating the sides of plots in each region that contribute to the
// perimeter using - and |, the above map's regions' perimeters are measured as
// follows:
//
//    +-+-+-+-+
//    |A A A A|
//    +-+-+-+-+     +-+
//                  |D|
//    +-+-+   +-+   +-+
//    |B B|   |C|
//    +   +   + +-+
//    |B B|   |C C|
//    +-+-+   +-+ +
//              |C|
//    +-+-+-+   +-+
//    |E E E|
//    +-+-+-+
//
// Plants of the same type can appear in multiple separate regions, and regions
// can even appear within other regions. For example:
//
//    OOOOO
//    OXOXO
//    OOOOO
//    OXOXO
//    OOOOO
//
// The above map contains five regions, one containing all of the O garden
// plots, and the other four each containing a single X plot.
//
// The four X regions each have area 1 and perimeter 4. The region containing 21
// type O plants is more complicated; in addition to its outer edge contributing
// a perimeter of 20, its boundary with each X region contributes an additional
// 4 to its perimeter, for a total perimeter of 36.
//
// Due to "modern" business practices, the price of fence required for a region
// is found by multiplying that region's area by its perimeter. The total price
// of fencing all regions on a map is found by adding together the price of
// fence for every region on the map.
//
// In the first example, region A has price 4 * 10 = 40, region B has price
// 4 * 8 = 32, region C has price 4 * 10 = 40, region D has price 1 * 4 = 4, and
// region E has price 3 * 8 = 24. So, the total price for the first example is
// 140.
//
// In the second example, the region with all of the O plants has price
// 21 * 36 = 756, and each of the four smaller X regions has price 1 * 4 = 4,
// for a total price of 772 (756 + 4 + 4 + 4 + 4).
//
// Here's a larger example:
//
//    RRRRIICCFF
//    RRRRIICCCF
//    VVRRRCCFFF
//    VVRCCCJFFF
//    VVVVCJJCFE
//    VVIVCCJJEE
//    VVIIICJJEE
//    MIIIIIJJEE
//    MIIISIJEEE
//    MMMISSJEEE
//
// It contains:
//
// * A region of R plants with price 12 * 18 = 216.
// * A region of I plants with price 4 * 8 = 32.
// * A region of C plants with price 14 * 28 = 392.
// * A region of F plants with price 10 * 18 = 180.
// * A region of V plants with price 13 * 20 = 260.
// * A region of J plants with price 11 * 20 = 220.
// * A region of C plants with price 1 * 4 = 4.
// * A region of E plants with price 13 * 18 = 234.
// * A region of I plants with price 14 * 22 = 308.
// * A region of M plants with price 5 * 12 = 60.
// * A region of S plants with price 3 * 8 = 24.
//
// So, it has a total price of 1930.
//
// What is the total price of fencing all regions on your map?
use std::io;
use std::io::Read;
use aoc2024::aoc::AocError;

fn main() -> Result<(), AocError> {
    let mut plots = Plots::parse_stdin()?;
    let mut cost = 0;
    for field in plots.find_fields() {
        cost += field.perimeter * field.area;
    }
    println!("{:?}", cost);
    return Ok(());
}

/// A field consists of a perimeter and an area.
#[derive(PartialEq, Eq, Debug, Clone)]
struct Field {

    /// The perimeter of the field.
    perimeter: u64,

    /// The area of the field.
    area: u64,

}

impl Field {

    /// Construct a new field.
    fn new() -> Self {
        return Field {
            perimeter: 0,
            area: 0
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
                let crop = self.plots[y][x].crop;
                let neighbours = self.neighbours(x, y);
                let mut perimeter: u64 = (4 - neighbours.len()).try_into().unwrap();
                let field_id = self.plots[y][x].field_id.unwrap();
                for neighbour in neighbours {
                    if crop != neighbour.crop {
                        perimeter += 1;
                    }
                }

                let field: &mut Field = &mut fields[field_id];
                field.area += 1;
                field.perimeter += perimeter;
            }
        }

        return fields;
    }

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

    /// Find list of neighbours of the given location.
    fn neighbours(&self, x: usize, y: usize) -> Vec<&Plot> {
        let mut neighbours = Vec::with_capacity(4);
        if x > 0 {
            neighbours.push(&self.plots[y][x - 1]);
        }
        if x < self.max_x {
            neighbours.push(&self.plots[y][x + 1]);
        }
        if y > 0 {
            neighbours.push(&self.plots[y - 1][x]);
        }
        if y < self.max_y {
            neighbours.push(&self.plots[y + 1][x]);
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

    /// Test that finding neighbours in the middle of a plot collection works.
    #[test]
    fn test_neighbours_middle() {
        let input = concat![
            "AAABB\n",
            "AABBB\n",
            "AABCC\n",
            "CCDCC\n",
            "CCCCC\n",
            "CCCCC\n",
            "AAAAB\n",
        ];
        let plots = Plots::parse(&mut input.as_bytes()).unwrap();

        assert_eq!(plots.neighbours(1, 1), vec![
            &Plot {
                crop: b'A',
                field_id: None,
            },
            &Plot {
                crop: b'B',
                field_id: None,
            },
            &Plot {
                crop: b'A',
                field_id: None,
            },
            &Plot {
                crop: b'A',
                field_id: None,
            },
        ]);

        assert_eq!(plots.neighbours(2, 2), vec![
            &Plot {
                crop: b'A',
                field_id: None,
            },
            &Plot {
                crop: b'C',
                field_id: None,
            },
            &Plot {
                crop: b'B',
                field_id: None,
            },
            &Plot {
                crop: b'D',
                field_id: None,
            },
        ]);
    }

    /// Test that finding neighbours on the border works.
    #[test]
    fn test_neighbours_border() {
        let input = concat![
            "AAABB\n",
            "AABBB\n",
            "AABCC\n",
            "CCDCC\n",
            "CCCCC\n",
            "CCCCC\n",
            "AAAAB\n",
        ];
        let plots = Plots::parse(&mut input.as_bytes()).unwrap();

        assert_eq!(plots.neighbours(2, 0), vec![
            &Plot {
                crop: b'A',
                field_id: None,
            },
            &Plot {
                crop: b'B',
                field_id: None,
            },
            &Plot {
                crop: b'B',
                field_id: None,
            },
        ]);

        assert_eq!(plots.neighbours(0, 6), vec![
            &Plot {
                crop: b'A',
                field_id: None,
            },
            &Plot {
                crop: b'C',
                field_id: None,
            },
        ]);
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
    }

}
