advent_of_code::solution!(10);

#[derive(Debug, Clone)]
struct Machine {
    lights: Vec<u8>,          // Target state for Part 1 (0 or 1)
    joltages: Vec<i64>,       // Target state for Part 2
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

// --- Part 2 Solver: Integers ---

struct RecursiveSolver<'a> {
    free_cols: &'a [usize],
    col_to_pivot: &'a [Option<usize>],
    pivot_cols: &'a [usize],
    matrix: &'a [Vec<f64>],
    num_vars: usize,
    search_limit: usize,
}

impl<'a> RecursiveSolver<'a> {
    fn solve(&self, free_idx: usize, free_vals: &mut Vec<f64>, min_presses: &mut u64) {
        if free_idx == self.free_cols.len() {
            // All free vars set, calculate pivots
            let mut current_solution = vec![0.0; self.num_vars];
            let mut current_sum = 0.0;

            for (i, &col) in self.free_cols.iter().enumerate() {
                current_solution[col] = free_vals[i];
                current_sum += free_vals[i];
            }

            for &j in self.pivot_cols.iter().rev() {
                let r = self.col_to_pivot[j].unwrap();
                let mut val = self.matrix[r][self.matrix[0].len() - 1]; // constant term
                for &k in self.free_cols {
                    val -= self.matrix[r][k] * current_solution[k];
                }

                // Check integrity and non-negativity
                if val < -1e-9 {
                    return;
                } // Negative presses not allowed

                // Must be integer
                let rounded = val.round();
                if (val - rounded).abs() > 1e-9 {
                    return;
                }

                current_solution[j] = rounded;
                current_sum += rounded;
            }

            let total = current_sum as u64;
            if total < *min_presses {
                *min_presses = total;
            }
            return;
        }

        // Iterate this free variable
        for val in 0..=self.search_limit {
            free_vals[free_idx] = val as f64;
            self.solve(free_idx + 1, free_vals, min_presses);
        }
    }
}

fn solve_part2_machine(m: &Machine) -> Option<u64> {
    let num_eq = m.joltages.len();
    let num_vars = m.buttons.len();

    // Augmented Matrix with f64 for division
    // We use f64 to handle potential fractions during intermediate steps,
    // though final answer should be integer.
    let mut matrix = vec![vec![0.0f64; num_vars + 1]; num_eq];

    for (j, btn) in m.buttons.iter().enumerate() {
        for &affected_idx in btn {
            if affected_idx < num_eq {
                matrix[affected_idx][j] = 1.0;
            }
        }
    }

    for (i, row) in matrix.iter_mut().enumerate().take(num_eq) {
        row[num_vars] = m.joltages[i] as f64;
    }

    let mut pivot_row = 0;
    let mut col_to_pivot = vec![None; num_vars];
    let mut pivot_cols = Vec::new();
    let mut free_cols = Vec::new();

    for j in 0..num_vars {
        if pivot_row >= num_eq {
            free_cols.push(j);
            continue;
        }

        let selected_row = (pivot_row..num_eq).find(|&i| matrix[i][j].abs() > 1e-9);

        if let Some(row_idx) = selected_row {
            matrix.swap(pivot_row, row_idx);

            // Normalize pivot row
            let div = matrix[pivot_row][j];
            for elem in matrix[pivot_row].iter_mut().skip(j).take(num_vars + 1 - j) {
                *elem /= div;
            }

            // Eliminate others
            for i in 0..num_eq {
                if i != pivot_row && matrix[i][j].abs() > 1e-9 {
                    let factor = matrix[i][j];
                    let pivot_row_copy = matrix[pivot_row].clone();
                    for (k, elem) in matrix[i]
                        .iter_mut()
                        .enumerate()
                        .skip(j)
                        .take(num_vars + 1 - j)
                    {
                        *elem -= factor * pivot_row_copy[k];
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
        .any(|row| row[num_vars].abs() > 1e-9)
    {
        return None;
    }

    let mut min_presses = u64::MAX;
    let search_limit = 200;

    let solver = RecursiveSolver {
        free_cols: &free_cols,
        col_to_pivot: &col_to_pivot,
        pivot_cols: &pivot_cols,
        matrix: &matrix,
        num_vars,
        search_limit,
    };

    let mut free_vals = vec![0.0; free_cols.len()];
    solver.solve(0, &mut free_vals, &mut min_presses);

    if min_presses == u64::MAX {
        None
    } else {
        Some(min_presses)
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
