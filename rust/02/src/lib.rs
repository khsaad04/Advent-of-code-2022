pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|turn| {
            let moves: Vec<&str> = turn.split(' ').collect();
            match moves[0] {
                "A" => match moves[1] {
                    "X" => 1 + 3,
                    "Y" => 2 + 6,
                    "Z" => 3,
                    _ => 0,
                },
                "B" => match moves[1] {
                    "X" => 1,
                    "Y" => 2 + 3,
                    "Z" => 3 + 6,
                    _ => 0,
                },
                "C" => match moves[1] {
                    "X" => 1 + 6,
                    "Y" => 2,
                    "Z" => 3 + 3,
                    _ => 0,
                },
                _ => 0,
            }
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|turn| {
            let moves: Vec<&str> = turn.split(' ').collect();
            match moves[0] {
                "A" => match moves[1] {
                    "X" => 3,
                    "Y" => 1 + 3,
                    "Z" => 2 + 6,
                    _ => 0,
                },
                "B" => match moves[1] {
                    "X" => 1,
                    "Y" => 2 + 3,
                    "Z" => 3 + 6,
                    _ => 0,
                },
                "C" => match moves[1] {
                    "X" => 2,
                    "Y" => 3 + 3,
                    "Z" => 1 + 6,
                    _ => 0,
                },
                _ => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "A Y
B X
C Z
";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 12);
    }
}
