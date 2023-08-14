use std::ops::RangeInclusive;

pub fn process_part1(input: &str) -> String {
    let result: usize = input
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
            if a_range.contains(&b1) && a_range.contains(&b2) {
                1
            } else if b_range.contains(&a1) && b_range.contains(&a2) {
                1
            } else {
                0
            }
        })
        .sum();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: usize = input
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
        .sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "4");
    }
}
