use itertools::iproduct;

pub fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let grid = parse_input(input);
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;
    
    let diffs: Vec<(isize, isize)> = iproduct!(-1..=1, -1..=1)
        .filter(|(x, y)| *x != 0 || *y != 0)
        .collect();

    let accessible: Vec<_> = iproduct!(0..height, 0..width)
        .filter(|(x, y)| {
            if !grid[*y as usize][*x as usize] {
                return false;
            }

            let adjacent: Vec<_> = diffs
                .iter()
                .filter(|(dx, dy)| {
                    let nx: isize = x + *dx;
                    let ny: isize = y + *dy;
                    if !(0..width).contains(&nx) || !(0..height).contains(&ny) {
                        return true;
                    }

                    !grid[ny as usize][nx as usize]
                })
                .collect();
            adjacent.len() > 4
        })
        .collect();

    accessible.len() as u64
}

pub fn part2(_input: &str) -> u64 {
    0
}
