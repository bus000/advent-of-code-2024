// --- Part Two ---
// The Elf looks quizzically at you. Did you misunderstand the assignment?

// Looking for the instructions, you flip over the word search to find that this
// isn't actually an XMAS puzzle; it's an X-MAS puzzle in which you're supposed
// to find two MAS in the shape of an X. One way to achieve that is like this:
//
//   M.S
//   .A.
//   M.S
//
// Irrelevant characters have again been replaced with . in the above diagram.
// Within the X, each MAS can be written forwards or backwards.
//
// Here's the same example from before, but this time all of the X-MASes have
// been kept instead:
//
//    .M.S......
//    ..A..MSMS.
//    .M.S.MAA..
//    ..A.ASMSM.
//    .M.S.M....
//    ..........
//    S.S.S.S.S.
//    .A.A.A.A..
//    M.M.M.M.M.
//    ..........
//
// In this example, an X-MAS appears 9 times.
//
// Flip the word search from the instructions back over to the word search side
// and try again. How many times does an X-MAS appear?
use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    match read_input() {
        Ok(input) => {
            println!("{:?}", count_x_mas(input));
        },
        Err(err) => {
            eprintln!("Could not read input: {:?}", err);
            return ExitCode::from(1);
        }
    }

    return ExitCode::from(0);
}

/// Count instances of the MAS strings in an X shape. Like this:
///
///    M.S
///    .A.
///    M.S
fn count_x_mas(array: Vec<Vec<u8>>) -> u32 {
    let rows = array.len();
    if rows < 3 {
        return 0;
    }
    let columns = array[0].len();
    if columns < 3 {
        return 0;
    }

    let mut count = 0;
    for i in 0..rows-2 {
        for j in 0..columns-2 {
            if is_sam(array[i][j], array[i+1][j+1], array[i+2][j+2]) &&
                is_sam(array[i+2][j], array[i+1][j+1], array[i][j+2]) {
                count = count + 1;
            }
        }
    }

    return count;
}

fn is_sam(c1: u8, c2: u8, c3: u8) -> bool {
    return (c1 == b'M' && c2 == b'A' && c3 == b'S') ||
        (c1 == b'S' && c2 == b'A' && c3 == b'M');
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that counting single X-MAS works.
    #[test]
    fn test_count_x_mas_single() {
        let input = vec![
            "M.S\n".bytes().collect(),
            ".A.\n".bytes().collect(),
            "M.S\n".bytes().collect()
        ];

        assert_eq!(count_x_mas(input), 1);
    }

    /// Test that counting multiple X-MAS works.
    #[test]
    fn test_count_x_mas_multiple() {
        let input = vec![
            ".M.S......\n".bytes().collect(),
            "..A..MSMS.\n".bytes().collect(),
            ".M.S.MAA..\n".bytes().collect(),
            "..A.ASMSM.\n".bytes().collect(),
            ".M.S.M....\n".bytes().collect(),
            "..........\n".bytes().collect(),
            "S.S.S.S.S.\n".bytes().collect(),
            ".A.A.A.A..\n".bytes().collect(),
            "M.M.M.M.M.\n".bytes().collect(),
            "..........\n".bytes().collect()
        ];

        assert_eq!(count_x_mas(input), 9);
    }

}
