advent_of_code::solution!(3);

fn resolve_full_mull(val: &str) -> i32 {
    let (left, right) = val.split_once(",").expect("Could not split mull");
    let l = left
        .strip_prefix("mul(")
        .expect("Could not split first half")
        .parse::<i32>();
    let r = right
        .strip_suffix(")")
        .expect("Could not split first half")
        .parse::<i32>();
    l.unwrap() * r.unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let reference = "mul(,)".as_bytes();
    //               012345
    let mut pointer = 0;
    let mut buffer: String = String::new();
    let mut result = 0;
    for bchar in input.bytes() {
        if bchar.is_ascii_digit() && (pointer == 4 || pointer == 5) {
            buffer.push(bchar as char);
            continue;
        }
        if bchar == reference[pointer] {
            pointer += 1;
            buffer.push(bchar as char);
        } else {
            pointer = 0;
            buffer.clear();
        }
        if pointer > 5 {
            result += resolve_full_mull(&buffer) as u64;
            pointer = 0;
            buffer.clear();
        }
    }

    Some(result)
}

enum Options {
    Mul,
    Do,
    Dont,
}

impl Options {
    fn prefix(&self) -> &[u8] {
        match self {
            Self::Mul => "mul(".as_bytes(),
            Self::Do => "do()".as_bytes(),
            Self::Dont => "don't()".as_bytes(),
        }
    }

    fn get_from(bytes: &[u8]) -> Option<Self> {
        for opt in [Self::Mul, Self::Do, Self::Dont].into_iter() {
            let prefix = opt.prefix();
            if bytes.starts_with(prefix) {
                return Some(opt);
            }
        }
        return None;
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut pointer = 0;
    let bytes = input.as_bytes();
    let mut result = 0;
    let mut activated = true;
    while pointer < bytes.len() - 4 {
        let kind = Options::get_from(&bytes[pointer..]);
        if kind.is_none() {
            pointer += 1;
            continue;
        }
        match kind.unwrap() {
            Options::Mul => {
                pointer += 4;
                if activated {
                    let mut left = 0;
                    while bytes[pointer].is_ascii_digit() {
                        left += 10 * left + (bytes[pointer] - b'0') as u64;
                        pointer += 1;
                    }

                    if bytes[pointer] != b',' {
                        continue;
                    }
                    pointer += 1;

                    let mut right = 0;
                    while bytes[pointer].is_ascii_digit() {
                        right += 10 * right + (bytes[pointer] - b'0') as u64;
                        pointer += 1;
                    }
                    if bytes[pointer] != b')' {
                        continue;
                    }
                    result += left * right;
                    pointer += 1;
                }
            }
            Options::Do => {
                activated = true;
                pointer += 4;
            }
            Options::Dont => {
                activated = false;
                pointer += 7;
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
