use day5::{parse, process_part1};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = process_part1(parse(&input));

    println!("Part 1: {result}");
}
