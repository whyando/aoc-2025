const SPACE: u8 = 0;
// const OP_NEWLINE: u8 = b'\n'; // 10
const OP_ADD: u8 = 16;
const OP_MUL: u8 = 32;

#[inline(always)]
fn byte_substitution(b: &u8) -> u8 {
    match b {
        b' ' => SPACE,
        b'1'..=b'9' => b - b'0',
        b'+' => OP_ADD,
        b'*' => OP_MUL,
        _ => *b,
    }
}

pub fn read(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut out = Vec::new();
    let mut buf = [0u8; 8192];

    loop {
        let n = std::io::Read::read(&mut file, &mut buf)?;
        if n == 0 {
            break;
        }

        out.extend(buf[..n].iter().map(byte_substitution));
    }

    // Remove trailing newline
    if out.last() == Some(&b'\n') {
        out.pop();
    }
    Ok(out)
}

pub fn solve<const HEIGHT: usize>(bytes: &[u8]) -> (i64, i64) {
    let mut part1 = 0;
    let mut part2 = 0;

    let lines = bytes.split(|&b| b == b'\n').collect::<Vec<&[u8]>>();
    let width = lines[0].len();

    // Easiest way to get the width of each column is using the final line
    // since the operator is always left aligned
    let mut op_idx = lines[HEIGHT].len() - 1;
    let mut last_op_idx = width;
    loop {
        while lines[HEIGHT][op_idx] == SPACE {
            op_idx -= 1;
        }
        let op_is_add = lines[HEIGHT][op_idx] == OP_ADD;
        if op_is_add {
            // Part 1
            for i in (0..HEIGHT).rev() {
                let mut x = 0;
                for j in op_idx..last_op_idx {
                    if lines[i][j] != SPACE {
                        x = x * 10 + lines[i][j] as i64;
                    }
                }
                part1 += x
            }
            // Part 2
            for j in op_idx..last_op_idx {
                let mut x = 0;
                for i in 0..HEIGHT {
                    if lines[i][j] != SPACE {
                        x = x * 10 + lines[i][j] as i64;
                    }
                }
                part2 += x;
            }
        } else {
            let mut acc1 = 1;
            let mut acc2 = 1;
            // Part 1
            for i in (0..HEIGHT).rev() {
                let mut x = 0;
                for j in op_idx..last_op_idx {
                    if lines[i][j] != SPACE {
                        x = x * 10 + lines[i][j] as i64;
                    }
                }
                acc1 *= x
            }

            // Part 2
            for j in op_idx..last_op_idx {
                let mut x = 0;
                for i in 0..HEIGHT {
                    if lines[i][j] != SPACE {
                        x = x * 10 + lines[i][j] as i64;
                    }
                }
                acc2 *= x;
            }
            part1 += acc1;
            part2 += acc2;
        }

        // move left to next column
        if op_idx == 0 {
            break;
        }
        op_idx -= 1;
        last_op_idx = op_idx;
    }
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = read("inputs/06.txt").unwrap();
        assert_eq!(solve::<4>(&input), (3785892992137, 7669802156452));
    }

    #[test]
    fn test_part1() {
        let test_input = b"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let test_input = test_input
            .iter()
            .map(byte_substitution)
            .collect::<Vec<u8>>();
        assert_eq!(solve::<3>(&test_input.to_vec()), (4277556, 3263827));
    }
}
