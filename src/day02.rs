use gcd::Gcd;
use prime_factorization::Factorization;
use std::cmp::{max, min};

pub fn parse_input(input: &str) -> Vec<(u64, u64, u32)> {
    input
        .split(",")
        .flat_map(|range| {
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
                    d,
                ))
            }
            res
        })
        .collect()
}

pub fn sum_n(n: u64) -> u64 {
    return (n * (n + 1)) / 2;
}

pub fn part1(input: &str) -> u64 {
    // we turn all ranges into ranges with the same number of decimal digits
    let boundaries: Vec<(u64, u64, u32)> = parse_input(input);
    boundaries
        .iter()
        .map(|(lb, ub, d)| find_invalid(lb, ub, d, 2))
        .sum::<u64>()
}

pub fn is_prime_power(n: u32) -> bool {
    return [2, 3, 4, 5, 7, 8, 9].contains(&n);
}

pub fn find_invalid(lb: &u64, ub: &u64, d: &u32, reps: u32) -> u64 {
    if d % reps != 0 {
        return 0;
    }
    let replen = d / reps;
    let inc: u64 = (0..(d / replen)).map(|i| (10 as u64).pow(i * replen)).sum();
    let lower = if lb % inc == 0 {
        lb / inc
    } else {
        lb / inc + 1
    };
    let upper = ub / inc;
    inc * (sum_n(upper) - sum_n(lower - 1))
}

// we have the condition d <= 10
pub fn part2(input: &str) -> u64 {
    let boundaries: Vec<(u64, u64, u32)> = parse_input(input);
    boundaries
        .iter()
        .map(|(lb, ub, d)| {
            if *d == 1 {
                0
            } else if is_prime_power(*d) {
                let p = (2..=*d).find(|&i| d % i == 0).unwrap_or(*d);
                find_invalid(lb, ub, d, p)
            } else {
                // d = pq, since d <= 10
                let factors = (2..*d).filter(|fac| d % fac == 0);
                let pq = factors
                    .map(|reps| find_invalid(lb, ub, d, reps))
                    .sum::<u64>();
                pq - find_invalid(lb, ub, d, *d)
            }
        })
        .sum()
}

// attempted generalized solution...
pub fn part2_a(input: &str) -> u64 {
    let boundaries: Vec<(u64, u64, u32)> = parse_input(input);
    boundaries
        .iter()
        .map(|(lb, ub, d)| {
            if *d == 1 {
                return 0;
            };
            let facs = Factorization::run(*d).prime_factor_repr();
            let maximals: Vec<u32> = facs.iter().map(|(p, _n)| *d / p).collect();
            let n = maximals.len();
            let mut res = maximals
                .iter()
                .map(|bsize| find_invalid(lb, ub, d, *d / bsize))
                .sum::<u64>();

            // fake inclusion exclusion principle... the general solution would use it but properly
            for i in 0..n {
                for j in i + 1..n {
                    res -= find_invalid(lb, ub, d, d / maximals[i].gcd(maximals[j]));
                }
            }
            res
        })
        .sum()
}
