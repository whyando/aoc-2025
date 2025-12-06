fn parse_i64_from_bytes(bytes: &[u8]) -> i64 {
    let mut n = 0i64;
    for &b in bytes {
        n = n * 10 + (b - b'0') as i64;
    }
    n
}

pub fn solve(bytes: &[u8]) -> (i64, i64) {
    let mut part1 = 0;
    let mut part2 = 0;

    // Part 1
    // Read into Vec<Vec<&[u8]>> splitting on newlines and then splitting on spaces
    // (discard all extra whitespace)
    let grid: Vec<Vec<&[u8]>> = bytes
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(|&b| b == b' ')
                .filter(|s| !s.is_empty())
                .collect()
        })
        .collect();
    let height = grid.len();
    let width = grid[0].len();

    // Now for each column, compute the sum/product
    for x in 0..width {
        let operator = grid[height - 1][x][0];
        match operator {
            b'+' => {
                let mut acc = 0;
                for y in 0..height - 1 {
                    acc += parse_i64_from_bytes(grid[y][x]);
                }
                part1 += acc as i64;
            }
            b'*' => {
                let mut acc = 1;
                for y in 0..height - 1 {
                    acc *= parse_i64_from_bytes(grid[y][x]);
                }
                part1 += acc as i64;
            }
            _ => {
                panic!("Invalid operator: {}", (operator - b'0') as i64);
            }
        }
    }

    // Part 2
    let lines = bytes
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<&[u8]>>();
    let height = lines.len();
    let width = lines[0].len();

    // Easiest way to get the width of each column is using the final line
    // since the operator is always left aligned
    let mut op_idx = width - 1;
    let mut last_op_idx = width;
    loop {
        while lines[height - 1][op_idx] == b' ' {
            op_idx -= 1;
        }
        let op_is_add = lines[height - 1][op_idx] == b'+';

        // Read lines[y][op_idx..last_op_idx]
        let mut acc = if op_is_add { 0 } else { 1 };
        for j in (op_idx..last_op_idx).rev() {
            let mut x = 0;
            for i in 0..height - 1 {
                if lines[i][j] != b' ' {
                    x = x * 10 + (lines[i][j] - b'0') as i64;
                }
            }
            acc = match op_is_add {
                true => acc + x,
                false => acc * x,
            };
        }
        part2 += acc;

        // move left to next column
        match op_idx {
            0 => break,
            _ => {
                op_idx -= 1;
                last_op_idx = op_idx;
            }
        }
    }
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = crate::file::read("inputs/06.txt").unwrap();
        assert_eq!(solve(&input), (3785892992137, 7669802156452));
    }

    #[test]
    fn test_part1() {
        let test_input = b"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(solve(&test_input.to_vec()), (4277556, 3263827));
    }
}
