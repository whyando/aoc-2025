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
fn at(bytes: &[u8], x: i32, y: i32, n: i32) -> u8 {
    unsafe { *bytes.get_unchecked((y * (n + 1) + x) as usize) }
}

#[inline(always)]
fn zero(bytes: &mut [u8], x: i32, y: i32, n: i32) {
    unsafe { *bytes.get_unchecked_mut((y * (n + 1) + x) as usize) = 0; }
}

pub fn solve(n: i32, bytes: &mut [u8]) -> (i64, i64) {
    let mut part1 = 0;
    let mut part2 = 0;

    // Part 1
    let mut stack: Vec<(i32, i32)> = Vec::new();
    for x in 0..n {
        for y in 0..n {
            if at(bytes, x, y, n) != b'@' {
                continue;
            }
            let mut adjacent_rolls = 0;
            for (dx, dy) in DIRECTIONS {
                let x1 = x + dx;
                let y1 = y + dy;
                if x1 >= 0 && x1 < n && y1 >= 0 && y1 < n {
                    if at(bytes, x1, y1, n) != b'.' {
                        adjacent_rolls += 1;
                    }
                }
            }
            if adjacent_rolls < 4 {
                part1 += 1;
                // Then remove this cell
                zero(bytes, x, y, n);
                // Add adjacent cells to the stack
                for (dx, dy) in DIRECTIONS {
                    let x1 = x + dx;
                    let y1 = y + dy;
                    if x1 != -1 && x1 != n && y1 != -1 && y1 != n {
                        stack.push((x1, y1));
                    }
                }
            }
        }
    }

    // Part 2
    while let Some((x, y)) = stack.pop() {
        if at(bytes, x, y, n) != b'@' {
            continue;
        }
        let mut adjacent_rolls = 0;
        for (dx, dy) in DIRECTIONS {
            let x1 = x + dx;
            let y1 = y + dy;
            if x1 >= 0 && x1 < n && y1 >= 0 && y1 < n {
                if at(bytes, x1, y1, n) == b'@' {
                    adjacent_rolls += 1;
                }
            }
        }
        if adjacent_rolls < 4 {
            // Then remove this cell
            zero(bytes, x, y, n);
            part2 += 1;
            // Add adjacent cells to the stack
            for (dx, dy) in DIRECTIONS {
                let x1 = x + dx;
                let y1 = y + dy;
                if x1 != -1 && x1 != n && y1 != -1 && y1 != n {
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
        let mut input = std::fs::read("inputs/04.txt").unwrap();
        assert_eq!(solve(138, &mut input), (1419, 8739));
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

        assert_eq!(solve(10, &mut test_input.to_vec()), (13, 43));
    }
}
