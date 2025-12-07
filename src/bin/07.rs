advent_of_code::solution!(7);

use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    if grid.is_empty() {
        return Some(0);
    }
    let height = grid.len();
    let width = grid.iter().map(|line| line.len()).max().unwrap_or(0);

    // Find the starting position of 'S' in the first row.
    let start_col = grid[0].iter().position(|&c| c == b'S')?;

    let mut active_beams: HashSet<usize> = HashSet::new();
    active_beams.insert(start_col);

    let mut split_count = 0;

    // Simulate row by row.
    for row in 0..(height - 1) {
        if active_beams.is_empty() {
            break;
        }

        let mut next_beams: HashSet<usize> = HashSet::new();
        for &col in &active_beams {
            // Check the character in the row below the current beam.
            // Beams that go off the side of the manifold are simply terminated.
            if let Some(&next_char) = grid.get(row + 1).and_then(|r| r.get(col)) {
                match next_char {
                    b'^' => {
                        split_count += 1;
                        // A new beam is created to the left, if not at the edge.
                        if col > 0 {
                            next_beams.insert(col - 1);
                        }
                        // A new beam is created to the right, if not at the edge.
                        if col + 1 < width {
                            next_beams.insert(col + 1);
                        }
                    }
                    b'.' => {
                        // The beam continues downward.
                        next_beams.insert(col);
                    }
                    // Any other character is treated as empty space for beam propagation.
                    _ => {
                        next_beams.insert(col);
                    }
                }
            }
        }
        active_beams = next_beams;
    }

    Some(split_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    if grid.is_empty() {
        return Some(0);
    }
    let height = grid.len();
    let width = grid.iter().map(|line| line.len()).max().unwrap_or(0);

    let start_col = grid[0].iter().position(|&c| c == b'S')?;

    let mut timelines: HashMap<usize, u64> = HashMap::new();
    timelines.insert(start_col, 1);

    for row in 0..(height - 1) {
        if timelines.is_empty() {
            break;
        }

        let mut next_timelines: HashMap<usize, u64> = HashMap::new();
        for (&col, &count) in &timelines {
            if let Some(&next_char) = grid.get(row + 1).and_then(|r| r.get(col)) {
                match next_char {
                    b'^' => {
                        if col > 0 {
                            *next_timelines.entry(col - 1).or_insert(0) += count;
                        }
                        if col + 1 < width {
                            *next_timelines.entry(col + 1).or_insert(0) += count;
                        }
                    }
                    b'.' => {
                        *next_timelines.entry(col).or_insert(0) += count;
                    }
                    _ => {
                        *next_timelines.entry(col).or_insert(0) += count;
                    }
                }
            }
        }
        timelines = next_timelines;
    }

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
