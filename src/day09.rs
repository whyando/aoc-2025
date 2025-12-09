use std::collections::BTreeMap;

pub fn read(path: &str) -> Result<Vec<u8>, std::io::Error> {
    Ok(std::fs::read(path)?)
}

fn parse_i64(bytes: &[u8]) -> i64 {
    let mut num = 0;
    for &b in bytes {
        num = num * 10 + (b - b'0') as i64;
    }
    num
}

struct Point {
    x: i64,
    y: i64,
}

fn parse_points(bytes: &[u8]) -> Vec<Point> {
    let mut points = Vec::new();
    for line in bytes.split(|&b| b == b'\n') {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<i64> = line.split(|&b| b == b',').map(parse_i64).collect();
        points.push(Point {
            x: parts[0],
            y: parts[1],
        });
    }
    points
}

pub fn solve(bytes: &[u8]) -> (i64, i64) {
    // Parse
    let points = parse_points(bytes);

    // Part 1
    let mut part1_max_area = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p = &points[i];
            let q = &points[j];
            let area = ((p.x - q.x).abs() + 1) * ((p.y - q.y).abs() + 1);
            if area > part1_max_area {
                part1_max_area = area;
            }
        }
    }

    // Part 2
    // Flood fill against grid of unique x-coords and y-coords
    let mut x_coord: Vec<i64> = Vec::new();
    let mut y_coord: Vec<i64> = Vec::new();
    for p in &points {
        x_coord.push(p.x);
        y_coord.push(p.y);
    }
    // Sort and drop duplicates
    x_coord.sort_unstable();
    y_coord.sort_unstable();
    x_coord.dedup();
    y_coord.dedup();

    // Create an index lookup
    let mut x_idx_lookup: BTreeMap<i64, usize> = BTreeMap::new();
    let mut y_idx_lookup: BTreeMap<i64, usize> = BTreeMap::new();
    for (i, x) in x_coord.iter().enumerate() {
        x_idx_lookup.insert(*x, i);
    }
    for (i, y) in y_coord.iter().enumerate() {
        y_idx_lookup.insert(*y, i);
    }

    let mut edge_north = vec![vec![0; y_coord.len() + 1]; x_coord.len() + 1];
    let mut edge_west = vec![vec![0; y_coord.len() + 1]; x_coord.len() + 1];
    // Iterate adjacent pairs
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        let mut x1 = points[i].x;
        let mut x2 = points[j].x;
        let mut y1 = points[i].y;
        let mut y2 = points[j].y;
        if x2 < x1 {
            std::mem::swap(&mut x1, &mut x2);
        }
        if y2 < y1 {
            std::mem::swap(&mut y1, &mut y2);
        }
        if x1 == x2 {
            let x_idx = x_idx_lookup.get(&x1).unwrap();
            let y1_idx = y_idx_lookup.get(&y1).unwrap();
            let y2_idx = y_idx_lookup.get(&y2).unwrap();
            for y_idx in *y1_idx..*y2_idx {
                edge_west[*x_idx][y_idx] = 1;
            }
        } else if y1 == y2 {
            let y_idx = y_idx_lookup.get(&y1).unwrap();
            let x1_idx = x_idx_lookup.get(&x1).unwrap();
            let x2_idx = x_idx_lookup.get(&x2).unwrap();
            for x_idx in *x1_idx..*x2_idx {
                edge_north[x_idx][*y_idx] = 1;
            }
        } else {
            panic!("Invalid pair: {:?}", (x1, x2, y1, y2));
        }
    }

    // Flood fill enclosed area
    // Look at first edge - assume that the points are clockwise
    let x1_idx = x_idx_lookup.get(&points[0].x).unwrap();
    let y1_idx = y_idx_lookup.get(&points[0].y).unwrap();
    let x2_idx = x_idx_lookup.get(&points[1].x).unwrap();
    let y2_idx = y_idx_lookup.get(&points[1].y).unwrap();

    let (sx, sy) = if *x1_idx == *x2_idx {
        if *y2_idx > *y1_idx {
            // Edge is heading southwards
            // This is an east edge, relative to the first point
            (*x1_idx - 1, *y1_idx)
        } else if *y2_idx < *y1_idx {
            // Edge is heading northwards
            // This is a west edge, relative to the first point
            (*x1_idx, *y1_idx - 1)
        } else {
            panic!("Invalid edge direction");
        }
    } else {
        if *x2_idx < *x1_idx {
            // Edge is heading westwards
            // This is a south edge, relative to the first point
            (*x1_idx - 1, *y1_idx - 1)
        } else if *x2_idx > *x1_idx {
            // Edge is heading eastwards
            // This is a north edge, relative to the first point
            (*x1_idx, *y1_idx)
        } else {
            panic!("Invalid edge direction");
        }
    };
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut grid = vec![vec![0; y_coord.len()]; x_coord.len()];
    stack.push((sx, sy));
    grid[sx][sy] = 1;

    while let Some((x, y)) = stack.pop() {
        // Check if edges are present in the 4 directions
        let north = edge_north[x][y] == 1;
        let west = edge_west[x][y] == 1;
        let south = edge_north[x][y + 1] == 1;
        let east = edge_west[x + 1][y] == 1;
        if !north {
            if grid[x][y - 1] == 0 {
                grid[x][y - 1] = 1;
                stack.push((x, y - 1));
            }
        }
        if !west {
            if grid[x - 1][y] == 0 {
                grid[x - 1][y] = 1;
                stack.push((x - 1, y));
            }
        }
        if !south {
            if grid[x][y + 1] == 0 {
                grid[x][y + 1] = 1;
                stack.push((x, y + 1));
            }
        }
        if !east {
            if grid[x + 1][y] == 0 {
                grid[x + 1][y] = 1;
                stack.push((x + 1, y));
            }
        }
    }

    // Finally, check pairs of points
    // check every point inside
    let mut part2_max_area = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p = &points[i];
            let q = &points[j];
            let area = ((p.x - q.x).abs() + 1) * ((p.y - q.y).abs() + 1);
            if area <= part2_max_area {
                continue;
            }
            let mut x1 = p.x;
            let mut x2 = q.x;
            let mut y1 = p.y;
            let mut y2 = q.y;
            if x2 < x1 {
                std::mem::swap(&mut x1, &mut x2);
            }
            if y2 < y1 {
                std::mem::swap(&mut y1, &mut y2);
            }
            let x1_idx = x_idx_lookup.get(&x1).unwrap();
            let x2_idx = x_idx_lookup.get(&x2).unwrap();
            let y1_idx = y_idx_lookup.get(&y1).unwrap();
            let y2_idx = y_idx_lookup.get(&y2).unwrap();
            let mut enclosed = true;
            'outer: for x_idx in *x1_idx..*x2_idx {
                for y_idx in *y1_idx..*y2_idx {
                    if grid[x_idx][y_idx] == 0 {
                        enclosed = false;
                        break 'outer;
                    }
                }
            }
            if enclosed {
                part2_max_area = area;
            }
        }
    }
    (part1_max_area, part2_max_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = read("inputs/09.txt").unwrap();
        assert_eq!(solve(&input), (4755429952, 1429596008));
    }

    #[test]
    fn test_part1() {
        let test_input = b"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(solve(&test_input.to_vec()), (50, 24));
    }
}
