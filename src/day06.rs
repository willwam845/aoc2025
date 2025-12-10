#[derive(Debug)]
pub enum ProbElem {
    Plus,
    Times
}

pub fn part1(input: &str) -> u64 {
    let (value_lines, operation_line) = input.rsplit_once("\n").unwrap();
    let operations: Vec<ProbElem> = operation_line.split_ascii_whitespace().map(|op| {
        match op {
            "+" => ProbElem::Plus,
            "*" => ProbElem::Times,
            _ => unreachable!()
        }
    }).collect();
    let values: Vec<u64> = value_lines.split_ascii_whitespace().map(|v| { v.parse::<u64>().unwrap() }).collect();
    let ops: &[ProbElem] = &operations[..];
    let vals = &values[..];
    let line_count = value_lines.lines().count();
    let problem_count = vals.len() / line_count;
    (0..problem_count).map(|i| {
        let problem_vals = (0..line_count).map(|l| { vals[i + problem_count * l] });
        let op = &ops[i];
        let f = match op {
            ProbElem::Plus => |x, y| { x + y },
            ProbElem::Times => |x, y| { x * y }
        };
        problem_vals.reduce(f).unwrap()
    }).sum()
}

pub fn part2(input: &str) -> u64 {
    0
}
