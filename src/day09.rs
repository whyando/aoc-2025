use std::cmp::{min, max};

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

fn vertical_edge_inside(
    hor_segments: &[(i64, i64, i64)],
    vertical_edge: (i64, i64, i64),
) -> bool {
    let (x, y1, y2) = vertical_edge;
    let y_min = min(y1, y2);
    let y_max = max(y1, y2);
    
    for &hor_segment in hor_segments {
        let (y, x1, x2) = hor_segment;
        // Check if the vertical edge crosses this horizontal segment
        // The edge crosses if: y is between y_min and y_max, and x is between x1 and x2
        if y_min < y && y < y_max {
            let x_min = min(x1, x2);
            let x_max = max(x1, x2);
            if x_min < x && x < x_max {
                return true;
            }
        }
    }
    false
}

fn horizontal_edge_inside(
    ver_segments: &[(i64, i64, i64)],
    horizontal_edge: (i64, i64, i64),
) -> bool {
    let (y, x1, x2) = horizontal_edge;
    let x_min = min(x1, x2);
    let x_max = max(x1, x2);
    
    for &ver_segment in ver_segments {
        let (x, y1, y2) = ver_segment;
        // Check if the horizontal edge crosses this vertical segment
        // The edge crosses if: x is between x_min and x_max, and y is between y1 and y2
        if x_min < x && x < x_max {
            let y_min = min(y1, y2);
            let y_max = max(y1, y2);
            if y_min < y && y < y_max {
                return true;
            }
        }
    }
    false
}

const NORTH: usize = 0;
const EAST: usize = 1;
const SOUTH: usize = 2;
const WEST: usize = 3;

const QUADRANT_NW: usize = 0;
const QUADRANT_NE: usize = 1;
const QUADRANT_SW: usize = 2;
const QUADRANT_SE: usize = 3;

pub fn solve(bytes: &[u8]) -> (i64, i64) {
    // Parse
    let points = parse_points(bytes);

    let mut ver_segments = vec![];
    let mut hor_segments = vec![];

    let mut last_edge_direction = 5;
    // pqi = point quadrant inside
    let mut pqi = vec![vec![false; 4]; points.len()]; // 0: top-left, 1: top-right, 2: bottom-left, 3: bottom-right
    for i in 0..=points.len() {
        let i = i % points.len();
        let j = (i + 1) % points.len();
        let p1 = &points[i];
        let p2 = &points[j];

        // Determine edge direction: 0 north, 1 east, 2 south, 3 west
        let edge_direction = if p1.x == p2.x {
            if p1.y < p2.y {
                2
            } else {
                0
            }
        } else {
            if p1.x < p2.x {
                1
            } else {
                3
            }
        };

        // Handle i==0 case last, since last_edge_direction is undefined
        if i != 0 {
            // s = first 'inside' quadrant
            let mut s = match last_edge_direction {
                NORTH => QUADRANT_SE,
                EAST => QUADRANT_SW,
                SOUTH => QUADRANT_NW,
                WEST => QUADRANT_NE,
                _ => panic!(),
            };
            // t = final 'inside' quadrant
            let t = match edge_direction {
                NORTH => QUADRANT_NE,
                EAST => QUADRANT_SE,
                SOUTH => QUADRANT_SW,
                WEST => QUADRANT_NW,
                _ => panic!(),
            };
            while s != t {
                pqi[i][s] = true;
                s = (s + 1) % 4;
            }
            pqi[i][t] = true;            

            if p1.x == p2.x {
                ver_segments.push((p1.x, p1.y, p2.y));
            } else if p1.y == p2.y {
                hor_segments.push((p1.y, p1.x, p2.x));
            } else {
                panic!();
            }
        }
        last_edge_direction = edge_direction;
    }

    ver_segments.sort_unstable_by_key(|s| s.0);
    hor_segments.sort_unstable_by_key(|s| s.0);

    let mut part1 = 0;
    let mut part2 = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let x1 = points[i].x;
            let y1 = points[i].y;
            let x2 = points[j].x;
            let y2 = points[j].y;

            let vert_edge1 = (x1, y1, y2);
            let vert_edge2 = (x2, y1, y2);
            let hor_edge1 = (y1, x1, x2);
            let hor_edge2 = (y2, x1, x2);

            // Check for intersections (linear at first)
            let area = ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1);
            part1 = std::cmp::max(part1, area);
            let intersects = vertical_edge_inside(&hor_segments, vert_edge1)
                && vertical_edge_inside(&hor_segments, vert_edge2)
                && horizontal_edge_inside(&ver_segments, hor_edge1)
                && horizontal_edge_inside(&ver_segments, hor_edge2);
            if !intersects {
                part2 = std::cmp::max(part2, area);
            }
            // if !intersects {
            //     println!("pqi[i] = {:?}, pqi[j] = {:?}", pqi[i], pqi[j]);
            //     println!("({}, {}) -> ({}, {}), area={} intersects={}", x1, y1, x2, y2, area, intersects);
            // }
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
