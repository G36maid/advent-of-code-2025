advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    if lines.is_empty() {
        return Some(0);
    }

    let height = lines.len();
    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let mut columns = Vec::with_capacity(width);
    for j in 0..width {
        let mut column = String::new();
        for i in 0..height {
            column.push(*lines.get(i).and_then(|row| row.get(j)).unwrap_or(&' '));
        }
        columns.push(column);
    }

    let problem_results: Option<Vec<u64>> = columns
        .split(|col| col.trim().is_empty())
        .filter(|p_cols| !p_cols.is_empty())
        .map(|p_cols| {
            let p_height = p_cols.first()?.len();
            let mut p_rows = Vec::with_capacity(p_height);
            for i in 0..p_height {
                let mut row = String::with_capacity(p_cols.len());
                for col in p_cols {
                    row.push(col.chars().nth(i)?);
                }
                p_rows.push(row);
            }

            let op_char = p_rows.last()?.trim().chars().next()?;

            let numbers: Vec<u64> = p_rows[..p_rows.len() - 1]
                .iter()
                .map(|row| row.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().ok())
                .collect::<Option<Vec<_>>>()?;

            Some(match op_char {
                '+' => numbers.iter().sum(),
                '*' => numbers.iter().product(),
                _ => 0,
            })
        })
        .collect();

    problem_results.map(|results| results.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    if lines.is_empty() {
        return Some(0);
    }

    let height = lines.len();
    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let mut columns = Vec::with_capacity(width);
    for j in 0..width {
        let mut column = String::new();
        for i in 0..height {
            column.push(*lines.get(i).and_then(|row| row.get(j)).unwrap_or(&' '));
        }
        columns.push(column);
    }

    let problems: Vec<_> = columns
        .split(|col| col.trim().is_empty())
        .filter(|p_cols| !p_cols.is_empty())
        .collect();

    let problem_results: Option<Vec<u64>> = problems
        .iter()
        .rev()
        .map(|p_cols| {
            let op_char = p_cols.first()?.chars().last()?;

            let numbers = p_cols
                .iter()
                .map(|s| s[..s.len() - 1].trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().ok())
                .collect::<Option<Vec<u64>>>()?;

            Some(match op_char {
                '+' => numbers.iter().sum(),
                '*' => numbers.iter().product(),
                _ => 0,
            })
        })
        .collect();

    problem_results.map(|results| results.iter().sum())
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
