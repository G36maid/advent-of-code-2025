advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    // For Part 1, we want the largest 2-digit number (k=2).
    // The result fits in u32.
    Some(solve(input, 2) as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    // For Part 2, we want the largest 12-digit number (k=12).
    // The sum will exceed u32, so we use u64.
    Some(solve(input, 12))
}

fn solve(input: &str, k: usize) -> u64 {
    let mut total_joltage = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        // Convert characters to digits
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        if digits.len() < k {
            continue;
        }

        // We want to find the subsequence of length k that forms the largest number.
        // This is equivalent to finding the lexicographically largest subsequence of length k.
        // We can use a greedy approach with a stack.
        // We have `n` digits and want to keep `k`. This means we can drop `n - k` digits.
        // Iterate through digits: if current digit > stack.top() and we can still drop digits, pop stack.

        let mut stack = Vec::with_capacity(digits.len());
        let mut drop_count = digits.len() - k;

        for &digit in &digits {
            while drop_count > 0 && !stack.is_empty() && *stack.last().unwrap() < digit {
                stack.pop();
                drop_count -= 1;
            }
            stack.push(digit);
        }

        // Truncate to exactly k digits (in case we didn't drop enough because the sequence was decreasing)
        stack.truncate(k);

        // Convert the sequence of digits to a number
        let mut value: u64 = 0;
        for &d in &stack {
            value = value * 10 + d as u64;
        }

        total_joltage += value;
    }

    total_joltage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
