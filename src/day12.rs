pub fn read(path: &str) -> Result<Vec<u8>, std::io::Error> {
    Ok(std::fs::read(path)?)
}

#[derive(Debug)]
struct Shape {
    area: usize,
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

#[derive(Debug)]
struct Problem {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

fn parse_input(bytes: &[u8]) -> Problem {
    // Split on double newline (\n\n)
    let mut sections = Vec::new();
    let mut start = 0;
    let mut i = 0;

    while i < bytes.len() {
        if i + 1 < bytes.len() && bytes[i] == b'\n' && bytes[i + 1] == b'\n' {
            sections.push(&bytes[start..i]);
            start = i + 2;
            i += 2;
        } else {
            i += 1;
        }
    }
    if start < bytes.len() {
        sections.push(&bytes[start..]);
    }

    let mut shapes = Vec::new();
    // Parse first 6 sections as shapes (each is 3x3)
    for section in sections.iter().take(6) {
        let lines: Vec<&[u8]> = section
            .split(|&b| b == b'\n')
            .filter(|line| !line.is_empty())
            .collect();
        // Skip the header line (e.g., "0:") and take 3 lines
        let shape_lines: Vec<Vec<u8>> = lines
            .iter()
            .skip(1)
            .take(3)
            .map(|line| line.to_vec())
            .collect();
        let area = shape_lines.iter().flatten().filter(|&&b| b == b'#').count();
        shapes.push(Shape { area });
    }

    // Parse remaining sections as regions
    let mut regions = Vec::new();
    for section in sections.iter().skip(6) {
        let lines: Vec<&[u8]> = section
            .split(|&b| b == b'\n')
            .filter(|line| !line.is_empty())
            .collect();
        for line in lines {
            // Parse region line: "WxH: count1 count2 ..."
            if line.contains(&b'x') && line.contains(&b':') {
                let parts: Vec<&[u8]> = line.split(|&b| b == b':').collect();
                if parts.len() == 2 {
                    let dim_part = parts[0];
                    let counts_part = parts[1];

                    // Parse dimensions
                    let dim_str = String::from_utf8_lossy(dim_part);
                    if let Some(x_pos) = dim_str.find('x') {
                        if let (Ok(width), Ok(height)) = (
                            dim_str[..x_pos].parse::<usize>(),
                            dim_str[x_pos + 1..].trim().parse::<usize>(),
                        ) {
                            // Parse counts
                            let counts: Vec<usize> = counts_part
                                .split(|&b| b == b' ')
                                .filter(|s| !s.is_empty())
                                .filter_map(|s| String::from_utf8_lossy(s).parse().ok())
                                .collect();

                            regions.push(Region {
                                width,
                                height,
                                counts,
                            });
                        }
                    }
                }
            }
        }
    }

    Problem { shapes, regions }
}

pub fn solve(bytes: &[u8]) -> (u64, u64) {
    let problem = parse_input(bytes);

    let mut part1 = 0;
    let mut part2 = 0;
    for region in problem.regions {
        let mut best_case = 0;
        let mut worst_case = 0;
        for (i, count) in region.counts.iter().enumerate() {
            best_case += count * problem.shapes[i].area;
            worst_case += count * 9;
        }
        let region_area = region.width * region.height;
        if region_area >= worst_case {
            // Possible (probably), only guaranteed if one of the sides is a multiple of 3
            part1 += 1;
        } else if region_area < best_case {
            // Impossible
            part2 += 0;
        } else {
            panic!("unknown case");
        }
    }

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = read("inputs/12.txt").unwrap();
        assert_eq!(solve(&input), (454, 0));
    }

    #[ignore]
    #[test]
    fn test_part1() {
        let test_input = b"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
        assert_eq!(solve(&test_input.to_vec()), (0, 0));
    }
}
