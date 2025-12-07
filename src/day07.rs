pub fn read(path: &str) -> Result<Vec<u8>, std::io::Error> {
    Ok(std::fs::read(path)?)
}

#[inline(always)]
fn at<const WIDTH: usize>(bytes: &[u8], y: usize, x: usize) -> &u8 {
    // unsafe { bytes.get_unchecked(y * (WIDTH + 1) + x) }
    &bytes[y * (WIDTH + 1) + x]
}

pub fn solve<const WIDTH: usize>(bytes: &[u8]) -> (u64, u64) {
    let s_idx = (WIDTH - 1) / 2;
    let mut x = [0u64; WIDTH];

    x[s_idx] = 1;
    let mut part1 = 0;
    for r in 0..(WIDTH / 2) {
        let y = 2 * r + 2;
        for s in s_idx - r..=s_idx + r {
            if *at::<WIDTH>(bytes, y, s) == b'^' {
                part1 += (x[s] > 0) as u64;
                x[s - 1] += x[s];
                x[s + 1] += x[s];
                x[s] = 0;
            }
        }
    }
    (part1, x.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = read("inputs/07.txt").unwrap();
        assert_eq!(solve::<141>(&input), (1672, 231229866702355));
    }

    #[test]
    fn test_part1() {
        let test_input = b".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(solve::<15>(&test_input.to_vec()), (21, 40));
    }
}
