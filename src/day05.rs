pub fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (range_str, item_str) = input.split_once("\n\n").unwrap();
    let ranges = range_str.lines().map(|line| {
        let (lb_s, ub_s) = line.split_once("-").unwrap();
        (lb_s.parse().unwrap(),  ub_s.parse().unwrap())
    }).collect();

    let items = item_str.lines().map(|line| {
        line.parse().unwrap()
    }).collect();

    (ranges, items)
}

pub fn non_overlapping_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut new_ranges = vec![];
    ranges.sort_by(|(l1, u1), (l2, u2)|{
        l1.cmp(l2)
    });
    let mut prev_u = 0u64;
    for (l, u) in ranges.iter() {
        // just need to compare against the previous range....
        // we have the guarantee that prev_l < l
        // the iffy cases are containment and overlap
        if prev_u < *l {
            // disjoint. we can add the new range as is
            new_ranges.push((*l, *u));
            prev_u = *u
        } else if prev_u >= *u {
            // new range is contained within the previous. ignore.
        } else {
            // overlap. we have that prev_l < l < prev_u < u
            // since the interval prev_l - prev_u is already in,
            // we want to take the interval prev_u + 1, u
            new_ranges.push((prev_u + 1, *u));
            prev_u = *u
        }
    }
    new_ranges
}

pub fn query_ranges(ranges: &[(u64, u64)], val: u64) -> bool {
    let n = ranges.len();
    let mut lb = 0usize;
    let mut ub = n - 1;
    // binsearch so annoying...
    while ub > lb {
        let mp = (lb + ub) / 2;
        let (l, u) = ranges[mp];
        if l <= val && val <= u { return true }
        else if val < l { if mp == 0 { return false } else { ub = mp - 1 } }
        else { if mp == ub { return false } else { lb = mp + 1 } }
    }

    let (l, u) = ranges[lb];
    l <= val && val <= u
}

pub fn part1(input: &str) -> u64 {
    let (ranges, items) = parse_input(input);
    let new_ranges = non_overlapping_ranges(ranges);
    items.into_iter().filter(|item| {
        let res = query_ranges(&new_ranges, *item);
        res
    }).count() as u64
}

pub fn part2(input: &str) -> u64 {
    let (ranges, _items) = parse_input(input);
    let new_ranges = non_overlapping_ranges(ranges);
    new_ranges.into_iter().map(|(l, u)| { u - l + 1}).sum()
}
