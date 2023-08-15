use day5::{parse, process_part2};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = process_part2(parse(&input));

    println!("Part 2: {result}");
}
