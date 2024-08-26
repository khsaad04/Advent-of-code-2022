use day5::{part1, parse};
use std::{fs, io::Error};

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    let result = part1(parse(&input));

    println!("Part 1: {result}");
    Ok(())
}
