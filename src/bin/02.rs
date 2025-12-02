advent_of_code::solution!(2);

fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Must have even length to split in half
    if !len.is_multiple_of(2) {
        return false;
    }

    let mid = len / 2;
    s[..mid] == s[mid..]
}

fn is_invalid_id_v2(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Check all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=(len / 2) {
        // Pattern must divide evenly and repeat at least twice
        if len.is_multiple_of(pattern_len) {
            let pattern = &s[..pattern_len];
            let mut is_repeated = true;

            // Check if the entire string is this pattern repeated
            for chunk_start in (pattern_len..len).step_by(pattern_len) {
                if &s[chunk_start..chunk_start + pattern_len] != pattern {
                    is_repeated = false;
                    break;
                }
            }

            if is_repeated {
                return true;
            }
        }
    }

    false
}

fn parse_ranges(input: &str) -> Option<Vec<(u64, u64)>> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let parts: Vec<_> = range.split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].parse().ok()?;
                let end = parts[1].parse().ok()?;
                Some((start, end))
            } else {
                None
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input)?;

    let sum: u64 = ranges
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&n| is_invalid_id(n))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input)?;

    let sum: u64 = ranges
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&n| is_invalid_id_v2(n))
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_id() {
        assert!(is_invalid_id(11));
        assert!(is_invalid_id(22));
        assert!(is_invalid_id(99));
        assert!(is_invalid_id(1010));
        assert!(is_invalid_id(6464));
        assert!(is_invalid_id(123123));
        assert!(is_invalid_id(222222));
        assert!(is_invalid_id(446446));

        assert!(!is_invalid_id(101));
        assert!(!is_invalid_id(123));
        assert!(!is_invalid_id(1234));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_is_invalid_id_v2() {
        // Part 1 examples still work
        assert!(is_invalid_id_v2(11));
        assert!(is_invalid_id_v2(22));
        assert!(is_invalid_id_v2(99));
        assert!(is_invalid_id_v2(1010));
        assert!(is_invalid_id_v2(6464));
        assert!(is_invalid_id_v2(123123));
        assert!(is_invalid_id_v2(222222));
        assert!(is_invalid_id_v2(446446));

        // Part 2 new examples
        assert!(is_invalid_id_v2(111)); // "1" repeated 3 times
        assert!(is_invalid_id_v2(999)); // "9" repeated 3 times
        assert!(is_invalid_id_v2(12341234)); // "1234" repeated 2 times
        assert!(is_invalid_id_v2(123123123)); // "123" repeated 3 times
        assert!(is_invalid_id_v2(1212121212)); // "12" repeated 5 times
        assert!(is_invalid_id_v2(1111111)); // "1" repeated 7 times
        assert!(is_invalid_id_v2(565656)); // "56" repeated 3 times
        assert!(is_invalid_id_v2(824824824)); // "824" repeated 3 times
        assert!(is_invalid_id_v2(2121212121)); // "21" repeated 5 times

        assert!(!is_invalid_id_v2(101));
        assert!(!is_invalid_id_v2(1234));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
