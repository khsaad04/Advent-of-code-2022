type Stack = Vec<Vec<char>>;

struct Instruction {
    From: u32,
    Amount: u32,
    To: u32,
}

fn parse_to(input: &str) -> (Stack, Instruction) {
    let (stack, instructions) = input.split_once("\n\n").unwrap();
}

pub fn process_part1(input: &str) -> String {
    "".to_string()
}

pub fn process_part2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "");
    }
}
