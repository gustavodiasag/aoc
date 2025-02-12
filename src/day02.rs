type Input = Vec<Vec<i32>>;

pub fn parse_input(input: &str) -> Input {
    use core::str::FromStr;

    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(i32::from_str)
                .collect::<Result<_, _>>()
                .expect("unable to parse line")
        })
        .collect()
}

pub fn part_one(reports: &Input) -> usize {
    reports.iter().filter(|r| is_safe_report(r)).count()
}

pub fn part_two(reports: &Input) -> usize {
    reports
        .iter()
        .filter(|r| is_safe_report(r) || is_safe_with_tolerance(r))
        .count()
}

fn is_safe_report(report: &[i32]) -> bool {
    let diff = report[0] - report[1];
    if !(1..=3).contains(&diff.abs()) {
        return false;
    }

    for i in 1..report.len() - 1 {
        let d = report[i] - report[i + 1];
        if !(1..=3).contains(&d.abs()) || d.signum() != diff.signum() {
            return false;
        }
    }
    true
}

fn is_safe_with_tolerance(report: &[i32]) -> bool {
    (0..report.len()).into_iter().any(|i| {
        let mut report = report.to_vec();
        report.remove(i);
        is_safe_report(&report)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!(
            part_one(&vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ]),
            2
        );
    }

    #[test]
    fn part_two_example() {
        assert_eq!(
            part_two(&vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ]),
            4
        );
    }
}
