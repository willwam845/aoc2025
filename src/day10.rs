
pub struct Machine {
    pub init: u16,
    pub buttons: Vec<u16>,
    pub joltages: Vec<u64>
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
            chars.as_str().split(",").into_iter().map(|v| { v.parse::<u64>().unwrap() }).collect()
        };
        
        Machine {
            init,
            buttons,
            joltages
        }
    }).collect()

}

pub fn part1(input: &str) -> u64 {
    let machines = parse_input(input);
    machines.iter().map(|machine| {
        let button_changes = &machine.buttons[..];
        let button_count: usize = button_changes.len();

        (0..1 << button_count).filter(|i| {
            (0..button_count).map(|bit| {
                if i & 1 << bit > 0 { button_changes[bit] }
                else { 0 } 
            }).fold(machine.init, |x, y| { x ^ y }) == 0
        }).map(|i| { (i as u64).count_ones() as u64 }).min().unwrap()        // things.min().unwrap()
    }).sum()
}

pub fn part2(input: &str) -> u64 {
    0
}
