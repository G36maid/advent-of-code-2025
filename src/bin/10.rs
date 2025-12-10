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

/// Performs Gaussian elimination in GF(2) (binary field) on an augmented matrix.
///
/// This function transforms the matrix into reduced row echelon form using XOR operations.
/// It identifies pivot columns (basic variables) and free columns (free variables).
///
/// # Arguments
/// * `matrix` - Augmented matrix [A|b] where operations are performed in GF(2)
/// * `num_vars` - Number of variables (columns excluding augmented column)
///
/// # Returns
/// A tuple containing:
/// * `pivot_cols` - List of column indices that have pivots (basic variables)
/// * `free_cols` - List of column indices without pivots (free variables)
/// * `col_to_pivot` - Maps each column to its pivot row (if any)
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

    (pivot_cols, free_cols, col_to_pivot)
}

/// Finds the minimum number of button presses by exhaustively trying all free variable combinations.
///
/// This function iterates through all 2^k possible assignments to free variables, where k is the
/// number of free variables. For each assignment, it performs back substitution to determine the
/// values of pivot variables and counts the total number of button presses (sum of variables = 1).
///
/// # Arguments
/// * `matrix` - The reduced row echelon form matrix from Gaussian elimination
/// * `num_vars` - Total number of variables
/// * `pivot_cols` - Indices of columns with pivots
/// * `free_cols` - Indices of columns without pivots (free variables)
/// * `col_to_pivot` - Mapping from column index to pivot row
///
/// # Returns
/// The minimum number of button presses needed
fn find_minimum_solution_gf2(
    matrix: &[Vec<u8>],
    num_vars: usize,
    pivot_cols: &[usize],
    free_cols: &[usize],
    col_to_pivot: &[Option<usize>],
) -> usize {
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
        for &j in pivot_cols.iter().rev() {
            let r = col_to_pivot[j].unwrap();
            let mut val = matrix[r][num_vars];
            for &k in free_cols {
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

    min_presses
}

/// Solves Part 1 for a single machine using Gaussian elimination in GF(2).
///
/// Part 1 involves toggling lights (XOR operations), which forms a system of linear equations
/// over GF(2). This function builds the augmented matrix, solves it via Gaussian elimination,
/// and finds the minimum number of button presses.
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

    // Perform Gaussian elimination
    let (pivot_cols, free_cols, col_to_pivot) = gaussian_elimination_gf2(&mut matrix, num_vars);

    // Check consistency
    let pivot_row = pivot_cols.len();
    if matrix
        .iter()
        .skip(pivot_row)
        .take(num_eq - pivot_row)
        .any(|row| row[num_vars] != 0)
    {
        return None; // No solution
    }

    // Find minimum solution
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
    let mut total_presses = 0;

    for m in machines {
        if let Some(p) = solve_part1_machine(&m) {
            total_presses += p as u64;
        }
    }

    Some(total_presses)
}

// --- Part 2 Solver: Integer Linear Programming with good_lp ---

/// Builds an Integer Linear Programming (ILP) problem for minimizing button presses.
///
/// This function creates an ILP model where:
/// - Variables: Number of times each button is pressed (non-negative integers)
/// - Objective: Minimize the sum of all button presses
/// - Constraints: For each counter, the sum of button presses affecting it equals the target
///
/// # Arguments
/// * `buttons` - List of buttons, where each button is a list of counter indices it affects
/// * `joltages` - Target values for each counter
///
/// # Returns
/// A tuple containing:
/// * `button_vars` - The decision variables (one per button)
/// * `problem` - The configured ILP problem ready to be solved
fn build_ilp_problem(
    buttons: &[Vec<usize>],
    joltages: &[i32],
) -> (Vec<Variable>, impl SolverModel) {
    let num_buttons = buttons.len();

    // Create variables for button presses (non-negative integers)
    let mut vars = ProblemVariables::new();
    let button_vars: Vec<Variable> = (0..num_buttons)
        .map(|i| vars.add(variable().integer().min(0).name(format!("button_{}", i))))
        .collect();

    // Build the problem: minimize sum of button presses
    let objective: Expression = button_vars.iter().map(|&v| Expression::from(v)).sum();
    let mut problem = vars.minimise(objective).using(default_solver);

    // Add constraints: for each counter, sum of button presses affecting it = target
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

/// Solves the ILP problem and extracts the total number of button presses.
///
/// # Arguments
/// * `button_vars` - The decision variables representing button presses
/// * `problem` - The configured ILP problem to solve
///
/// # Returns
/// Some(total) if a solution exists, None otherwise
fn solve_ilp_problem(button_vars: &[Variable], problem: impl SolverModel) -> Option<u64> {
    match problem.solve() {
        Ok(solution) => {
            let total: f64 = button_vars.iter().map(|&v| solution.value(v)).sum();
            Some(total.round() as u64)
        }
        Err(_) => None,
    }
}

/// Solves Part 2 for a single machine using Integer Linear Programming.
///
/// Part 2 involves incrementing counters (addition), which forms a system of linear equations
/// over integers. This function uses the good_lp library with minilp solver to find the
/// minimum number of button presses needed to reach the target joltage values.
fn solve_part2_machine(m: &Machine) -> Option<u64> {
    let num_counters = m.joltages.len();

    if num_counters == 0 {
        return Some(0);
    }

    // Build and solve the ILP problem
    let (button_vars, problem) = build_ilp_problem(&m.buttons, &m.joltages);
    solve_ilp_problem(&button_vars, problem)
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
