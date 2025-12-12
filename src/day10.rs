
use core::num;

use nalgebra::{DMatrix, DVector, Dyn};

#[derive(Debug)]
pub struct Machine {
    pub lights: usize,
    pub init: u16,
    pub buttons: Vec<u16>,
    pub joltages: Vec<f64>
}

pub fn parse_input(input: &str) -> Vec<Machine> {
    input.lines().map(|line| {
        let (init_str, rest) = line.split_once(" ").unwrap();
        let (buttons_str, joltages_str) = rest.rsplit_once(" ").unwrap();
        
        let init = {
            let mut init_chars = init_str.chars();
            init_chars.next(); init_chars.next_back();
            init_chars.enumerate().map(|(i, x)| {
                ((x == '#') as u16) * 1 << i
            }).sum()
        };

        let buttons = buttons_str.split_ascii_whitespace().into_iter().map(|b_str| {
            let mut chars = b_str.chars();   
            chars.next(); chars.next_back();
            chars.as_str().split(",").into_iter().map(|bit| {
                1 << bit.parse::<u8>().unwrap()
            }).sum()
        }).collect();

        let joltages = {
            let mut chars = joltages_str.chars();   
            chars.next(); chars.next_back();
            chars.as_str().split(",").into_iter().map(|v| { v.parse::<f64>().unwrap() }).collect()
        };
        
        Machine {
            lights: init_str.len() - 2,
            init,
            buttons,
            joltages
        }
    }).collect()

}

pub fn part1(input: &str) -> u64 {
    let machines = parse_input(input);
    machines.iter().map(|machine| {
        find_dependent_combinations(&machine.buttons, machine.init)
        .into_iter().map(|i| { (i as u64).count_ones() as u64 }).min().unwrap()
    }).sum()
}

pub fn to_bits(v: u16, n: usize) -> Vec<f64> {
    (0..n).map(|i| { (v >> i & 1) as f64 }).collect()
}

pub fn find_dependent_combinations(values: &Vec<u16>, initial: u16) -> Vec<u16> {
    let vals = &values[..];
    let n = vals.len();
     (1..1 << n).filter(|i| {
        (0..n).map(|bit| {
            if i & 1 << bit > 0 { vals[bit] }
            else { 0 } 
        }).fold(initial, |x, y| { x ^ y }) == 0
     }).map(|x| { x as u16 }).collect()
}

// a hot mess of attempts
// VAGUE IDEA: we want to try and find linearly independent sets of the buttons
// then we can try and solve a matrix equation
// however, since we need to find the smallest, we need to consider all of the linearly
// independent sets

// i thought we could be smart and see what the linear relations are to determine what subsets are valid
// but apparently that doesn't actually work
pub fn part2(input: &str) -> u64 {
    let machines = parse_input(input);
    machines.iter().map(|machine| {
        let bits = machine.lights;

        println!("{:?}", machine);
        let linear_relations = find_dependent_combinations(&machine.buttons, 0);
        let buttons = machine.buttons.clone().into_iter().map(|v| { 
            to_bits(v, bits) 
        });
        let num_buttons = buttons.len();
        let bad_dims = (usize::BITS - (linear_relations.len()).leading_zeros()) as usize;
        let good_dims = num_buttons - bad_dims;

        let b: nalgebra::Matrix<f64, Dyn, nalgebra::Const<1>, nalgebra::VecStorage<f64, Dyn, nalgebra::Const<1>>> = DVector::from_iterator(bits, machine.joltages.clone());
        ((1 << good_dims) - 1..1 << num_buttons).map(|k| {
            let row_count = (k as u32).count_ones() as usize;
            let rows = buttons.clone().enumerate().filter(|(i, _row)| {
                k & 1 << i > 0
            }).map(|(_i, row)| { row }).flatten();
            let a_m = DMatrix::from_row_iterator_generic(
                Dyn(row_count), Dyn(bits), rows.into_iter()
            ).transpose();
            let decomp = a_m.clone().svd(true, true);
            let x = decomp.solve(&b, 1e-10).unwrap();
            let a_mx = a_m * x.clone();
            (a_mx, x)
        }).filter(|(am_x, x)| {
            x.iter().all(|x_val| { 
                (x_val.round() - x_val).abs() < 1e-2 && x_val.round() >= 0.0
            }) && am_x.iter().zip(&b).map(|(a_v, b_v)| {
                (a_v - b_v).abs()
            }).sum::<f64>() < 1.0
        }).map(|(_a_mx, x)| {
            x.map(|x_val| { x_val.round() as u64 }).sum()
        }).min().unwrap()
    }).sum()
}
