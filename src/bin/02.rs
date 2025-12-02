advent_of_code::solution!(2);

#[cfg(test)]
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

#[cfg(test)]
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

// Optimized: Generate invalid IDs using math instead of string operations
pub fn part_one_fast(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input)?;
    let mut sum = 0u64;

    // Find min and max across all ranges to determine bounds
    let min = ranges.iter().map(|(start, _)| start).min()?;
    let max = ranges.iter().map(|(_, end)| end).max()?;

    // Generate all numbers of form XX (pattern repeated exactly twice)
    // Mathematical approach: if base has d digits, repeated = base * (10^d + 1)
    // Example: 123 (3 digits) -> 123123 = 123 * (10^3 + 1) = 123 * 1001

    // Try all possible digit lengths for the base pattern
    for num_digits in 1..=5 {
        // Max 5 digits per half (10 digits total)
        let start = 10u64.pow(num_digits - 1);
        let end = 10u64.pow(num_digits);
        let multiplier = 10u64.pow(num_digits) + 1;

        for base in start..end {
            let n = base * multiplier;

            // Check if n is in any range
            if n > *max {
                break;
            }
            if n >= *min {
                for &(start, end) in &ranges {
                    if n >= start && n <= end {
                        sum += n;
                        break;
                    }
                }
            }
        }
    }

    Some(sum)
}

// Optimized: Generate all repeated patterns (at least twice)
pub fn part_two_fast(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input)?;
    let mut found = std::collections::HashSet::new();

    let min = ranges.iter().map(|(start, _)| start).min()?;
    let max = ranges.iter().map(|(_, end)| end).max()?;
    let max_digits = max.to_string().len();

    // For each possible repetition count k (2, 3, 4, ...)
    for k in 2..=max_digits {
        // For each repetition count, determine max base needed
        let max_base_digits = max_digits / k;
        if max_base_digits == 0 {
            break;
        }

        // Generate bases from 1 to 10^max_base_digits - 1
        let end_base = 10u64.pow(max_base_digits as u32);

        for base in 1..end_base {
            let base_str = base.to_string();
            let repeated_str = base_str.repeat(k);

            if let Ok(n) = repeated_str.parse::<u64>()
                && n >= *min
                && n <= *max
                && !found.contains(&n)
            {
                for &(start, end) in &ranges {
                    if n >= start && n <= end {
                        found.insert(n);
                        break;
                    }
                }
            }
        }
    }

    Some(found.iter().sum())
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_fast(input)
}

pub fn part_two(input: &str) -> Option<u64> {
    part_two_fast(input)
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
