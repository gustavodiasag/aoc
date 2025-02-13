pub fn part_one(mut input: &str) -> i32 {
    let mut total = 0;

    while !input.is_empty() {
        if let Ok((remain, mul)) = parser::parse_mul(input) {
            total += mul;
            input = remain;
        } else {
            input = &input[1..];
        }
    }

    total
}

pub fn part_two(mut input: &str) -> i32 {
    let mut total = 0;
    let mut enabled = true;

    while !input.is_empty() {
        if enabled {
            if let Ok((remain, mul)) = parser::parse_mul(input) {
                total += mul;
                input = remain;
                continue;
            }
        }

        if let Ok((remain, cond)) = parser::parse_cond(input) {
            match cond {
                parser::DO => enabled = true,
                parser::DONT => enabled = false,
                _ => unreachable!(),
            }
            input = &remain;
        } else {
            input = &input[1..];
        }
    }

    total
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while},
        AsChar, IResult, Parser,
    };

    pub const DO: &'static str = "do()";
    pub const DONT: &'static str = "don't()";

    pub fn parse_mul(input: &str) -> IResult<&str, i32> {
        let (input, _) = tag("mul(")(input)?;
        let (input, lhs) = parse_num(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, rhs) = parse_num(input)?;
        let (input, _) = tag(")")(input)?;

        Ok((input, lhs * rhs))
    }

    pub fn parse_num(input: &str) -> IResult<&str, i32> {
        let (input, num) = take_while(AsChar::is_dec_digit)(input)?;

        Ok((input, num.parse().unwrap()))
    }

    pub fn parse_cond(input: &str) -> IResult<&str, &str> {
        alt((tag(DO), tag(DONT))).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!(
            part_one("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn part_two_example() {
        assert_eq!(
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
