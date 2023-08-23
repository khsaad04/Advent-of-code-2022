pub fn process_part1(input: &str) -> String {
    let edge = &input.lines().count() * 2 + input.lines().last().unwrap().len() - 4;
    let result = 0;
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result = 0;
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "");
    }
}
