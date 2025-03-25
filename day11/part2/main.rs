// --- Part Two ---
//
// The Historians sure are taking a long time. To be fair, the infinite
// corridors are very large.
//
// How many stones would you have after blinking a total of 75 times?
use std::io;
use std::io::Read;
use aoc2024::aoc::AocError;
use std::collections::HashMap;

fn main() -> Result<(), AocError> {
    let stdin = io::stdin();
    let mut lock = stdin.lock();
    let mut count = 0;
    let mut n: u64 = 0;
    let mut buffer = [0; 1024];
    let mut cache = HashMap::new();
    loop {
        match lock.read(&mut buffer) {
            Ok(0) => {
                println!("{:?}", count);
                return Ok(());
            },
            Ok(bytes_read) => {
                for i in 0..bytes_read {
                    let c = buffer[i];
                    if c >= b'0' && c <= b'9' {
                        let digit: u64 = (c - b'0').into();
                        n = n * 10 + digit;
                    } else if c == b' ' || c == b'\n' {
                        count += blink(n, 75, &mut cache);
                        n = 0;
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

/// Cache of intermediate results.
type Cache = HashMap<(u64, u64), u64>;

/// Count the number of stones after blinking 'count' times.
fn blink(stone: u64, count: u64, cache: &mut Cache) -> u64 {
    let stone_count = if count == 0 {
        1
    } else if let Some(sc) = cache.get(&(stone, count)) {
        *sc
    } else if stone == 0 {
        blink(1, count - 1, cache)
    } else {
        let str_stone = stone.to_string();
        let len = str_stone.len();
        if len % 2 == 0 {
            let left = str_stone[0..len / 2].parse().unwrap();
            let right = str_stone[len / 2..].parse().unwrap();
            blink(left, count - 1, cache) + blink(right, count - 1, cache)
        } else {
            blink(stone * 2024, count - 1, cache)
        }
    };

    cache.insert((stone, count), stone_count);
    return stone_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that blinking works as expected.
    #[test]
    fn test_blink() {
        assert_eq!(blink(0, 1, &mut HashMap::new()), 1);
        assert_eq!(blink(1, 1, &mut HashMap::new()), 1);
        assert_eq!(blink(1, 2, &mut HashMap::new()), 2);
        assert_eq!(blink(99, 2, &mut HashMap::new()), 2);
        assert_eq!(blink(1299, 1, &mut HashMap::new()), 2);
        assert_eq!(blink(1299, 2, &mut HashMap::new()), 4);
        assert_eq!(blink(1299, 3, &mut HashMap::new()), 4);
        assert_eq!(blink(1299, 4, &mut HashMap::new()), 6);
        assert_eq!(blink(1299, 5, &mut HashMap::new()), 12);
        assert_eq!(blink(125, 6, &mut HashMap::new())
                   + blink(17, 6, &mut HashMap::new()), 22);
        assert_eq!(blink(125, 25, &mut HashMap::new())
                   + blink(17, 25, &mut HashMap::new()), 55312);
    }

}
