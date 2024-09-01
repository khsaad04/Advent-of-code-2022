use std::collections::HashMap;

fn get_letters() -> HashMap<char, usize> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>()
}

pub fn part1(input: &str) -> usize {
    let letters = get_letters();
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let common_char = a.chars().find(|c| b.contains(*c)).unwrap();
            letters.get(&common_char).unwrap()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let letters = get_letters();

    let groups: Vec<_> = input.lines().collect();
    let groups: Vec<_> = groups.chunks(3).collect();

    groups
        .iter()
        .map(|group| {
            let common_char = group[0]
                .chars()
                .find(|c: &char| group[1].contains(*c) && group[2].contains(*c))
                .unwrap();
            letters.get(&common_char).unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 70);
    }
}
