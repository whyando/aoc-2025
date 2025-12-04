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
fn at<const N: i32>(bytes: &[u8], x: i32, y: i32) -> &u8 {
    unsafe { bytes.get_unchecked((y * N + x) as usize) }
}

pub fn solve(bytes: &[u8]) -> (i64, i64) {
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

#[inline(always)]
fn adj_index<const N: i32>(x: i32, y: i32) -> usize {
    ((y + 1) * (N + 2) + x + 1) as usize
}

#[inline(always)]
fn adj_at<const N: i32>(adj: &[u8], x: i32, y: i32) -> &u8 {
    unsafe { adj.get_unchecked(adj_index::<N>(x, y)) }
}

#[inline(always)]
fn adj_at_mut<const N: i32>(adj: &mut [u8], x: i32, y: i32) -> &mut u8 {
    unsafe { adj.get_unchecked_mut(adj_index::<N>(x, y)) }
}

fn solve_inner<const N: i32>(bytes: &[u8]) -> (i64, i64) {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut adj = vec![0u8; ((N + 2) * (N + 2)) as usize];
    let mut stack: Vec<(i32, i32)> = Vec::with_capacity((N * N) as usize);

    for y in 0..N {
        for x in 0..N {
            if *at::<N>(bytes, x, y) == b'@' {
                for (dx, dy) in DIRECTIONS {
                    *adj_at_mut::<N>(&mut adj, x + dx, y + dy) += 1;
                }
            } else {
                *adj_at_mut::<N>(&mut adj, x, y) = 12;
            }
        }
    }
    for i in 0..(N * N) as usize {
        if unsafe { *bytes.get_unchecked(i) } == b'@' {
            let x = (i % N as usize) as i32;
            let y = (i / N as usize) as i32;
            if *adj_at::<N>(&adj, x, y) < 4 {
                part1 += 1;
                stack.push((x, y));
            }
        }
    }

    while let Some((x, y)) = stack.pop() {
        part2 += 1;
        for (dx, dy) in DIRECTIONS {
            if *adj_at::<N>(&adj, x + dx, y + dy) == 4 {
                stack.push((x + dx, y + dy));
            }
            *adj_at_mut::<N>(&mut adj, x + dx, y + dy) -= 1;
        }
    }
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = crate::file::read_no_newlines("inputs/04.txt").unwrap();
        assert_eq!(solve(&input), (1419, 8739));
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
        let test_input: Vec<u8> = test_input
            .iter()
            .copied()
            .filter(|b| *b != b'\n' && *b != b'\r')
            .collect();
        assert_eq!(solve(&test_input), (13, 43));
    }
}
