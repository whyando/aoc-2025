use microlp::{ComparisonOp, OptimizationDirection, Problem};

pub fn read(path: &str) -> Result<Vec<u8>, std::io::Error> {
    Ok(std::fs::read(path)?)
}

fn parse_u64(bytes: &[u8]) -> u64 {
    let mut num = 0;
    for &b in bytes {
        num = num * 10 + (b - b'0') as u64;
    }
    num
}

#[derive(Debug)]
struct Machine {
    target: BitSet,
    buttons: Vec<BitSet>,
    joltage: Vec<u64>,
}

// Bitset (maxes out at 64 bits)
#[derive(Debug)]
struct BitSet(u64);

impl BitSet {
    fn new() -> Self {
        Self(0)
    }

    fn set(&mut self, i: usize) {
        self.0 |= 1 << i;
    }

    fn get(&self, i: usize) -> bool {
        self.0 & (1 << i) != 0
    }
}

fn encode_target_str(buttons_str: &[u8]) -> BitSet {
    let mut x = BitSet::new();
    for (i, &b) in buttons_str.iter().enumerate() {
        if b == b'#' {
            x.set(i);
        }
    }
    x
}

fn parse(bytes: &[u8]) -> Vec<Machine> {
    let mut machines = Vec::new();
    for line in bytes.split(|&b| b == b'\n').filter(|line| !line.is_empty()) {
        let split = line.split(|&b| b == b' ').collect::<Vec<&[u8]>>();

        // Split[0]
        let target_str = &split[0][1..split[0].len() - 1];
        let target = encode_target_str(target_str);

        // Split[1..split.len() - 1]
        let buttons: Vec<BitSet> = split[1..split.len() - 1]
            .iter()
            .map(|s| {
                let buttons_str = &s[1..s.len() - 1];
                let mut button = BitSet::new();
                for i_str in buttons_str.split(|&b| b == b',') {
                    let i = (i_str[0] - b'0') as usize;
                    button.set(i);
                }
                button
            })
            .collect();

        // Split[split.len() - 1]
        let joltage_str = &split[split.len() - 1][1..split[split.len() - 1].len() - 1];
        let joltage = joltage_str
            .split(|&b| b == b',')
            .map(|s| parse_u64(s))
            .collect();

        machines.push(Machine {
            target,
            buttons,
            joltage,
        });
    }
    machines
}

fn solve_with_lp(machine: &Machine) -> u64 {
    let n_buttons = machine.buttons.len();
    let n_positions = machine.joltage.len();

    // We want to minimize: sum of x[j] (L1 norm)
    // Subject to: for each position i, sum over j of (x[j] * button[j].get(i)) = joltage[i]
    // And: x[j] >= 0 for all j

    // Create problem: minimize objective
    let mut problem = Problem::new(OptimizationDirection::Minimize);

    // Add variables: x[0], x[1], ..., x[n_buttons-1] (number of times to press each button)
    // Each variable has coefficient 1.0 in the objective (L1 norm)
    // Variables are non-negative integers (lower bound 0, upper bound i32::MAX)
    let mut vars = Vec::new();
    for _ in 0..n_buttons {
        let var = problem.add_integer_var(1.0, (0, i32::MAX));
        vars.push(var);
    }

    // Add constraints: for each position i, sum of x[j] where button j affects position i = joltage[i]
    for i in 0..n_positions {
        let mut constraint_vars = Vec::new();
        for j in 0..n_buttons {
            if machine.buttons[j].get(i) {
                constraint_vars.push((vars[j], 1.0));
            }
        }
        if !constraint_vars.is_empty() {
            problem.add_constraint(
                &constraint_vars,
                ComparisonOp::Eq,
                machine.joltage[i] as f64,
            );
        }
    }

    // Solve
    let solution = problem.solve().expect("Failed to solve LP");

    // Extract solution values and compute L1 norm
    // Use var_value_rounded to get integer values (handles floating-point precision issues)
    let mut l1_norm = 0.0;
    for &var in &vars {
        l1_norm += solution.var_value_rounded(var);
    }

    l1_norm as u64
}

pub fn solve(bytes: &[u8]) -> (usize, u64) {
    let machines = parse(bytes);

    let mut part1 = 0;
    for machine in machines {
        let mut min_presses = usize::MAX;
        let n = machine.buttons.len();
        // Try 2^n combinations of button presses
        for i in 0..(1 << n) {
            let mut x = 0;
            for j in 0..n {
                if i & (1 << j) != 0 {
                    x ^= machine.buttons[j].0;
                }
            }
            if x == machine.target.0 {
                let total_presses = (0..n).filter(|&j| i & (1 << j) != 0).count();
                if total_presses < min_presses {
                    min_presses = total_presses;
                }
            }
        }
        part1 += min_presses;
    }

    let mut part2 = 0;
    let machines = parse(bytes);
    for machine in machines {
        // Part 2:
        // construct linear system, then solve with microlp crate (probably branch and bound + simplex)
        // finding positive integer solution with minimum L1 norm
        let cost = solve_with_lp(&machine);
        part2 += cost;
    }
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = read("inputs/10.txt").unwrap();
        assert_eq!(solve(&input), (449, 17848));
    }

    #[test]
    fn test_part1() {
        let test_input = b"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(solve(&test_input.to_vec()), (7, 33));
    }
}
