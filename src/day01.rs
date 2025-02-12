use std::collections::HashMap;

type Input = (Vec<u32>, Vec<u32>);

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let l = parts.next().unwrap().parse::<u32>().unwrap();
            let r = parts.next().unwrap().parse::<u32>().unwrap();

            (l, r)
        })
        .unzip()
}

pub fn part_one(input: &Input) -> u32 {
    let (mut left, mut right) = input.clone();

    left.sort_unstable();
    right.sort_unstable();

    left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum()
}

pub fn part_two(input: &Input) -> u32 {
    let (left, right) = input;

    let mut frequency = HashMap::with_capacity(right.capacity());

    right
        .iter()
        .for_each(|n| *frequency.entry(n).or_insert(0) += 1);

    left.iter()
        .filter_map(|l| frequency.get(l).map(|f| l * f))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!(
            part_one(&(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])),
            11
        );
    }

    #[test]
    fn part_two_example() {
        assert_eq!(
            part_two(&(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])),
            31
        );
    }
}
