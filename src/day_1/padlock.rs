use std::collections::VecDeque;
pub enum Rotation {
    L,
    R,
}
#[derive(Default)]
pub struct Padlock {
    dial: VecDeque<u32>,
    current: usize,
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
        let clamped = match offset.checked_rem(self.dial.len()) {
            Some(v) => v,
            None => {
                println!("Rotated a full turn");
                return self.current;
            }
        };
        match dir {
            Rotation::L => {
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
            Rotation::R => {
                // TODO: Handle the right case
                // TODO: Handle the right overflow case
                // TODO: Handle the right multiple case <- this is handled by the clamped
                // check
                match upper_clamp(self.current + offset, 99) {
                    Some(loc) => loc,
                    None => {
                        let to_zero = 99 - self.current;
                        self.current - to_zero - 1
                    }
                }
            }
        }
    }
    pub fn rotate(&mut self, offset: usize) -> u32 {
        todo!()
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

#[cfg(test)]
mod tests {
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
        assert_eq!(padlock.get_next_location(50, Rotation::R), 0);
    }
    #[test]
    fn test_rollover() {
        let mut padlock = Padlock::new();
        padlock.current = 99;
        assert_eq!(padlock.get_next_location(1, Rotation::R), 0);
    }
}
