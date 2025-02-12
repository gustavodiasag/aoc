use regex::Regex;

pub fn part_one(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    re.captures_iter(input)
        .map(|caps| {
            let (_, [x, y]) = caps.extract();

            Op::Mul(x.parse().unwrap(), y.parse().unwrap())
        })
        .map(|op| op.calculate())
        .sum()
}

enum Op {
    Mul(i32, i32),
}

impl Op {
    fn calculate(&self) -> i32 {
        match self {
            Op::Mul(x, y) => x * y,
        }
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
        )
    }
}
