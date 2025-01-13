use nom::{
    Parser,
    IResult,
    multi::many1,
    character::complete::{one_of},
    combinator::{map_res, recognize}
};

/// Parse an unsigned integer from the input.
///
/// Will return the parsed integer together with any remaining input.
pub fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(recognize(many1(one_of("0123456789"))),
        |ns: &str| u32::from_str_radix(ns, 10)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that parsing a number works as expected.
    #[test]
    fn test_parse_u32_valid_number() {
        let parsed = parse_u32("123whatever");
        assert_eq!(parsed, Ok(("whatever", 123)));
    }

    /// Test that empty numbers don't work.
    #[test]
    fn test_parse_u32_empty_number() {
        let parsed = parse_u32("whatever");
        assert!(parsed.is_err());
    }

}
