pub fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input.lines().map(|line| {
        let (x_s, y_s) = line.split_once(",").unwrap();
        (x_s.parse().unwrap(), y_s.parse().unwrap())
    }).collect()
}

// o(n^2)
// https://per.austrin.se/icpc/finals2017solutions.pdf for an o(nlogn)
pub fn part1(input: &str) -> u64 {
    let points = parse_input(input);
    points.iter().map(|(x1, y1)| {
        points.iter().map(|(x2, y2)| {
            (1 + x1.abs_diff(*x2)) * (1 + y1.abs_diff(*y2))
        }).max().unwrap()
    }).max().unwrap()
}

pub fn part2(input: &str) -> u64 {
    0
}

