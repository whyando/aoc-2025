use std::collections::HashMap;

pub fn read(path: &str) -> Result<Vec<u8>, std::io::Error> {
    Ok(std::fs::read(path)?)
}

// Hardcoded IDs for special nodes
const ID_YOU: usize = 0;
const ID_OUT: usize = 1;
const ID_SVR: usize = 2;
const ID_FFT: usize = 3;
const ID_DAC: usize = 4;

fn parse_key(bytes: &[u8; 3]) -> usize {
    (bytes[0] - b'a') as usize * 26 * 26
        + (bytes[1] - b'a') as usize * 26
        + (bytes[2] - b'a') as usize
}

fn parse(bytes: &[u8]) -> Vec<Vec<usize>> {
    let mut string_to_id = HashMap::new();
    let mut next_id = 5usize; // Start after reserved IDs

    // Initialize reserved IDs
    string_to_id.insert(parse_key(b"you"), ID_YOU);
    string_to_id.insert(parse_key(b"out"), ID_OUT);
    string_to_id.insert(parse_key(b"svr"), ID_SVR);
    string_to_id.insert(parse_key(b"fft"), ID_FFT);
    string_to_id.insert(parse_key(b"dac"), ID_DAC);

    // Initialize vector with empty vecs for reserved IDs
    let mut edges_inv = vec![Vec::new(); 5];

    for line in bytes.split(|&b| b == b'\n').filter(|line| !line.is_empty()) {
        let colon_idx = line.iter().position(|&b| b == b':').unwrap();
        let key_bytes: [u8; 3] = line[..colon_idx].try_into().unwrap();
        let value_bytes = &line[colon_idx + 2..];

        let key = parse_key(&key_bytes);
        let key_id = *string_to_id.entry(key).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            edges_inv.push(Vec::new());
            id
        });

        let values = value_bytes
            .split(|&b| b == b' ')
            .map(|s| parse_key(s.try_into().unwrap()))
            .collect::<Vec<_>>();

        for value_str in values {
            let value_id = *string_to_id.entry(value_str).or_insert_with(|| {
                let id = next_id;
                next_id += 1;
                edges_inv.push(Vec::new());
                id
            });
            edges_inv[value_id as usize].push(key_id);
        }
    }

    edges_inv
}

const ENTER: bool = false;
const EXIT: bool = true;

pub fn solve(bytes: &[u8]) -> (u64, u64) {
    let edges_inv = parse(bytes);
    let n = edges_inv.len();

    // Topological sort the graph (DFS)
    let mut stack = vec![];
    let mut order = Vec::with_capacity(n);
    let mut state = vec![0; n];

    for i in 0..n {
        stack.push((i, ENTER));
    }

    while let Some((i, dir)) = stack.pop() {
        match dir {
            ENTER => {
                if state[i] == 0 {
                    state[i] = 1;
                    stack.push((i, EXIT));
                    for &j in &edges_inv[i] {
                        stack.push((j, ENTER));
                    }
                }
            }
            EXIT => {
                order.push(i);
                state[i] = 2;
            }
        }
    }

    let mut paths_from_you = vec![0; n];
    let mut paths_from_svr = vec![0; n];
    let mut paths_from_fft = vec![0; n];
    let mut paths_from_dac = vec![0; n];
    paths_from_you[ID_YOU] = 1;
    paths_from_svr[ID_SVR] = 1;
    paths_from_fft[ID_FFT] = 1;
    paths_from_dac[ID_DAC] = 1;
    for x in order.iter() {
        for &y in &edges_inv[*x] {
            paths_from_you[*x] += paths_from_you[y];
            paths_from_svr[*x] += paths_from_svr[y];
            paths_from_fft[*x] += paths_from_fft[y];
            paths_from_dac[*x] += paths_from_dac[y];
        }
    }
    let part1 = paths_from_you[ID_OUT];
    let part2 = paths_from_svr[ID_FFT] * paths_from_fft[ID_DAC] * paths_from_dac[ID_OUT];

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = read("inputs/11.txt").unwrap();
        assert_eq!(solve(&input), (786, 495845045016588));
    }

    #[test]
    fn test_part1() {
        let test_input = b"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!(solve(&test_input.to_vec()), (5, 0));
    }

    #[test]
    fn test_part2() {
        let test_input = b"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!(solve(&test_input.to_vec()), (0, 2));
    }
}
