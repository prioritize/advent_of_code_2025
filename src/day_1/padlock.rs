use anyhow::Result;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    usize,
};
#[derive(Debug)]
pub struct Rotation {
    dir: Direction,
    offset: usize,
}

impl Rotation {
    pub fn clamp_to_one_rotation(&mut self) {
        self.offset = get_tens_value(self.offset) * 10 + get_ones_value(self.offset)
    }
    pub fn new(dir: Direction, offset: usize) -> Self {
        let offset = clamp_to_one_rotation(offset);
        Self { dir, offset }
    }
}
#[derive(Debug)]
pub enum Direction {
    L,
    R,
}

#[derive(Default)]
pub struct Padlock {
    dial: VecDeque<u32>,
    current: usize,
}
fn parse_line_to_rotation(l: String) -> Rotation {
    let length = l.len();
    let dir = match l.chars().nth(0) {
        Some(f) => match f {
            'R' => Direction::R,
            'L' => Direction::L,
            _ => panic!("Got an incorrect direction"),
        },
        None => panic!("Found an empty line"),
    };
    let offset: usize = match length {
        0..2 => {
            panic!("Got a bad line")
        }

        2 => match l.chars().nth(1) {
            Some(x) => x.to_digit(10).unwrap() as usize,
            None => panic!("Got bad input"),
        },
        3.. => {
            let tens = l.chars().nth(length - 2).unwrap().to_digit(10).unwrap() as usize;
            let ones = l.chars().nth(length - 1).unwrap().to_digit(10).unwrap() as usize;
            tens * 10 + ones
        }
    };
    Rotation::new(dir, offset)
}
pub fn parse_day_1_input(f_name: &str) -> Result<Vec<Rotation>> {
    let file = File::open(f_name)?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|l| {
            if let Ok(line) = l {
                parse_line_to_rotation(line)
            } else {
                println!("Got bad input");
                panic!("Bad input");
            }
        })
        .collect::<Vec<Rotation>>();

    lines.iter().for_each(|x| println!("{x:?}"));
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
    pub fn get_next_location(&self, rot: Rotation) -> usize {
        match rot.dir {
            Direction::L => {
                // TODO: Handle the left case
                // TODO: Handle the left overflow case
                // TODO: Handle the left multiple case
                match self.current.checked_sub(rot.offset) {
                    Some(loc) => {
                        println!("left rotation after: {loc}");
                        loc
                    }
                    None => match rot.offset.checked_sub(self.current) {
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
            Direction::R => {
                // TODO: Handle the right case
                // TODO: Handle the right overflow case
                // TODO: Handle the right multiple case <- this is handled by the clamped
                // check
                match upper_clamp(self.current + rot.offset, 99) {
                    Some(loc) => loc,
                    None => {
                        let to_zero = 99 - self.current;
                        let remaining = rot.offset - to_zero;
                        println!(
                            "values: offset: {}, to_zero: {to_zero}, remaining: {remaining}",
                            rot.offset
                        );
                        remaining - 1
                    }
                }
            }
        }
    }
    pub fn rotate(&mut self, rot: Rotation) {
        let next = self.get_next_location(rot);
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
        assert_eq!(
            padlock.get_next_location(Rotation {
                dir: Direction::R,
                offset: 50
            }),
            0
        );
    }
    #[test]
    fn test_rollover() {
        let mut padlock = Padlock::new();
        padlock.current = 99;
        let rot = Rotation::new(Direction::R, 1);
        assert_eq!(padlock.get_next_location(rot), 0);
    }
    #[test]
    fn test_n_rollover() {
        let padlock = Padlock::new();
        let rot = Rotation::new(Direction::R, 200);
        let endpoint = padlock.get_next_location(rot);
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
    fn test_rotate_left() {
        let mut padlock = Padlock::new();
        padlock.current = 1;
        let rot = Rotation::new(Direction::L, 2);
        let location = padlock.get_next_location(rot);
        assert_eq!(location, 99);
    }
    #[test]
    fn test_rotate_left_1() {
        let padlock = Padlock::new();
        let rot = Rotation::new(Direction::L, 2);
        let location = padlock.get_next_location(rot);

        assert_eq!(location, 48);
    }
    #[test]
    fn test_rotate_left_3() {
        let padlock = Padlock::new();
        let rot = Rotation::new(Direction::L, 68);
        let location = padlock.get_next_location(rot);
        assert_eq!(location, 82);
    }
    #[test]
    fn test_read_input() -> Result<()> {
        let rotations = parse_day_1_input("input/day_1_input.txt")?;
        assert!(rotations.len() == 4059);
        Ok(())
    }
    #[test]
    fn test_read_test_input() -> Result<()> {
        let rotations = parse_day_1_input("input/day_1_test_input.txt")?;
        assert!(rotations.len() == 10);
        Ok(())
    }
}
