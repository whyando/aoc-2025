const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[inline(always)]
fn at<const N: i32>(bytes: &[u8], x: i32, y: i32) -> u8 {
    unsafe { *bytes.get_unchecked((y * N + x) as usize) }
}

#[inline(always)]
fn zero<const N: i32>(bytes: &mut [u8], x: i32, y: i32) {
    unsafe {
        *bytes.get_unchecked_mut((y * N + x) as usize) = 0;
    }
}

pub fn solve(bytes: &mut [u8]) -> (i64, i64) {
    match bytes.len() {
        100 => solve_inner::<10>(bytes),
        18225 => solve_inner::<135>(bytes),
        18496 => solve_inner::<136>(bytes),
        18769 => solve_inner::<137>(bytes),
        19044 => solve_inner::<138>(bytes),
        19321 => solve_inner::<139>(bytes),
        19596 => solve_inner::<140>(bytes),
        _ => panic!("Invalid input size"),
    }
}

fn solve_inner<const N: i32>(bytes: &mut [u8]) -> (i64, i64) {
    let mut part1 = 0;
    let mut part2 = 0;

    // Part 1
    let mut stack: Vec<(i32, i32)> = Vec::with_capacity((N * N) as usize);
    for x in 0..N {
        for y in 0..N {
            if at::<N>(bytes, x, y) != b'@' {
                continue;
            }
            let mut adjacent_rolls = 0;
            for (dx, dy) in DIRECTIONS {
                let x1 = x + dx;
                let y1 = y + dy;
                if x1 >= 0 && x1 < N && y1 >= 0 && y1 < N {
                    if at::<N>(bytes, x1, y1) != b'.' {
                        adjacent_rolls += 1;
                    }
                }
            }
            if adjacent_rolls < 4 {
                part1 += 1;
                // Then remove this cell
                zero::<N>(bytes, x, y);
                // Add adjacent cells to the stack
                for (dx, dy) in DIRECTIONS {
                    let x1 = x + dx;
                    let y1 = y + dy;
                    if x1 != -1 && x1 != N && y1 != -1 && y1 != N {
                        stack.push((x1, y1));
                    }
                }
            }
        }
    }

    // Part 2
    while let Some((x, y)) = stack.pop() {
        if at::<N>(bytes, x, y) != b'@' {
            continue;
        }
        let mut adjacent_rolls = 0;
        for (dx, dy) in DIRECTIONS {
            let x1 = x + dx;
            let y1 = y + dy;
            if x1 >= 0 && x1 < N && y1 >= 0 && y1 < N {
                if at::<N>(bytes, x1, y1) == b'@' {
                    adjacent_rolls += 1;
                }
            }
        }
        if adjacent_rolls < 4 {
            // Then remove this cell
            zero::<N>(bytes, x, y);
            part2 += 1;
            // Add adjacent cells to the stack
            for (dx, dy) in DIRECTIONS {
                let x1 = x + dx;
                let y1 = y + dy;
                if x1 != -1 && x1 != N && y1 != -1 && y1 != N {
                    stack.push((x1, y1));
                }
            }
        }
    }
    (part1, part1 + part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut input = crate::file::read_no_newlines("inputs/04.txt").unwrap();
        assert_eq!(solve(&mut input), (1419, 8739));
    }

    #[test]
    fn test_part1() {
        let test_input = b"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let mut test_input: Vec<u8> = test_input
            .iter()
            .copied()
            .filter(|b| *b != b'\n' && *b != b'\r')
            .collect();
        assert_eq!(solve(&mut test_input), (13, 43));
    }
}
