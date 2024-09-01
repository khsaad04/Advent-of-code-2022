use std::ops::RangeInclusive;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let ranges: Vec<_> = line.split(',').collect();
            let mut a = ranges[0].split('-');
            let mut b = ranges[1].split('-');
            let a1: usize = a.next().unwrap().parse().unwrap();
            let a2: usize = a.next().unwrap().parse().unwrap();
            let b1: usize = b.next().unwrap().parse().unwrap();
            let b2: usize = b.next().unwrap().parse().unwrap();
            let a_range: RangeInclusive<usize> = a1..=a2;
            let b_range: RangeInclusive<usize> = b1..=b2;
            if a_range.contains(&b1) && a_range.contains(&b2) {
                1
            } else if b_range.contains(&a1) && b_range.contains(&a2) {
                1
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let ranges: Vec<_> = line.split(',').collect();
            let mut a = ranges[0].split('-');
            let mut b = ranges[1].split('-');
            let a1 = a.next().unwrap().parse::<u32>().unwrap();
            let a2 = a.next().unwrap().parse::<u32>().unwrap();
            let b1 = b.next().unwrap().parse::<u32>().unwrap();
            let b2 = b.next().unwrap().parse::<u32>().unwrap();
            let a_range: RangeInclusive<u32> = a1..=a2;
            let b_range: RangeInclusive<u32> = b1..=b2;
            if a_range.contains(&b1) | a_range.contains(&b2) {
                1
            } else if b_range.contains(&a1) | b_range.contains(&a2) {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 4);
    }
}
