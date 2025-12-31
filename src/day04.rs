use itertools::iproduct;

pub fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}
const DIFFS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1, 0),           (1,  0),
    (-1, 1),  (0,  1), (1,  1),
];

pub fn part1(input: &str) -> u64 {
    let grid = parse_input(input);
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let accessible = iproduct!(0..height, 0..width).filter(|&(x, y)| {
        if !grid[y as usize][x as usize] {
            return false;
        }

        let adjacent: Vec<_> = DIFFS
            .iter()
            .filter_map(|&(dx, dy)| {
                let nx = usize::try_from(x + dx).ok()?;
                let ny = usize::try_from(y + dy).ok()?;
                Some(grid.get(ny)?.get(nx)?)
            })
            .filter(|&&cell| cell)
            .collect();

        adjacent.len() < 4
    });

    accessible.count() as u64
}

pub fn part2(input: &str) -> u64 {
    0
}
