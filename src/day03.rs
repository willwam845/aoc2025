pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - '0' as u8).collect())
        .collect()
}

pub fn pick_digit(digits: &[u8], remaining: usize) -> (u8, usize) {
    let valid_digits = &digits[..digits.len() - remaining];
    let mut max_digit = valid_digits[0];
    let mut max_pos = 0usize;
    for (pos, digit) in valid_digits.iter().enumerate() {
        if *digit > max_digit {
            max_digit = *digit;
            max_pos = pos;
        }
    }
    (max_digit, max_pos)
}

// claim: we want to pick greedily. i.e. when we have picked the best possible first digit,
// then we pick greedily from the remaining digits. we also pick the earliest occurence.
// provided that there are enough digits remaining to pick from, this will always work

pub fn solve(digits: &[u8], to_pick: usize) -> u64 {
    let mut total = 0u64;
    let mut digits = digits;
    for rem_digits in (0..to_pick).rev() {
        let (digit, rem) = pick_digit(digits, rem_digits);
        total = total * 10 + digit as u64;
        digits = &digits[rem + 1..]
    }
    total
}

pub fn part1(input: &str) -> u64 {
    let lines = parse_input(input);
    lines.iter().map(|line| solve(&line[..], 2usize)).sum()
}

pub fn part2(input: &str) -> u64 {
    let lines = parse_input(input);
    lines.iter().map(|line| solve(&line[..], 12usize)).sum()
}
