pub fn parse(input: &mut std::str::Lines) -> Vec<u64> {
    let (mut total, mut subdirs) = (0, vec![]);
    loop {
        match input
            .next()
            .map(|s| s.split_whitespace().collect::<Vec<_>>())
            .as_deref()
        {
            Some(["$", "cd", ".."]) | None => break,
            Some(["$", "cd", dir]) if *dir != "/" => {
                subdirs.extend(parse(input));
                total += subdirs.last().unwrap();
            }
            Some([s, _]) if *s != "$" && *s != "dir" => {
                total += s.parse::<u64>().unwrap();
            }
            _ => (),
        }
    }
    subdirs.push(total);
    subdirs
}

pub fn process_part1(input: &str) -> String {
    let result: u64 = parse(&mut input.lines())
        .into_iter()
        .filter(|&s| s <= 100_000)
        .sum();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut sizes = parse(&mut input.lines());
    let missing = 30_000_000 - (70_000_000 - sizes.last().unwrap());
    sizes.sort_unstable();
    let result = sizes.into_iter().find(|&s| s >= missing).unwrap();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "95437");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "24933642");
    }
}
