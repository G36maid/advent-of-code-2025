advent_of_code::solution!(2);

/// --- Day 2: Gift Shop ---
///
/// This solution avoids brute-force checking of all numbers in the ranges.
/// Instead, it mathematically generates only the "invalid" numbers (those with
/// repeated digit patterns) and sums the ones that fall within the given ranges.
/// This is significantly faster, especially for large ranges.
///
/// # Part 1: Pattern Repeated Exactly Twice (e.g., 123123)
///
/// An ID is invalid if it's a number `X` concatenated with itself.
/// This can be expressed mathematically:
///
/// `N = X * (10^d + 1)`
///
/// where `d` is the number of digits in `X`.
///
/// We generate these numbers for all possible base patterns `X` and sum those
/// that are within the provided ranges.
///
/// # Part 2: Pattern Repeated At Least Twice (e.g., 123123, 123123123)
///
/// An ID is invalid if it's a number `X` repeated `k` times.
/// This can be expressed as a geometric series:
///
/// `N = X * (1 + 10^d + 10^(2d) + ... + 10^((k-1)d))`
///
/// which simplifies to:
///
/// `N = X * (10^(kd) - 1) / (10^d - 1)`
///
/// We generate these numbers for all `k >= 2` and all possible base patterns `X`,
/// use a HashSet to handle duplicates (e.g., `1111` can be "1" repeated 4 times
/// or "11" repeated 2 times), and sum the results.
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

/// Generates invalid IDs for Part 1 (pattern repeated exactly twice) within a single range.
/// Formula: `N = base * (10^d + 1)`
fn generate_doubled_in_range(start: u64, end: u64) -> impl Iterator<Item = u64> {
    // Max 5 digits for base since 10-digit numbers are max
    (1..=5).flat_map(move |num_digits| {
        let multiplier = 10u64.pow(num_digits) + 1;

        // Calculate which bases produce numbers in [start, end]
        let min_base = start.div_ceil(multiplier);
        let max_base = end / multiplier;

        // Base must have exactly num_digits
        let base_start = 10u64.pow(num_digits - 1);
        let base_end = 10u64.pow(num_digits);

        (min_base.max(base_start)..=max_base.min(base_end - 1))
            .map(move |base| base * multiplier)
            .filter(move |&n| n >= start && n <= end)
    })
}

/// Generates invalid IDs for Part 2 (pattern repeated k times) within a single range.
/// Formula: `N = base * (10^(kd) - 1) / (10^d - 1)`
fn generate_repeated_in_range(start: u64, end: u64, k: usize) -> impl Iterator<Item = u64> {
    let start_digits = start.to_string().len();
    let end_digits = end.to_string().len();

    // Determine min and max pattern lengths needed for this range and k
    let min_pattern_len = start_digits.div_ceil(k);
    let max_pattern_len = end_digits / k;

    (min_pattern_len..=max_pattern_len).flat_map(move |pattern_len| {
        let base_start = 10u64.pow(pattern_len.saturating_sub(1) as u32);
        let base_end = 10u64.pow(pattern_len as u32);

        let d = pattern_len as u32;
        let power_d = 10u64.pow(d);

        // Calculate the geometric series sum multiplier
        let multiplier = if let Some(power_kd) = 10u64.checked_pow(k as u32 * d) {
            (power_kd - 1) / (power_d - 1)
        } else {
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

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input)?;

    Some(
        ranges
            .into_iter()
            .flat_map(|(start, end)| generate_doubled_in_range(start, end))
            .sum(),
    )
}

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
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
