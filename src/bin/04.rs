advent_of_code::solution!(4);

fn indexes(pos: usize, width: usize) -> [(usize, usize, usize); 8] {
    let mut result = [(0, 0, 0); 8];
    let mut index = 0;
    for up in [-1, 0, 1] {
        for right in [-1, 0, 1] {
            if up == 0 && right == 0 {
                continue;
            }
            result[index] = (
                (pos as i32 + 1 * (right + up * (width) as i32)) as usize,
                (pos as i32 + 2 * (right + up * (width) as i32)) as usize,
                (pos as i32 + 3 * (right + up * (width) as i32)) as usize,
            );
            index += 1;
        }
    }

    return result;
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    for _ in 0..10000 {
        let w = input.find("\n").unwrap() + 1;
        let bytes = input.as_bytes();
        for (i, b) in bytes.iter().enumerate() {
            if *b == b'X' {
                for (l, j, k) in indexes(i, w) {
                    match bytes.get(l).zip(bytes.get(j).zip(bytes.get(k))) {
                        Some((m, (a, s))) => {
                            if *m == b'M' && *a == b'A' && *s == b'S' {
                                result += 1
                            }
                        }
                        None => {}
                    }
                }
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
