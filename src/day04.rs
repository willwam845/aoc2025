use std::collections::{HashMap, HashSet};

use itertools::iproduct;

pub fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

const DIFFS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub fn part1(input: &str) -> u64 {
    let grid = parse_input(input);
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let accessible = iproduct!(0..height, 0..width).filter(|&(x, y)| {
        if !grid[y as usize][x as usize] {
            return false;
        }

        let adjacent = DIFFS
            .iter()
            .filter_map(|&(dx, dy)| {
                let nx = usize::try_from(x + dx).ok()?;
                let ny = usize::try_from(y + dy).ok()?;
                Some(grid.get(ny)?.get(nx)?)
            })
            .filter(|&&cell| cell);

        adjacent.count() < 4
    });

    accessible.count() as u64
}

pub fn part2(input: &str) -> u64 {
    let grid = parse_input(input);
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let rolls: HashSet<_> = iproduct!(0..height, 0..width)
        .filter(|&(x, y)| grid[y as usize][x as usize])
        .collect();

    let initial_counts = rolls.iter().map(|&(x, y)| {
        let adjacent = DIFFS
            .iter()
            .filter_map(|&(dx, dy)| {
                let nx = usize::try_from(x + dx).ok()?;
                let ny = usize::try_from(y + dy).ok()?;
                Some(grid.get(ny)?.get(nx)?)
            })
            .filter(|&&cell| cell);
        ((x, y), adjacent.count())
    });

    let mut adjacent_map: HashMap<(isize, isize), usize> = HashMap::from_iter(initial_counts);
    let mut answer = 0usize;
    loop {
        let to_remove: HashSet<_> = adjacent_map
            .iter()
            .filter(|&(&_pos, &v)| v < 4)
            .map(|(&pos, _v)| pos)
            .collect();

        answer += to_remove.len();
        if to_remove.len() == 0 {
            break;
        };

        for (x, y) in to_remove {
            for (dx, dy) in DIFFS {
                let pos = (x + dx, y + dy);
                adjacent_map.get_mut(&pos).map(|x| *x -= 1);
            }

            adjacent_map.remove(&(x, y));
        }
    }

    answer as u64
}
