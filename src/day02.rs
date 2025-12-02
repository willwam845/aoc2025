use std::cmp::{max, min};

pub fn parse_input(input: &str) -> Vec<(u64, u64, u32)> {
    input.split(",").flat_map(
        |range| {
            let (lbstr, ubstr) = range.split_once("-").unwrap();
            let lb: u64 = lbstr.parse().unwrap();
            let ub: u64 = ubstr.parse().unwrap();
            let lbdigits = lb.to_string().len();
            let ubdigits = ub.to_string().len();
            let mut res: Vec<(u64, u64, u32)> = vec![];
            for i in lbdigits..=ubdigits {
                let d = i as u32;
                res.push((
                    max(lb, (10 as u64).pow(d - 1)), 
                    min(ub, (10 as u64).pow(d) - 1), 
                    d
                ))
            };
            res
        }
    ).collect()
}

pub fn sum_n(n: u64) -> u64 {
    return (n * (n + 1)) / 2
}

pub fn part1(input: &str) -> u64 {
    // we turn all ranges into ranges with the same number of decimal digits
    let boundaries: Vec<(u64, u64, u32)> = parse_input(input);
    boundaries.iter().map(
        |(lb, ub, d)| {
            if d % 2 != 0 { return 0 }
            let inc = (10 as u64).pow(*d / 2) + 1;
            // sum of the multiples of inc there are between lb and ub (inclusive)
            let lower = if lb % inc == 0 { lb / inc } else { lb / inc + 1 };
            let upper = ub / inc;
            let res = inc * (sum_n(upper) - sum_n(lower - 1));
            res
        }
    ).sum::<u64>()
}

pub fn part2(input: &str) -> u64 {
    0
}
