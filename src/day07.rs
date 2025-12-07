pub fn read(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut out = Vec::new();
    let mut buf = [0u8; 8192];

    loop {
        let n = std::io::Read::read(&mut file, &mut buf)?;
        if n == 0 {
            break;
        }

        out.extend(buf[..n].iter().copied());
    }

    // Remove trailing newline
    if out.last() == Some(&b'\n') {
        out.pop();
    }
    Ok(out)
}

pub fn solve(bytes: &[u8]) -> (i64, i64) {
    let mut part1 = 0;

    // Read into a grid
    let mut grid = Vec::new();
    for line in bytes.split(|&b| b == b'\n') {
        grid.push(line.to_vec());
    }
    let height = grid.len();
    let width = grid[0].len();

    let mut v = vec![vec![0; width]; height];
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if grid[y][x] == b'^' {
                v[y][x] += 1;
            }
        }
    }
    let s_idx = grid[0].iter().position(|&c| c == b'S').unwrap();
    v[0][s_idx] = 1;

    for y in 1..height {
        for x in 0..width {
            // For each cell, check cell above
            if grid[y - 1][x] == b'|' || grid[y - 1][x] == b'S' {
                // propagate to this cell, or if ^, propagate sideways
                if grid[y][x] == b'^' {
                    // Propagate sideways
                    grid[y][x - 1] = b'|';
                    grid[y][x + 1] = b'|';
                    v[y][x - 1] += v[y - 1][x];
                    v[y][x + 1] += v[y - 1][x];
                    part1 += 1;
                } else {
                    // Propagate to this cell
                    grid[y][x] = b'|';
                    v[y][x] += v[y - 1][x];
                }
            }
        }
    }
    let part2 = v.last().unwrap().iter().sum::<i64>();
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = read("inputs/07.txt").unwrap();
        assert_eq!(solve(&input), (1672, 231229866702355));
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
        assert_eq!(solve(&test_input.to_vec()), (21, 40));
    }
}
