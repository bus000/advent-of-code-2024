// --- Day 3: Mull It Over ---
//
// "Our computers are having issues, so I have no idea if we have any Chief
// Historians in stock! You're welcome to check the warehouse, though," says the
// mildly flustered shopkeeper at the North Pole Toboggan Rental Shop. The
// Historians head out to take a look.
//
// The shopkeeper turns to you. "Any chance you can see why our computers are
// having issues again?"
//
// The computer appears to be trying to run a program, but its memory (your
// puzzle input) is corrupted. All of the instructions have been jumbled up!
//
// It seems like the goal of the program is just to multiply some numbers. It
// does that with instructions like mul(X,Y), where X and Y are each 1-3 digit
// numbers. For instance, mul(44,46) multiplies 44 by 46 to get a result of
// 2024. Similarly, mul(123,4) would multiply 123 by 4.
//
// However, because the program's memory has been corrupted, there are also many
// invalid characters that should be ignored, even if they look like part of a 
// mul instruction. Sequences like mul(4*, mul(6,9!, ?(12,34), or mul ( 2 , 4 )
// do nothing.
//
// For example, consider the following section of corrupted memory:
//
//     xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
//
// Only the four highlighted sections are real mul instructions. Adding up the
// result of each instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).
//
// Scan the corrupted memory for uncorrupted mul instructions. What do you get
// if you add up all of the results of the multiplications?
use std::io;
use regex::Regex;

fn main() {
    let mut total_sum = 0;
    for line in io::stdin().lines() {
        total_sum = total_sum + sum_of_muls(&line.unwrap());
    }

    println!("{:?}", total_sum);
}

// Extract the multiplications, compute them and return the sum.
fn sum_of_muls(string : &str) -> u32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut product_sum = 0;
    for (_, [n1_str, n2_str]) in re.captures_iter(string).map(|c| c.extract()) {
        let n1 = u32::from_str_radix(n1_str, 10).unwrap();
        let n2 = u32::from_str_radix(n2_str, 10).unwrap();
        product_sum = product_sum + n1 * n2;
    }

    return product_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that we don't crash on empty strings.
    #[test]
    fn test_sum_of_muls_empty() {
        assert_eq!(0, sum_of_muls(""));
    }

    /// Test that we don't make up phantom muls.
    #[test]
    fn test_sum_of_muls_no_phantoms() {
        assert_eq!(0, sum_of_muls("whatever but no muls here. mul mul(123n)"));
    }

    /// Test that we can find a normal mul.
    #[test]
    fn test_sum_of_muls_middle() {
        assert_eq!(20, sum_of_muls("whatever but no muls here. mul(4,5)rsfd"));
    }

    /// Test that we can find a mul even if it is last.
    #[test]
    fn test_sum_of_muls_last() {
        assert_eq!(20, sum_of_muls("whatever but no muls here. mul(4,5)"));
    }

    /// Test that we can find a mul even if it is first.
    #[test]
    fn test_sum_of_muls_first() {
        assert_eq!(20, sum_of_muls("mul(4,5)whatever but no muls here."));
    }

    /// Test that we can find multiple muls and will sum them.
    #[test]
    fn test_sum_of_muls_multiple() {
        assert_eq!(79, sum_of_muls("mul(4,5)whatever mul(1,5) mul(6,9)"));
    }

}
