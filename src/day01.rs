pub fn parse_input(input: &str) -> Vec<i64> {
    input
    .lines()
    .map(|line| {
        let (direction, value) = line.split_at(1);
        let dir: i64 = if direction == "R" { 1 } else { -1 };
        let val: i64 = value.parse().unwrap();
        dir * val
    })
    .collect()
}

pub fn part1(input: &str) -> u64 {
    let shifts = parse_input(input);
    let mut states: Vec<i64> = vec![];
    let mut state: i64 = 1000050; // lol
    shifts.iter().for_each(
        |shift| {
            state = state + shift;
            states.push(state)
        }
    );
    states.iter().filter(|state| { *state % 100 == 0 }).count() as u64
}

pub fn part2(input: &str) -> u64 {
    let shifts = parse_input(input);
    let mut state: i64 = 1000050;
    let mut solution: u64 = 0;
    shifts.iter().for_each(
        |shift| {
            // weird cases: 
            // if you start on a 0, we might shift from 00 -> 99 (left)
            // which results in a double count. subtract one in this case
            // if we end on a 0, we might have shifted from 01 -> 00 (left)
            // but we'll be in the same 100-interval. add one in this case

            if state % 100 == 0 && *shift < 0 { solution -= 1 }
            let prev_bound = state / 100;
            state = state + shift;
            let new_bound: i64 = state / 100;
            if state % 100 == 0 && *shift < 0 { solution += 1 }
            solution += (new_bound - prev_bound).abs() as u64;
        }
    );
    solution
}
