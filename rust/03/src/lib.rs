use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
    let letters = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>();

    let result: usize = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let common_char = a.chars().find(|c| b.contains(*c)).unwrap();
            letters.get(&common_char).unwrap()
        })
        .sum();

    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let letters = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>();

    let groups: Vec<_> = input.lines().collect();
    let groups: Vec<_> = groups.chunks(3).collect();

    let result: usize = groups
        .iter()
        .map(|group| {
            let common_char = group[0]
                .chars()
                .find(|c: &char| group[1].contains(*c) && group[2].contains(*c))
                .unwrap();
            letters.get(&common_char).unwrap()
        })
        .sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "70");
    }
}
