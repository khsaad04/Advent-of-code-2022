use itertools::Itertools;

type Stack = Vec<Vec<char>>;
type Move = (usize, usize, usize);
type Input = (Stack, Vec<Move>);

pub fn parse(input: &str) -> Input {
    let (stack_str, moves_str) = input.split_once("\n\n").unwrap();

    let mut stack_iter = stack_str.lines().rev();
    let mut stack = vec![vec![]; stack_iter.next().unwrap().len() / 4 + 1];

    stack_iter.for_each(|l| {
        l.chars().skip(1).enumerate().for_each(|(i, c)| {
            if i % 4 == 0 && c != ' ' {
                stack[i / 4].push(c);
            }
        });
    });

    let moves = moves_str
        .lines()
        .filter_map(|l| {
            let s: Vec<&str> = l.split_ascii_whitespace().collect();
            Some((s[1].parse().ok()?, s[3].parse().ok()?, s[5].parse().ok()?))
        })
        .collect();

    (stack, moves)
}

fn move_stacks(stack: &mut Stack, moves: &[Move], version: u16) {
    moves.iter().for_each(|(amount, from, to)| {
        let from = &mut stack[*from - 1];
        let crates = from.split_off(from.len() - amount);
        if version == 9000 {
            stack[*to - 1].extend(crates.iter().rev());
        } else {
            stack[*to - 1].extend(crates.iter());
        }
    });
}

fn get_top_row(stack: &Stack) -> String {
    stack.iter().filter_map(|m| m.last()).join("")
}

pub fn part1(input: Input) -> String {
    let (mut stack, moves) = input;
    move_stacks(&mut stack, &moves, 9000);
    Some(get_top_row(&stack)).unwrap()
}

pub fn part2(input: Input) -> String {
    let (mut stack, moves) = input;
    move_stacks(&mut stack, &moves, 9001);
    Some(get_top_row(&stack)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{parse, part1, part2};

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
    fn test_part1() {
        let result = part1(parse(INPUT));
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part2() {
        let result = part2(parse(INPUT));
        assert_eq!(result, "MCD");
    }
}
