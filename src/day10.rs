use std::{f64::MAX, iter::zip};

use itertools::Itertools;
use peroxide::fuga::*;

#[derive(Debug)]
pub struct Machine {
    pub lights: usize,
    pub init: u16,
    pub buttons: Vec<u16>,
    pub joltages: Vec<f64>,
}

pub fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let (init_str, rest) = line.split_once(" ").unwrap();
            let (buttons_str, joltages_str) = rest.rsplit_once(" ").unwrap();

            let init = {
                let mut init_chars = init_str.chars();
                init_chars.next();
                init_chars.next_back();
                init_chars
                    .enumerate()
                    .map(|(i, x)| ((x == '#') as u16) * 1 << i)
                    .sum()
            };

            let buttons = buttons_str
                .split_ascii_whitespace()
                .into_iter()
                .map(|b_str| {
                    let mut chars = b_str.chars();
                    chars.next();
                    chars.next_back();
                    chars
                        .as_str()
                        .split(",")
                        .into_iter()
                        .map(|bit| 1 << bit.parse::<u8>().unwrap())
                        .sum()
                })
                .collect();

            let joltages = {
                let mut chars = joltages_str.chars();
                chars.next();
                chars.next_back();
                chars
                    .as_str()
                    .split(",")
                    .into_iter()
                    .map(|v| v.parse::<f64>().unwrap())
                    .collect()
            };

            Machine {
                lights: init_str.len() - 2,
                init,
                buttons,
                joltages,
            }
        })
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let machines = parse_input(input);
    machines
        .iter()
        .map(|machine| {
            find_dependent_combinations(&machine.buttons, machine.init)
                .into_iter()
                .map(|i| (i as u64).count_ones() as u64)
                .min()
                .unwrap()
        })
        .sum()
}

pub fn to_bits(v: u16, n: usize) -> Vec<f64> {
    (0..n).map(|i| (v >> i & 1) as f64).collect()
}

pub fn find_dependent_combinations(values: &Vec<u16>, initial: u16) -> Vec<u16> {
    let vals = &values[..];
    let n = vals.len();
    (1..1 << n)
        .filter(|i| {
            (0..n)
                .map(|bit| if i & 1 << bit == 0 { 0 } else { vals[bit] })
                .fold(initial, |x, y| x ^ y)
                == 0
        })
        .map(|x| x as u16)
        .collect()
}

pub fn is_integer(v: f64) -> bool {
    (v - v.round()).abs() < 1e-2
}

pub fn is_integer_vector(vec: Vec<f64>) -> bool {
    let error: f64 = vec.iter().map(|v| (v - v.round()).abs()).sum();
    error < 1e-2
}

pub fn part2(input: &str) -> u64 {
    let machines = parse_input(input);
    machines
        .iter()
        .map(|machine: &Machine| {
            println!("{machine:?}");
            let bits = machine.lights;
            let buttons: Vec<Vec<_>> = machine
                .buttons
                .clone()
                .into_iter()
                .map(|v| to_bits(v, bits))
                .collect();
            let num_buttons = buttons.len();
            let a = py_matrix(buttons).t();
            let a_aug: Vec<Vec<_>> = machine
                .joltages
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let mut r = a.row(i);
                    r.push(*v);
                    r
                })
                .collect();

            let mut a_rref = py_matrix(a_aug).rref();

            let mut free_button_indices = vec![];
            let mut curr_target = 0usize;
            for i in 0..bits {
                let row = a_rref.row(i);
                while (curr_target < num_buttons) && ((row[curr_target] - 1.0).abs() > 1e-5) {
                    free_button_indices.push(curr_target);
                    curr_target += 1;
                }

                let k: f64 = (1..1000)
                    .find(|k| {
                        let modified_row: Vec<_> = row.iter().map(|x| (*k as f64) * x).collect();
                        is_integer_vector(modified_row)
                    })
                    .unwrap_or(1) as f64;

                if k > 1.0 {
                    let modified_row: Vec<_> = row.iter().map(|x| (k * x).round()).collect();
                    a_rref.subs_row(i, &modified_row[..]);
                }
                curr_target += 1;
            }
            for i in curr_target..num_buttons {
                free_button_indices.push(i)
            }

            let max_presses: Vec<_> = {
                let free_buttons = free_button_indices.iter().map(|i| machine.buttons[*i]);
                free_buttons
                    .map(|v| {
                        let bits = to_bits(v, bits);
                        let max_presses = machine
                            .joltages
                            .iter()
                            .zip(bits)
                            .filter(|(_joltage, bit)| *bit as u8 == 1)
                            .map(|(joltage, _bit)| *joltage as u16)
                            .min()
                            .unwrap_or(0);
                        max_presses
                    })
                    .collect()
            };

            let ranges: Box<dyn Iterator<Item = _>> = if max_presses.is_empty() {
                Box::new(std::iter::once(vec![]))
            } else {
                Box::new(max_presses.iter().map(|&n| 0..n).multi_cartesian_product())
            };

            let mut minimum_presses: f64 = MAX;
            let _brute_size: usize = ranges.try_len().unwrap();

            for combo in ranges {
                let mut press_counts = vec![0.0; num_buttons];
                for (i, v) in zip(free_button_indices.iter(), combo) {
                    press_counts[*i] = v as f64
                }

                for i in (0..bits).rev() {
                    let row = a_rref.row(i);
                    let (target_button, times) = row
                        .iter()
                        .enumerate()
                        .find(|(_, coeff)| coeff.abs() > 1e-2)
                        .unwrap_or((num_buttons, &0.0));

                    if target_button == num_buttons {
                        continue;
                    }

                    let target: f64 = row[num_buttons]
                        - (target_button + 1..num_buttons)
                            .map(|b| row[b] * press_counts[b])
                            .sum::<f64>();
                    press_counts[target_button] = target / times;
                }

                let total_presses: f64 = press_counts.iter().sum();
                if press_counts.iter().all(|x| x.round() >= 0.0)
                    && is_integer_vector(press_counts.clone())
                {
                    minimum_presses = minimum_presses.min(total_presses.round())
                }
            }
            
            println!("{minimum_presses:?}");
            minimum_presses as u64
        })
        .sum()
}
