pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let mut result: Vec<_> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|item| item.parse::<usize>().unwrap()).sum())
        .collect();
    result.sort();
    result.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 45000);
    }
}
