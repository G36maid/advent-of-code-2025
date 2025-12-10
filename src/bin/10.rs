use good_lp::*;

advent_of_code::solution!(10);

#[derive(Debug, Clone)]
struct Machine {
    lights: Vec<u8>,          // Target state for Part 1 (0 or 1)
    joltages: Vec<i32>,       // Target state for Part 2
    buttons: Vec<Vec<usize>>, // Indices affected by each button
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            // Example: [.##.] (3) (1,3) (2) ... {3,5,4,7}

            // 1. Parse Lights [.##.]
            let start_bracket = line.find('[').unwrap();
            let end_bracket = line.find(']').unwrap();
            let lights_str = &line[start_bracket + 1..end_bracket];
            let lights = lights_str
                .chars()
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect();

            // 2. Parse Joltages {3,5,4,7}
            let start_brace = line.find('{').unwrap();
            let end_brace = line.find('}').unwrap();
            let jolt_str = &line[start_brace + 1..end_brace];
            let joltages = jolt_str
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();

            // 3. Parse Buttons (3) (1,3) ...
            // We look for content between ']' and '{'
            let middle_part = &line[end_bracket + 1..start_brace];
            let mut buttons = Vec::new();
            // Split by ')' to get chunks like " (3" or " (1,3"
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

// --- Part 1 Solver: GF(2) ---

fn solve_part1_machine(m: &Machine) -> Option<usize> {
    let num_eq = m.lights.len();
    let num_vars = m.buttons.len();

    // Build Augmented Matrix [A | b]
    // A[i][j] = 1 if button j affects light i
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

    // Gaussian Elimination (GF2)
    let mut pivot_row = 0;
    let mut col_to_pivot = vec![None; num_vars]; // Maps column to pivot row index
    let mut pivot_cols = Vec::new(); // List of pivot columns (indices)
    let mut free_cols = Vec::new();

    for j in 0..num_vars {
        if pivot_row >= num_eq {
            free_cols.push(j);
            continue;
        }

        // Find row with 1 in this column
        let selected_row = (pivot_row..num_eq).find(|&i| matrix[i][j] == 1);

        if let Some(row_idx) = selected_row {
            // Swap
            matrix.swap(pivot_row, row_idx);

            // Eliminate other rows
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

    // Check consistency
    if matrix
        .iter()
        .skip(pivot_row)
        .take(num_eq - pivot_row)
        .any(|row| row[num_vars] != 0)
    {
        return None; // No solution
    }

    // Brute force free variables
    // Given the constraints of AoC, free variables count should be small.
    let num_free = free_cols.len();
    let mut min_presses = usize::MAX;

    let combinations = 1 << num_free;
    for mask in 0..combinations {
        let mut current_solution = vec![0u8; num_vars];
        let mut presses = 0;

        // Set free variables
        for (k, &col_idx) in free_cols.iter().enumerate() {
            if (mask >> k) & 1 == 1 {
                current_solution[col_idx] = 1;
                presses += 1;
            }
        }

        // Back substitute for pivot variables
        // x_pivot = b_pivot ^ sum(A_pivot_k * x_k)
        // Since we diagonalized, row `r` corresponds to pivot col `j`.
        // matrix[r][j] is 1. x_j = matrix[r][last] ^ sum(matrix[r][free] * x_free)

        // We iterate pivot columns in reverse order (though not strictly necessary due to full elimination)
        for &j in pivot_cols.iter().rev() {
            let r = col_to_pivot[j].unwrap();
            let mut val = matrix[r][num_vars];
            for &k in &free_cols {
                if matrix[r][k] == 1 && current_solution[k] == 1 {
                    val ^= 1;
                }
            }
            current_solution[j] = val;
            if val == 1 {
                presses += 1;
            }
        }

        if presses < min_presses {
            min_presses = presses;
        }
    }

    if min_presses == usize::MAX {
        None
    } else {
        Some(min_presses)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    let mut total_presses = 0;

    for m in machines {
        if let Some(p) = solve_part1_machine(&m) {
            total_presses += p as u64;
        }
    }

    Some(total_presses)
}

// --- Part 2 Solver: Integer Linear Programming with good_lp ---

fn solve_part2_machine(m: &Machine) -> Option<u64> {
    let num_counters = m.joltages.len();
    let num_buttons = m.buttons.len();

    if num_counters == 0 {
        return Some(0);
    }

    // Create variables for button presses
    let mut vars = ProblemVariables::new();
    let button_vars: Vec<Variable> = (0..num_buttons)
        .map(|i| vars.add(variable().integer().min(0).name(format!("button_{}", i))))
        .collect();

    // Build the problem: minimize sum of button presses
    let objective: Expression = button_vars.iter().map(|&v| Expression::from(v)).sum();
    let mut problem = vars.minimise(objective).using(default_solver);

    // Add constraints: for each counter, sum of button presses affecting it = target
    for (counter_idx, &target) in m.joltages.iter().enumerate() {
        let mut constraint_expr = Expression::from(0);

        for (button_idx, button) in m.buttons.iter().enumerate() {
            if button.contains(&counter_idx) {
                constraint_expr += button_vars[button_idx];
            }
        }

        problem = problem.with(constraint!(constraint_expr == target));
    }

    // Solve the problem
    match problem.solve() {
        Ok(solution) => {
            let total: f64 = button_vars.iter().map(|&v| solution.value(v)).sum();
            Some(total.round() as u64)
        }
        Err(_) => None,
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    let mut total_presses = 0;

    for m in machines {
        if let Some(p) = solve_part2_machine(&m) {
            total_presses += p;
        }
    }

    Some(total_presses)
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
