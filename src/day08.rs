use std::collections::LinkedList;

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

pub fn solve(bytes: &[u8], num_connections: usize) -> (i64, i64) {
    let mut part1 = 0;

    let points = bytes
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts = line
                .split(|&b| b == b',')
                .map(parse_i64)
                .collect::<Vec<i64>>();
            (parts[0], parts[1], parts[2])
        })
        .collect::<Vec<(i64, i64, i64)>>();

    // Label all the points + use linked lists to track the size of each connected component
    let mut merges = 0;
    let mut label = vec![0; points.len()];
    let mut label_ll = vec![LinkedList::new(); points.len()];
    for p_idx in 0..points.len() {
        label[p_idx] = p_idx;
        label_ll[p_idx].push_back(p_idx);
    }

    // Generate list of all edges, and sort by length
    let mut edges = Vec::new();
    for p_idx in 0..points.len() {
        for q_idx in p_idx + 1..points.len() {
            let p = points[p_idx];
            let q = points[q_idx];
            let dist = ((p.0 - q.0).pow(2) + (p.1 - q.1).pow(2) + (p.2 - q.2).pow(2)) as u64;
            edges.push((p_idx + 0x10000 * q_idx, dist));
        }
    }
    edges.sort_unstable_by_key(|e| e.1);

    for (edge_idx, edge) in edges.iter().enumerate() {
        // Part 1: Find the 3 largest connected components
        if edge_idx == num_connections {
            let mut component_sizes = label_ll.iter().map(|ll| ll.len()).collect::<Vec<usize>>();
            component_sizes.sort_unstable_by_key(|&size| std::cmp::Reverse(size));
            part1 = component_sizes[0] * component_sizes[1] * component_sizes[2];
        }
        let p_idx = edge.0 & 0xFFFF;
        let q_idx = edge.0 >> 16;

        if label[p_idx] != label[q_idx] {
            // Merge the two labels
            let p_label = label[p_idx];
            let q_label = label[q_idx];
            if label_ll[p_label].len() < label_ll[q_label].len() {
                for q_idx in label_ll[q_label].iter() {
                    label[*q_idx] = p_label;
                }
                let mut q_list = std::mem::take(&mut label_ll[q_label]);
                label_ll[p_label].append(&mut q_list);
            } else {
                for p_idx in label_ll[p_label].iter() {
                    label[*p_idx] = q_label;
                }
                let mut p_list = std::mem::take(&mut label_ll[p_label]);
                label_ll[q_label].append(&mut p_list);
            }
            merges += 1;

            if merges == points.len() - 1 {
                let part2 = points[p_idx].0 * points[q_idx].0;
                return (part1 as i64, part2);
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = read("inputs/08.txt").unwrap();
        assert_eq!(solve(&input, 1000), (352584, 9617397716));
    }

    #[test]
    fn test_part1() {
        let test_input = b"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(solve(&test_input.to_vec(), 10), (40, 25272));
    }
}
