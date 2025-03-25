// --- Day 11: Plutonian Pebbles ---
//
// The ancient civilization on Pluto was known for its ability to manipulate
// spacetime, and while The Historians explore their infinite corridors, you've
// noticed a strange set of physics-defying stones.
//
// At first glance, they seem like normal stones: they're arranged in a
// perfectly straight line, and each stone has a number engraved on it.
//
// The strange part is that every time you blink, the stones change.
//
// Sometimes, the number engraved on a stone changes. Other times, a stone might
// split in two, causing all the other stones to shift over a bit to make room
// in their perfectly straight line.
//
// As you observe them for a while, you find that the stones have a consistent
// behavior. Every time you blink, the stones each simultaneously change
// according to the first applicable rule in this list:
//
// * If the stone is engraved with the number 0, it is replaced by a stone
//   engraved with the number 1.
// * If the stone is engraved with a number that has an even number of digits,
//   it is replaced by two stones. The left half of the digits are engraved on
//   the new left stone, and the right half of the digits are engraved on the
//   new right stone. (The new numbers don't keep extra leading zeroes: 1000
//   would become stones 10 and 0.)
// * If none of the other rules apply, the stone is replaced by a new stone; the
//   old stone's number multiplied by 2024 is engraved on the new stone.
//
// No matter how the stones change, their order is preserved, and they stay on
// their perfectly straight line.
//
// How will the stones evolve if you keep blinking at them? You take a note of
// the number engraved on each stone in the line (your puzzle input).
//
// If you have an arrangement of five stones engraved with the numbers 0 1 10 99
// 999 and you blink once, the stones transform as follows:
//
// * The first stone, 0, becomes a stone marked 1.
// * The second stone, 1, is multiplied by 2024 to become 2024.
// * The third stone, 10, is split into a stone marked 1 followed by a stone
//   marked 0.
// * The fourth stone, 99, is split into two stones marked 9.
// * The fifth stone, 999, is replaced by a stone marked 2021976.
//
// So, after blinking once, your five stones would become an arrangement of
// seven stones engraved with the numbers 1 2024 1 0 9 9 2021976.
//
// Here is a longer example:
//
//    Initial arrangement:
//    125 17
//
//    After 1 blink:
//    253000 1 7
//
//    After 2 blinks:
//    253 0 2024 14168
//
//    After 3 blinks:
//    512072 1 20 24 28676032
//
//    After 4 blinks:
//    512 72 2024 2 0 2 4 2867 6032
//
//    After 5 blinks:
//    1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32
//
//    After 6 blinks:
//    2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2
//
// In this example, after blinking six times, you would have 22 stones. After
// blinking 25 times, you would have 55312 stones!
//
// Consider the arrangement of stones in front of you. How many stones will you
// have after blinking 25 times?
use std::io;
use std::io::Read;
use aoc2024::aoc::AocError;

fn main() -> Result<(), AocError> {
    let stdin = io::stdin();
    let mut lock = stdin.lock();
    let mut count = 0;
    let mut n: u64 = 0;
    let mut buffer = [0; 1024];
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
                        count += blink(n, 25);
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

/// Count the number of stones after blinking 'count' times.
fn blink(stone: u64, count: u64) -> u64 {
    if count == 0 {
        return 1;
    } else if stone == 0 {
        return blink(1, count - 1);
    }
    let str_stone = stone.to_string();
    let len = str_stone.len();
    if len % 2 == 0 {
        let left = str_stone[0..len / 2].parse().unwrap();
        let right = str_stone[len / 2..].parse().unwrap();
        return blink(left, count - 1) + blink(right, count - 1);
    } else {
        return blink(stone * 2024, count - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that blinking works as expected.
    #[test]
    fn test_blink() {
        assert_eq!(blink(0, 1), 1);
        assert_eq!(blink(1, 1), 1);
        assert_eq!(blink(1, 2), 2);
        assert_eq!(blink(99, 2), 2);
        assert_eq!(blink(1299, 1), 2);
        assert_eq!(blink(1299, 2), 4);
        assert_eq!(blink(1299, 3), 4);
        assert_eq!(blink(1299, 4), 6);
        assert_eq!(blink(1299, 5), 12);
        assert_eq!(blink(125, 6) + blink(17, 6), 22);
        assert_eq!(blink(125, 25) + blink(17, 25), 55312);
    }

}
