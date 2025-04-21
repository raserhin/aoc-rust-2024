use std::cmp::Ordering;

advent_of_code::solution!(2);

const MAX_STEP: u64 = 3;

#[derive(Debug)]
struct Record {
    ord: Ordering,
    last: u64,
    current: u64,
}

impl Record {
    fn new(last: u64, current: u64) -> Self {
        Self {
            ord: current.cmp(&last),
            last,
            current,
        }
    }

    fn insert_and_validate(&mut self, new: u64) -> bool {
        self.last = self.current;
        self.current = new;
        self.is_valid_state()
    }

    fn is_valid_state(&self) -> bool {
        if self.current.cmp(&self.last) != self.ord {
            return false;
        }
        match self.ord {
            Ordering::Less => self.last.wrapping_sub(self.current) <= MAX_STEP,
            Ordering::Greater => self.current.wrapping_sub(self.last) <= MAX_STEP,
            Ordering::Equal => false,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    for line in input.lines().map(|line| line.split_ascii_whitespace()) {
        result += 1;
        let mut iter = line.map(|num| str::parse(num).unwrap());
        let mut record = Record::new(iter.next().unwrap(), iter.next().unwrap());
        if !record.is_valid_state() {
            result -= 1;
            continue;
        }
        for num in iter {
            if !record.insert_and_validate(num) {
                result -= 1;
                break;
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    for line in input.lines().map(|line| line.split_ascii_whitespace()) {
        result += 1;
        let mut iter = line.map(|num| str::parse(num).unwrap());
        let mut record = Record::new(iter.next().unwrap(), iter.next().unwrap());
        if !record.is_valid_state() {
            result -= 1;
            continue;
        }
        for num in iter {
            if !record.insert_and_validate(num) {
                result -= 1;
                break;
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
