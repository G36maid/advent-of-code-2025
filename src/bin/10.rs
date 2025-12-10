use good_lp::*;

advent_of_code::solution!(10);

#[derive(Debug, Clone)]
struct Machine {
    lights: Vec<u8>,
    joltages: Vec<i32>,
    buttons: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let start_bracket = line.find('[').unwrap();
            let end_bracket = line.find(']').unwrap();
            let lights_str = &line[start_bracket + 1..end_bracket];
            let lights = lights_str
                .chars()
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect();

            let start_brace = line.find('{').unwrap();
            let end_brace = line.find('}').unwrap();
            let jolt_str = &line[start_brace + 1..end_brace];
            let joltages = jolt_str
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();

            let middle_part = &line[end_bracket + 1..start_brace];
            let mut buttons = Vec::new();
            for chunk in middle_part.split(')') {
                if let Some(start_paren) = chunk.find('(') {
                    let nums = &chunk[start_paren + 1..];
                    let indices = nums
                        .split(',')
                        .map(|s| s.trim().parse::<usize>().unwrap())
                        .collect();
                    buttons.push(indices);
                }
            }

            Machine {
                lights,
                joltages,
                buttons,
            }
        })
        .collect()
}

// Part 1: GF(2) Gaussian Elimination
// Toggling lights is XOR, so we solve Ax = b in GF(2) where A[i][j] = 1 if button j affects light i

/// Gaussian elimination in GF(2). Returns (pivot_cols, free_cols, col_to_pivot).
fn gaussian_elimination_gf2(
    matrix: &mut [Vec<u8>],
    num_vars: usize,
) -> (Vec<usize>, Vec<usize>, Vec<Option<usize>>) {
    let num_eq = matrix.len();
    let mut pivot_row = 0;
    let mut col_to_pivot = vec![None; num_vars];
    let mut pivot_cols = Vec::new();
    let mut free_cols = Vec::new();

    for j in 0..num_vars {
        if pivot_row >= num_eq {
            free_cols.push(j);
            continue;
        }

        let selected_row = (pivot_row..num_eq).find(|&i| matrix[i][j] == 1);

        if let Some(row_idx) = selected_row {
            matrix.swap(pivot_row, row_idx);

            let pivot_row_copy = matrix[pivot_row].clone();
            for (i, row) in matrix.iter_mut().enumerate() {
                if i != pivot_row && row[j] == 1 {
                    for k in j..=num_vars {
                        row[k] ^= pivot_row_copy[k];
                    }
                }
            }

            col_to_pivot[j] = Some(pivot_row);
            pivot_cols.push(j);
            pivot_row += 1;
        } else {
            free_cols.push(j);
        }
    }

    (pivot_cols, free_cols, col_to_pivot)
}

/// Try all 2^k free variable assignments, back-substitute pivots, find minimum button presses.
fn find_minimum_solution_gf2(
    matrix: &[Vec<u8>],
    num_vars: usize,
    pivot_cols: &[usize],
    free_cols: &[usize],
    col_to_pivot: &[Option<usize>],
) -> usize {
    let num_free = free_cols.len();
    let mut min_presses = usize::MAX;

    for mask in 0..(1 << num_free) {
        let mut solution = vec![0u8; num_vars];
        let mut presses = 0;

        for (k, &col_idx) in free_cols.iter().enumerate() {
            if (mask >> k) & 1 == 1 {
                solution[col_idx] = 1;
                presses += 1;
            }
        }

        for &j in pivot_cols.iter().rev() {
            let r = col_to_pivot[j].unwrap();
            let mut val = matrix[r][num_vars];
            for &k in free_cols {
                if matrix[r][k] == 1 && solution[k] == 1 {
                    val ^= 1;
                }
            }
            solution[j] = val;
            if val == 1 {
                presses += 1;
            }
        }

        min_presses = min_presses.min(presses);
    }

    min_presses
}

fn solve_part1_machine(m: &Machine) -> Option<usize> {
    let num_eq = m.lights.len();
    let num_vars = m.buttons.len();

    let mut matrix = vec![vec![0u8; num_vars + 1]; num_eq];

    for (j, btn) in m.buttons.iter().enumerate() {
        for &affected_light in btn {
            if affected_light < num_eq {
                matrix[affected_light][j] = 1;
            }
        }
    }

    for (i, row) in matrix.iter_mut().enumerate().take(num_eq) {
        row[num_vars] = m.lights[i];
    }

    let (pivot_cols, free_cols, col_to_pivot) = gaussian_elimination_gf2(&mut matrix, num_vars);

    let pivot_row = pivot_cols.len();
    if matrix.iter().skip(pivot_row).any(|row| row[num_vars] != 0) {
        return None;
    }

    let min_presses =
        find_minimum_solution_gf2(&matrix, num_vars, &pivot_cols, &free_cols, &col_to_pivot);

    if min_presses == usize::MAX {
        None
    } else {
        Some(min_presses)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    machines
        .iter()
        .filter_map(solve_part1_machine)
        .map(|p| p as u64)
        .sum::<u64>()
        .into()
}

// Part 2: Integer Linear Programming
// Incrementing counters is addition, so we minimize sum(x) subject to Ax = b, x >= 0

/// Build ILP: minimize sum of button presses, subject to each counter reaching its target.
fn build_ilp_problem(
    buttons: &[Vec<usize>],
    joltages: &[i32],
) -> (Vec<Variable>, impl SolverModel) {
    let mut vars = ProblemVariables::new();
    let button_vars: Vec<Variable> = (0..buttons.len())
        .map(|i| vars.add(variable().integer().min(0).name(format!("b{}", i))))
        .collect();

    let objective: Expression = button_vars.iter().map(|&v| Expression::from(v)).sum();
    let mut problem = vars.minimise(objective).using(default_solver);

    for (counter_idx, &target) in joltages.iter().enumerate() {
        let mut constraint_expr = Expression::from(0);
        for (button_idx, button) in buttons.iter().enumerate() {
            if button.contains(&counter_idx) {
                constraint_expr += button_vars[button_idx];
            }
        }
        problem = problem.with(constraint!(constraint_expr == target));
    }

    (button_vars, problem)
}

fn solve_ilp_problem(button_vars: &[Variable], problem: impl SolverModel) -> Option<u64> {
    problem.solve().ok().map(|solution| {
        button_vars
            .iter()
            .map(|&v| solution.value(v))
            .sum::<f64>()
            .round() as u64
    })
}

fn solve_part2_machine(m: &Machine) -> Option<u64> {
    if m.joltages.is_empty() {
        return Some(0);
    }

    let (button_vars, problem) = build_ilp_problem(&m.buttons, &m.joltages);
    solve_ilp_problem(&button_vars, problem)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    machines
        .iter()
        .filter_map(solve_part2_machine)
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
