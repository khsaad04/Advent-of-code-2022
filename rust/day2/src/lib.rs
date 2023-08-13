pub fn process_part1(input: &str) -> String {
    let result: u32 = input
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
        .sum();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: u32 = input
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
        .sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z
";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12");
    }
}
