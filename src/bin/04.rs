advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    if grid.is_empty() {
        return Some(0);
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut accessible_rolls = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                let mut neighbor_count = 0;
                // Iterate over the 8 neighbors
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        // Skip the cell itself
                        if dr == 0 && dc == 0 {
                            continue;
                        }

                        let nr = r as isize + dr;
                        let nc = c as isize + dc;

                        // Check bounds and neighbor type
                        if nr >= 0
                            && nr < rows as isize
                            && nc >= 0
                            && nc < cols as isize
                            && grid[nr as usize][nc as usize] == '@'
                        {
                            neighbor_count += 1;
                        }
                    }
                }

                if neighbor_count < 4 {
                    accessible_rolls += 1;
                }
            }
        }
    }

    Some(accessible_rolls)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    if grid.is_empty() {
        return Some(0);
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_removed = 0;

    loop {
        let mut removable_rolls = Vec::new();
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '@' {
                    let mut neighbor_count = 0;
                    for dr in -1..=1 {
                        for dc in -1..=1 {
                            if dr == 0 && dc == 0 {
                                continue;
                            }
                            let nr = r as isize + dr;
                            let nc = c as isize + dc;

                            if nr >= 0
                                && nr < rows as isize
                                && nc >= 0
                                && nc < cols as isize
                                && grid[nr as usize][nc as usize] == '@'
                            {
                                neighbor_count += 1;
                            }
                        }
                    }

                    if neighbor_count < 4 {
                        removable_rolls.push((r, c));
                    }
                }
            }
        }

        if removable_rolls.is_empty() {
            break;
        }

        total_removed += removable_rolls.len() as u32;
        for (r, c) in removable_rolls {
            grid[r][c] = '.';
        }
    }

    Some(total_removed)
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
