use day02::part1;
use std::{fs, io::Error};

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    let result = part1(&input);

    println!("Part 1: {result}");
    Ok(())
}
