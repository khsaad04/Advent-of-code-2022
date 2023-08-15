pub fn process_part1(input: &str) -> String {
    let result: usize = input
        .as_bytes()
        .windows(4)
        .position(|w| {
            let mut arr = [0u8; 4];
            let mut idx = 0;
            for x in w {
                for i in 0..idx {
                    if arr[i] == *x {
                        return false;
                    }
                }
                arr[idx] = *x;
                idx += 1;
            }
            return true;
        })
        .map(|x| x + 4)
        .unwrap();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: usize = input
        .as_bytes()
        .windows(14)
        .position(|w| {
            let mut arr = [0u8; 14];
            let mut idx = 0;
            for x in w {
                for i in 0..idx {
                    if arr[i] == *x {
                        return false;
                    }
                }
                arr[idx] = *x;
                idx += 1;
            }
            return true;
        })
        .map(|x| x + 14)
        .unwrap();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "5");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "23");
    }
}
