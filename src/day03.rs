use regex::Regex;

pub fn part_one(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    re.captures_iter(input)
        .map(|caps| {
            let (s, [x, y]) = caps.extract();

            println!("{}-{}-{}", s, x, y);

            let x: i32 = x.parse().unwrap();
            let y: i32 = y.parse().unwrap();

            x * y
        })
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|(do\(\))|(don't\(\))").unwrap();

    re.captures_iter(input).map(|caps| {
        let (s, x): (&str, [&str; N]) = caps.extract();

        println!("{}--{:?}", s, x);
    });
    0
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
