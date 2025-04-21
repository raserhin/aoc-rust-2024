advent_of_code::solution!(1);

use std::collections::{BinaryHeap, HashMap};

pub fn part_one(input: &str) -> Option<u64> {
    let mut left = BinaryHeap::with_capacity(1000);
    let mut right = BinaryHeap::with_capacity(1000);
    for (left_val, right_val) in input
        .lines()
        .map(|line| line.split_once("   ").expect("Could not split whitespace"))
    {
        left.push(str::parse(left_val).ok()?);
        right.push(str::parse(right_val).ok()?);
    }

    let mut result = 0u64;
    while let Some((l, r)) = left.pop().zip(right.pop()) {
        result += u64::max(l, r) - u64::min(l, r);
    }

    Some(result)
}

struct Counter {
    left: u64,
    right: u64,
}

impl Counter {
    fn new(left: u64, right: u64) -> Self {
        Self { left, right }
    }

    fn new_l() -> Self {
        Self::new(1, 0)
    }
    fn new_r() -> Self {
        Self::new(0, 1)
    }

    fn incr_l(&mut self) {
        self.left += 1
    }

    fn incr_r(&mut self) {
        self.right += 1
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map: HashMap<u64, Counter> = HashMap::with_capacity(1000);
    for (left_val, right_val) in input
        .lines()
        .map(|line| line.split_once("   ").expect("Could not split whitespace"))
    {
        let l: u64 = str::parse(left_val).ok()?;
        let r: u64 = str::parse(right_val).ok()?;
        map.entry(l)
            .and_modify(|c| c.incr_l())
            .or_insert_with(|| Counter::new_l());
        map.entry(r)
            .and_modify(|c| c.incr_r())
            .or_insert_with(|| Counter::new_r());
    }

    let mut result = 0;

    for (number, counter) in map.into_iter() {
        result += number * counter.left * counter.right;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
