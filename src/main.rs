use std::env;

use aoc::day02;

fn main() {
    let input = read_input(env::args()).unwrap();

    let input = aoc::day02::parse_input(&input);
    println!("{}", day02::part_two(&input));
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
