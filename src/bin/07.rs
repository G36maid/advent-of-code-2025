advent_of_code::solution!(7);

use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    if grid.is_empty() {
        return Some(0);
    }

    // Find the starting column of 'S' in the first row.
    let start_col = grid[0].iter().position(|&c| c == 'S')?;

    // `beams` will store the column index of each active beam for the current row.
    // Using a HashSet automatically handles cases where multiple beams merge at the same spot.
    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(start_col);

    let mut split_count = 0;

    // Iterate through the grid row by row, starting from the row below 'S'.
    for r in 1..grid.len() {
        let mut next_beams = HashSet::new();
        for &col in &beams {
            // Beams that would be out of bounds are ignored.
            if col >= grid[r].len() {
                continue;
            }

            match grid[r][col] {
                '^' => {
                    split_count += 1;
                    // A splitter creates two new beams for the next row.
                    if col > 0 {
                        next_beams.insert(col - 1);
                    }
                    next_beams.insert(col + 1);
                }
                '.' => {
                    // An empty space lets the beam pass through to the same column in the next row.
                    next_beams.insert(col);
                }
                _ => (), // Should not happen with valid input
            }
        }
        beams = next_beams;
    }

    Some(split_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    if grid.is_empty() {
        return Some(0);
    }

    // Find the starting column of 'S'.
    let start_col = grid[0].iter().position(|&c| c == 'S')?;

    // `timelines` stores the number of ways a particle can reach each column in the current row.
    // Format: { column_index => number_of_timelines }
    let mut timelines: HashMap<usize, u64> = HashMap::new();
    timelines.insert(start_col, 1);

    // Use dynamic programming, processing the grid row by row.
    for r in 1..grid.len() {
        let mut next_timelines = HashMap::new();
        for (&col, &count) in &timelines {
            // Particles that would go out of bounds are ignored.
            if col >= grid[r].len() {
                continue;
            }

            match grid[r][col] {
                '^' => {
                    // A splitter duplicates the timelines. The current count is added to the
                    // tallies for the left and right paths in the next row.
                    if col > 0 {
                        *next_timelines.entry(col - 1).or_insert(0) += count;
                    }
                    *next_timelines.entry(col + 1).or_insert(0) += count;
                }
                '.' => {
                    // An empty space just passes the timelines down to the same column.
                    *next_timelines.entry(col).or_insert(0) += count;
                }
                _ => (), // Should not happen with valid input
            }
        }
        timelines = next_timelines;
    }

    // The total number of timelines is the sum of timelines at all possible final positions.
    Some(timelines.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
