advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut position = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = line.chars().next()?;
        let distance: i32 = line[1..].parse().ok()?;

        position = match direction {
            'L' => (position - distance).rem_euclid(100),
            'R' => (position + distance).rem_euclid(100),
            _ => return None,
        };

        if position == 0 {
            zero_count += 1;
        }
    }

    Some(zero_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut position = 50;
    let mut zero_count: u32 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = line.chars().next()?;
        let distance: i32 = line[1..].parse().ok()?;

        // Count how many times we pass through 0 during this rotation
        let crosses = match direction {
            'L' => {
                // Going left by distance D from position P
                // We cross 0 when (P - k) mod 100 = 0 for k in [1, D]
                // This happens at k = P, P+100, P+200, ...
                if position == 0 {
                    // Special case: from 0, going left hits 0 at steps 100, 200, ...
                    (distance / 100) as u32
                } else if distance >= position {
                    (1 + (distance - position) / 100) as u32
                } else {
                    0
                }
            }
            'R' => {
                // Going right by distance D from position P
                // We cross 0 when (P + k) mod 100 = 0 for k in [1, D]
                // This happens at k = 100-P, 200-P, 300-P, ... (if P > 0)
                if position == 0 {
                    (distance / 100) as u32
                } else {
                    let first_hit = 100 - position;
                    if distance >= first_hit {
                        (1 + (distance - first_hit) / 100) as u32
                    } else {
                        0
                    }
                }
            }
            _ => return None,
        };

        zero_count += crosses;

        // Update position
        position = match direction {
            'L' => (position - distance).rem_euclid(100),
            'R' => (position + distance).rem_euclid(100),
            _ => return None,
        };
    }

    Some(zero_count)
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
        assert_eq!(result, Some(6));
    }
}
