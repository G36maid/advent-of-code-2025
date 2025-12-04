use std::collections::VecDeque;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Some(0);
    }

    let rows = lines.len();
    let cols = lines[0].len();
    let mut grid: Vec<u8> = Vec::with_capacity(rows * cols);

    // Build flat grid
    for line in lines {
        grid.extend(line.bytes());
    }

    let mut accessible_count = 0;

    for idx in 0..grid.len() {
        if grid[idx] == b'@' {
            let neighbor_count = count_neighbors(&grid, idx, rows, cols);
            if neighbor_count < 4 {
                accessible_count += 1;
            }
        }
    }

    Some(accessible_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Some(0);
    }

    let rows = lines.len();
    let cols = lines[0].len();
    let mut grid: Vec<u8> = Vec::with_capacity(rows * cols);

    // Build flat grid
    for line in lines {
        grid.extend(line.bytes());
    }

    let mut total_removed = 0;
    let mut queue: VecDeque<usize> = VecDeque::new();

    // Initially, add all paper rolls to the queue
    for (idx, &cell) in grid.iter().enumerate() {
        if cell == b'@' {
            queue.push_back(idx);
        }
    }

    // Process queue in a BFS-like manner
    while let Some(idx) = queue.pop_front() {
        // Skip if already removed
        if grid[idx] != b'@' {
            continue;
        }

        let neighbor_count = count_neighbors(&grid, idx, rows, cols);
        if neighbor_count < 4 {
            // Remove this roll
            grid[idx] = b'.';
            total_removed += 1;

            // Add neighbors to queue (they might become accessible now)
            for neighbor_idx in get_neighbor_indices(idx, rows, cols) {
                if grid[neighbor_idx] == b'@' {
                    queue.push_back(neighbor_idx);
                }
            }
        }
    }

    Some(total_removed)
}

#[inline]
fn count_neighbors(grid: &[u8], idx: usize, rows: usize, cols: usize) -> u32 {
    let mut count = 0;
    for neighbor_idx in get_neighbor_indices(idx, rows, cols) {
        if grid[neighbor_idx] == b'@' {
            count += 1;
        }
    }
    count
}

#[inline]
fn get_neighbor_indices(idx: usize, rows: usize, cols: usize) -> Vec<usize> {
    let row = idx / cols;
    let col = idx % cols;
    let mut neighbors = Vec::with_capacity(8);

    for dr in -1..=1_i32 {
        for dc in -1..=1_i32 {
            if dr == 0 && dc == 0 {
                continue;
            }

            let nr = row as i32 + dr;
            let nc = col as i32 + dc;

            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                neighbors.push((nr as usize) * cols + (nc as usize));
            }
        }
    }

    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
