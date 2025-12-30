pub fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map({ |c| c == '@' }).collect())
        .collect()
}

pub fn part1(_input: &str) -> u64 {
    0
}

pub fn part2(_input: &str) -> u64 {
    0
}
