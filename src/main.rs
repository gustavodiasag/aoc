use std::env;

use aoc::day03;

fn main() {
    let input = read_input(env::args()).unwrap();

    println!("{}", day03::part_one(&input));
}

fn read_input(mut args: impl Iterator<Item = String>) -> Result<String, &'static str> {
    args.next();

    let path = match args.next() {
        Some(path) => path,
        None => return Err("file does not exist"),
    };

    let input = std::fs::read_to_string(path).unwrap();

    Ok(input)
}
