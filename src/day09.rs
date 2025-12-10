use std::cmp::{max, min};

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

#[derive(Debug)]
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

#[inline(always)]
fn lines_intersect(horizontal_line: (i64, i64, i64), vertical_line: (i64, i64, i64)) -> bool {
    let (y, x1, x2) = horizontal_line;
    let (x, y1, y2) = vertical_line;
    x1 <= x && x <= x2 && y1 <= y && y <= y2
}

pub fn solve(bytes: &[u8]) -> (i64, i64) {
    // Parse
    let original_points = parse_points(bytes);
    let n = original_points.len();

    let mut x = -1;
    let mut y = -1;

    // Create a new set of mapped 'shifted' points, where we make the shape very slightly larger
    let mut points = Vec::with_capacity(n);
    let mut longest_left_segment_length = 0;
    let mut longest_left_segment_point_idx = 0;
    let mut longest_right_segment_length = 0;
    let mut longest_right_segment_point_idx = 0;
    for i in 0..n + 2 {
        let p1 = &original_points[i % n];
        let p2 = &original_points[(i + 1) % n];

        if p1.x == p2.x {
            if p2.y > p1.y {
                // Down
                x = 2 * p1.x + 1;
            } else {
                // Up
                x = 2 * p1.x - 1;
            }
        } else if p1.y == p2.y {
            if p2.x < p1.x {
                // Left
                y = 2 * p1.y + 1;
                if p1.x - p2.x > longest_left_segment_length {
                    longest_left_segment_length = p1.x - p2.x;
                    longest_left_segment_point_idx = i % n;
                }
            } else {
                // Right
                y = 2 * p1.y - 1;
                if p2.x - p1.x > longest_right_segment_length {
                    longest_right_segment_length = p2.x - p1.x;
                    longest_right_segment_point_idx = (i + 1) % n;
                }
            }
        } else {
            panic!();
        }
        if i >= 2 {
            points.push(Point { x, y });
        }
    }

    // Compute all the segments
    let mut up_segments = vec![];
    let mut down_segments = vec![];
    let mut left_segments = vec![];
    let mut right_segments = vec![];

    for i in 0..n + 2 {
        let p1 = &points[i % n];
        let p2 = &points[(i + 1) % n];

        if p1.x == p2.x {
            if p2.y > p1.y {
                // Down
                down_segments.push((p1.x, min(p1.y, p2.y), max(p1.y, p2.y)));
            } else {
                // Up
                up_segments.push((p1.x, min(p1.y, p2.y), max(p1.y, p2.y)));
            }
        } else if p1.y == p2.y {
            if p2.x < p1.x {
                // Left
                left_segments.push((p1.y, min(p1.x, p2.x), max(p1.x, p2.x)));
            } else {
                // Right
                right_segments.push((p1.y, min(p1.x, p2.x), max(p1.x, p2.x)));
            }
        } else {
            panic!();
        }
    }
    up_segments.sort_unstable_by_key(|s| s.0);
    down_segments.sort_unstable_by_key(|s| -s.0);
    left_segments.sort_unstable_by_key(|s| -s.0);
    right_segments.sort_unstable_by_key(|s| s.0);

    // For each point, calculate the how far we can go in each direction, before hitting an edge
    let mut dist = vec![vec![0; 4]; n];
    for i in (0..n).rev() {
        let px = original_points[i].x * 2;
        let py = original_points[i].y * 2;

        // West - keep going west till we hit an 'up' segment
        let mut idx = up_segments.partition_point(|&seg| seg.0 <= px);
        while idx != 0 && !lines_intersect(up_segments[idx - 1], (py, up_segments[idx - 1].0, px)) {
            idx -= 1;
        }
        dist[i][0] = (px - up_segments[idx - 1].0 - 1) / 2;

        // East - keep going east till we hit an 'down' segment
        let mut idx = down_segments.partition_point(|&seg| seg.0 >= px);
        while idx != 0
            && !lines_intersect(down_segments[idx - 1], (py, px, down_segments[idx - 1].0))
        {
            idx -= 1;
        }
        dist[i][1] = (down_segments[idx - 1].0 - px - 1) / 2;

        // North - keep going north till we hit a 'right' segment
        let mut idx = right_segments.partition_point(|&seg| seg.0 <= py);
        while idx != 0
            && !lines_intersect((px, right_segments[idx - 1].0, py), right_segments[idx - 1])
        {
            idx -= 1;
        }
        dist[i][2] = (py - right_segments[idx - 1].0 - 1) / 2;

        // South - keep going south till we hit a 'left' segment
        let mut idx = left_segments.partition_point(|&seg| seg.0 >= py);
        while idx != 0
            && !lines_intersect((px, py, left_segments[idx - 1].0), left_segments[idx - 1])
        {
            idx -= 1;
        }
        dist[i][3] = (left_segments[idx - 1].0 - py - 1) / 2;
    }

    let mut part1 = 0;
    let mut part2 = 0;

    // For each pair of points
    for i in 0..n {
        for j in i + 1..n {
            let p1 = &original_points[i];
            let p2 = &original_points[j];
            let area = ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1);
            part1 = max(part1, area);
        }
    }

    // Speed up the search by assuming that the solution always contains the rightmost point
    // of one of the longest 2 segments
    // If we checked each pair of points, this would be a fully general solution
    for i in [
        longest_left_segment_point_idx,
        longest_right_segment_point_idx,
    ] {
        for j in 0..n {
            if j == longest_left_segment_point_idx || j == longest_right_segment_point_idx {
                continue;
            }

            let p1 = &original_points[i];
            let p2 = &original_points[j];

            let area = ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1);
            part1 = max(part1, area);

            if area <= part2 {
                continue;
            }

            if p2.x < p1.x {
                // West from p1
                // req dist[i][west] >= p1.x - p2.x
                // req dist[j][east] >= p1.x - p2.x
                if !(dist[i][0] >= p1.x - p2.x && dist[j][1] >= p1.x - p2.x) {
                    continue;
                }
            } else {
                // East from p1
                // req dist[i][east] >= p2.x - p1.x
                // req dist[j][west] >= p2.x - p1.x
                if !(dist[i][1] >= p2.x - p1.x && dist[j][0] >= p2.x - p1.x) {
                    continue;
                }
            }

            if p2.y < p1.y {
                // North from p1
                // req dist[i][north] >= p1.y - p2.y
                // req dist[j][south] >= p1.y - p2.y
                if !(dist[i][2] >= p1.y - p2.y && dist[j][3] >= p1.y - p2.y) {
                    continue;
                }
            } else {
                // South from p1
                // req dist[i][south] >= p2.y - p1.y
                // req dist[j][north] >= p2.y - p1.y
                if !(dist[i][3] >= p2.y - p1.y && dist[j][2] >= p2.y - p1.y) {
                    continue;
                }
            }

            // We have a valid pair
            part2 = area;
        }
    }

    (part1, part2)
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
