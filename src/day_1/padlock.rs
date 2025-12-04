use anyhow::Result;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};
pub enum Rotation {
    L { offset: u32 },
    R { offset: u32 },
}
#[derive(Default)]
pub struct Padlock {
    dial: VecDeque<u32>,
    current: usize,
}
pub fn parse_day_1_input(f_name: &str) -> Result<Vec<Rotation>> {
    let file = File::open(f_name)?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|l| {
            if let Ok(line) = l {
                let dir = line.chars().nth(0).unwrap();
                let length = line.len();
                let tens = line.chars().nth(length - 2).unwrap();
                let ones = line.chars().nth(length - 1).unwrap();
                println!("{length} {tens} {ones}");
                match dir {
                    'R' => Rotation::R { offset: 0 },
                    'L' => Rotation::L { offset: 0 },
                    _ => panic!("Bad input"),
                }
            } else {
                println!("Got bad input");
                panic!("Bad input");
            }
        })
        .collect::<Vec<Rotation>>();
    Ok(lines)
}

impl Padlock {
    pub fn new() -> Self {
        let mut vd = VecDeque::new();

        for i in 0..100 {
            vd.insert(i, i.try_into().unwrap());
        }
        Padlock {
            dial: vd,
            current: 50,
        }
    }
    pub fn get_next_location(&self, offset: usize, dir: Rotation) -> usize {
        let offset = clamp_to_one_rotation(offset);
        match dir {
            Rotation::L { offset: _ } => {
                // TODO: Handle the left case
                // TODO: Handle the left overflow case
                // TODO: Handle the left multiple case
                match self.current.checked_sub(offset) {
                    Some(loc) => loc,
                    None => match offset.checked_sub(self.current) {
                        Some(of) => self.dial.len() - of,
                        None => {
                            println!(
                                "We shouldn't be getting here! We've failed on both directions"
                            );
                            panic!("Failed in rotating left");
                        }
                    },
                }
            }
            Rotation::R { offset: _ } => {
                // TODO: Handle the right case
                // TODO: Handle the right overflow case
                // TODO: Handle the right multiple case <- this is handled by the clamped
                // check
                match upper_clamp(self.current + offset, 99) {
                    Some(loc) => loc,
                    None => {
                        let to_zero = 99 - self.current;
                        let remaining = offset - to_zero;
                        println!(
                            "values: offset: {offset}, to_zero: {to_zero}, remaining: {remaining}"
                        );
                        remaining - 1
                    }
                }
            }
        }
    }
    pub fn rotate(&mut self, offset: usize, dir: Rotation) {
        let next = self.get_next_location(offset, dir);
        self.current = next;
    }
}
// This function takes an input and evaluates it against the upper bound.
// This function primarily preserves the API of the get_next_location function
pub fn upper_clamp(input: usize, upper_bound: usize) -> Option<usize> {
    match input > upper_bound {
        true => None,
        false => Some(input),
    }
}
pub fn get_tens_value(input: usize) -> usize {
    let inter = input / 10;
    inter % 10
}
pub fn get_ones_value(input: usize) -> usize {
    input % 10
}
pub fn clamp_to_one_rotation(input: usize) -> usize {
    get_tens_value(input) * 10 + get_ones_value(input)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;
    #[test]
    fn test_padlock_construction() {
        let padlock = Padlock::new();
        assert_eq!(padlock.current, 50);
        assert_eq!(padlock.dial[padlock.current], 50);
    }
    #[test]
    fn test_get_next_location() {
        let padlock = Padlock::new();
        assert_eq!(padlock.get_next_location(50, Rotation::R { offset: 50 }), 0);
    }
    #[test]
    fn test_rollover() {
        let mut padlock = Padlock::new();
        padlock.current = 99;
        assert_eq!(padlock.get_next_location(1, Rotation::R { offset: 1 }), 0);
    }
    #[test]
    fn test_n_rollover() {
        let padlock = Padlock::new();
        let endpoint = padlock.get_next_location(200, Rotation::R { offset: 200 });
        assert_eq!(endpoint, 50);
    }
    #[test]
    fn test_gets_tens() {
        assert_eq!(get_tens_value(2764), 6);
    }
    #[test]
    fn test_get_ones() {
        assert_eq!(get_ones_value(2764), 4);
    }
    #[test]
    fn test_get_ones_1() {
        assert_eq!(get_ones_value(65535), 5);
    }
    #[test]
    fn test_clamp() {
        assert_eq!(clamp_to_one_rotation(999), 99);
    }
    #[test]
    fn test_clamp_2() {
        assert_eq!(clamp_to_one_rotation(0), 0);
    }
    #[test]
    fn test_clamp_3() {
        assert_eq!(clamp_to_one_rotation(79087345), 45);
    }
    #[test]
    fn test_read_input() -> Result<()> {
        let rotations = parse_day_1_input("input/day_1_input.txt")?;
        assert!(rotations.len() == 4059);
        Ok(())
    }
}
