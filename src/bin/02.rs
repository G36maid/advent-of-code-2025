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

// Pure function: Generate invalid IDs (XX pattern) within a single range
fn generate_doubled_in_range(start: u64, end: u64) -> impl Iterator<Item = u64> {
    // For pattern repeated twice: n = base * (10^d + 1) where d = digit count of base
    (1..=5).flat_map(move |num_digits| {
        let base_start = 10u64.pow(num_digits - 1);
        let base_end = 10u64.pow(num_digits);
        let multiplier = 10u64.pow(num_digits) + 1;

        // Calculate which bases produce numbers in [start, end]
        let min_base = start.div_ceil(multiplier); // ceiling division
        let max_base = end / multiplier;

        (min_base.max(base_start)..=max_base.min(base_end - 1))
            .map(move |base| base * multiplier)
            .filter(move |&n| n >= start && n <= end)
    })
}

// Pure function: Generate invalid IDs (repeated k times) within a single range
// Uses mathematical formula: repeated = base × (10^(k×d) - 1) / (10^d - 1)
// This is a geometric series sum
fn generate_repeated_in_range(start: u64, end: u64, k: usize) -> impl Iterator<Item = u64> {
    let start_digits = start.to_string().len();
    let end_digits = end.to_string().len();

    // Determine min and max pattern lengths needed
    let min_pattern_len = start_digits.div_ceil(k); // ceiling division
    let max_pattern_len = end_digits / k;

    (min_pattern_len..=max_pattern_len).flat_map(move |pattern_len| {
        let base_start = 10u64.pow(pattern_len.saturating_sub(1) as u32);
        let base_end = 10u64.pow(pattern_len as u32);

        // Mathematical approach: avoid string operations
        // repeated = base × (10^(k×d) - 1) / (10^d - 1)
        let d = pattern_len as u32;
        let power_d = 10u64.pow(d);

        // Calculate (10^(k×d) - 1) / (10^d - 1) - this is always an integer
        let multiplier = if let Some(power_kd) = 10u64.checked_pow(k as u32 * d) {
            (power_kd - 1) / (power_d - 1)
        } else {
            // Overflow case - skip this pattern length
            return (0..0)
                .filter_map(|_| None::<u64>)
                .collect::<Vec<_>>()
                .into_iter();
        };

        (base_start..base_end)
            .filter_map(move |base| {
                base.checked_mul(multiplier)
                    .filter(|&n| n >= start && n <= end)
            })
            .collect::<Vec<_>>()
            .into_iter()
    })
}

// Pure functional approach for Part 1
pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input)?;

    Some(
        ranges
            .into_iter()
            .flat_map(|(start, end)| generate_doubled_in_range(start, end))
            .sum(),
    )
}

// Pure functional approach for Part 2
pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input)?;
    let max_digits = ranges.iter().map(|(_, end)| end).max()?.to_string().len();

    let found: std::collections::HashSet<u64> = (2..=max_digits)
        .flat_map(|k| {
            ranges
                .iter()
                .flat_map(move |&(start, end)| generate_repeated_in_range(start, end, k))
        })
        .collect();

    Some(found.iter().sum())
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
