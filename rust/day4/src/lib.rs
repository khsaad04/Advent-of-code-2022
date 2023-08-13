pub fn process_part1(input: &str) -> String {
    let parts: Vec<_> = input.lines().map(|line| line.split(',')).collect();
    "69".to_string()
}

pub fn process_part2(input: &str) -> String {
    "69".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "");
    }
}
