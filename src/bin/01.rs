advent_of_code::solution!(1);

#[derive(Debug, Clone, Copy)]
struct Rotation {
    direction: char,
    distance: i32,
}

impl Rotation {
    fn parse(line: &str) -> Option<Self> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }
        Some(Rotation {
            direction: line.chars().next()?,
            distance: line[1..].parse().ok()?,
        })
    }

    fn apply(self, position: i32) -> i32 {
        match self.direction {
            'L' => (position - self.distance).rem_euclid(100),
            'R' => (position + self.distance).rem_euclid(100),
            _ => position,
        }
    }

    fn count_zero_crosses(self, position: i32) -> u32 {
        match self.direction {
            'L' => {
                if position == 0 {
                    (self.distance / 100) as u32
                } else if self.distance >= position {
                    (1 + (self.distance - position) / 100) as u32
                } else {
                    0
                }
            }
            'R' => {
                if position == 0 {
                    (self.distance / 100) as u32
                } else {
                    let first_hit = 100 - position;
                    if self.distance >= first_hit {
                        (1 + (self.distance - first_hit) / 100) as u32
                    } else {
                        0
                    }
                }
            }
            _ => 0,
        }
    }
}

// Functional approach using fold
pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .filter_map(Rotation::parse)
        .try_fold((50, 0), |(pos, count), rotation| {
            let new_pos = rotation.apply(pos);
            let new_count = count + u32::from(new_pos == 0);
            Some((new_pos, new_count))
        })
        .map(|(_, count)| count)
}

// Functional approach using fold
pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .filter_map(Rotation::parse)
        .try_fold((50, 0), |(pos, count), rotation| {
            let crosses = rotation.count_zero_crosses(pos);
            let new_pos = rotation.apply(pos);
            Some((new_pos, count + crosses))
        })
        .map(|(_, count)| count)
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
