use std::iter::repeat;

pub fn part1(input: &str) -> u64 {
    let (value_lines, operation_line) = input.rsplit_once("\n").unwrap();
    let operations: Vec<&str> = operation_line.split_ascii_whitespace().collect();
    let values: Vec<u64> = value_lines.split_ascii_whitespace().map(|v| { v.parse::<u64>().unwrap() }).collect();
    let ops = &operations[..];
    let vals = &values[..];
    let line_count = value_lines.lines().count();
    let problem_count = vals.len() / line_count;
    (0..problem_count).map(|i| {
        let problem_vals = (0..line_count).map(|l| { vals[i + problem_count * l] });
        let op = ops[i];
        let f = match op {
            "+" => |x, y| { x + y },
            "*" => |x, y| { x * y },
            _ => unreachable!()
        };
        problem_vals.reduce(f).unwrap()
    }).sum()
}

pub fn part2(input: &str) -> u64 {
    let (value_lines, operation_line) = input.rsplit_once("\n").unwrap();
    let operations= operation_line.split_ascii_whitespace().into_iter();
    let line_length = operation_line.len();
    let mut values: Vec<_> = repeat(0u64).take(line_length).collect();
    value_lines.lines().for_each(|line| {
        values = line.chars().zip(values.iter()).map(|(i, v)| {
            if i != ' ' { v * 10 + i.to_digit(10).unwrap() as u64 } 
            else { *v }
        }).collect()
    });

    values.split(|v| *v == 0).into_iter().zip(operations).map(|(vals, op)| {
        let f = match op {
            "+" => |x, y| { x + y },
            "*" => |x, y| { x * y },
            _ => unreachable!()
        };
        vals.iter().copied().reduce(f).unwrap()
    }).sum()
}
