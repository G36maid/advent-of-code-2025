advent_of_code::solution!(6);

fn transpose(input: &str) -> Option<(Vec<u8>, usize, usize)> {
    let lines: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    if lines.is_empty() {
        return None;
    }

    let height = lines.len();
    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    // Flat grid for cache-friendly access: columns stored contiguously
    let mut grid = vec![b' '; width * height];

    for col in 0..width {
        for row in 0..height {
            grid[col * height + row] = *lines.get(row)?.get(col).unwrap_or(&b' ');
        }
    }

    Some((grid, width, height))
}

fn solve(input: &str, reverse: bool) -> Option<u64> {
    let (grid, width, height) = transpose(input)?;

    let mut total = 0u64;
    let mut col_start = 0;

    while col_start < width {
        // Skip empty columns (problem separators)
        while col_start < width
            && grid[col_start * height..(col_start + 1) * height]
                .iter()
                .all(|&b| b == b' ')
        {
            col_start += 1;
        }

        if col_start >= width {
            break;
        }

        // Find the end of the current problem
        let mut col_end = col_start;
        while col_end < width
            && !grid[col_end * height..(col_end + 1) * height]
                .iter()
                .all(|&b| b == b' ')
        {
            col_end += 1;
        }

        // Extract operator from the last row of any column in this problem
        let op = grid[col_start * height + height - 1];

        // Parse numbers based on direction
        let numbers = if reverse {
            // Part 2: Read columns right-to-left, each column is a number (digits top-to-bottom)
            ((col_start..col_end).rev())
                .filter_map(|col| {
                    let num_bytes: Vec<u8> = grid[col * height..(col + 1) * height - 1]
                        .iter()
                        .copied()
                        .filter(|&b| b.is_ascii_digit())
                        .collect();
                    if num_bytes.is_empty() {
                        None
                    } else {
                        std::str::from_utf8(&num_bytes).ok()?.parse::<u64>().ok()
                    }
                })
                .collect::<Vec<_>>()
        } else {
            // Part 1: Read rows top-to-bottom (except last row), each row is a number
            (0..height - 1)
                .filter_map(|row| {
                    let num_bytes: Vec<u8> = (col_start..col_end)
                        .map(|col| grid[col * height + row])
                        .filter(|&b| b.is_ascii_digit())
                        .collect();
                    if num_bytes.is_empty() {
                        None
                    } else {
                        std::str::from_utf8(&num_bytes).ok()?.parse::<u64>().ok()
                    }
                })
                .collect::<Vec<_>>()
        };

        // Calculate result based on operator
        let result = match op {
            b'+' => numbers.iter().sum::<u64>(),
            b'*' => numbers.iter().product::<u64>(),
            _ => 0,
        };

        total += result;
        col_start = col_end;
    }

    Some(total)
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
