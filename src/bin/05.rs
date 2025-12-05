advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = parse_input(input);

    let fresh_count = ids.iter().filter(|&&id| is_fresh(id, &ranges)).count();

    Some(fresh_count as u64)
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    // Parse ranges
    let ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .map(|line| {
            let nums: Vec<u64> = line.split('-').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();

    // Parse ingredient IDs
    let ids: Vec<u64> = parts[1].lines().map(|line| line.parse().unwrap()).collect();

    (ranges, ids)
}

fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    ranges.iter().any(|&(start, end)| id >= start && id <= end)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse_input(input);

    // Merge overlapping ranges
    let merged = merge_ranges(ranges);

    // Count total IDs in merged ranges
    let total: u64 = merged.iter().map(|&(start, end)| end - start + 1).sum();

    Some(total)
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return vec![];
    }

    // Sort ranges by start position
    ranges.sort_by_key(|&(start, _)| start);

    let mut merged = vec![ranges[0]];

    for &(start, end) in &ranges[1..] {
        let last_idx = merged.len() - 1;
        let (last_start, last_end) = merged[last_idx];

        // Check if current range overlaps or is adjacent to last merged range
        if start <= last_end + 1 {
            // Merge by extending the end if needed
            merged[last_idx] = (last_start, last_end.max(end));
        } else {
            // No overlap, add as new range
            merged.push((start, end));
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
