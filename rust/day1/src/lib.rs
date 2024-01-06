pub fn process_part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

pub fn process_part2(input: &str) -> i32 {
    let mut result: Vec<i32> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|item| item.parse::<i32>().unwrap()).sum())
        .collect();
    result.sort();
    result.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 24000);
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, 45000);
    }
}
