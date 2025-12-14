use std::collections::HashMap;

pub fn read(path: &str) -> Result<Vec<u8>, std::io::Error> {
    Ok(std::fs::read(path)?)
}

// Hardcoded IDs for special nodes
const ID_YOU: u32 = 0;
const ID_OUT: u32 = 1;
const ID_SVR: u32 = 2;
const ID_FFT: u32 = 3;
const ID_DAC: u32 = 4;

fn parse_key(bytes: &[u8; 3]) -> u32 {
    (bytes[0] - b'a') as u32 * 26 * 26 + (bytes[1] - b'a') as u32 * 26 + (bytes[2] - b'a') as u32
}

fn parse(bytes: &[u8]) -> Vec<Vec<u32>> {
    let mut string_to_id = HashMap::new();
    let mut next_id = 5u32; // Start after reserved IDs

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

pub fn solve(bytes: &[u8]) -> (u64, u64) {
    let edges_inv = parse(bytes);

    let part1 = count_paths(&edges_inv, ID_YOU, ID_OUT);

    let route1 = count_paths(&edges_inv, ID_SVR, ID_FFT)
        * count_paths(&edges_inv, ID_FFT, ID_DAC)
        * count_paths(&edges_inv, ID_DAC, ID_OUT);

    let route2 = count_paths(&edges_inv, ID_SVR, ID_DAC)
        * count_paths(&edges_inv, ID_DAC, ID_FFT)
        * count_paths(&edges_inv, ID_FFT, ID_OUT);

    (part1, route1 + route2)
}

fn count_paths(edges_inv: &[Vec<u32>], start: u32, end: u32) -> u64 {
    let mut cache = HashMap::new();
    f(edges_inv, start, end, &mut cache)
}

fn f(edges_inv: &[Vec<u32>], start: u32, x: u32, cache: &mut HashMap<u32, u64>) -> u64 {
    if let Some(&result) = cache.get(&x) {
        return result;
    }
    let result = f_inner(edges_inv, start, x, cache);
    cache.insert(x, result);
    result
}

fn f_inner(edges_inv: &[Vec<u32>], start: u32, x: u32, cache: &mut HashMap<u32, u64>) -> u64 {
    if x == start {
        return 1;
    }
    let x_idx = x as usize;
    if x_idx >= edges_inv.len() {
        return 0;
    }
    let edges = &edges_inv[x_idx];
    edges.iter().map(|&y| f(edges_inv, start, y, cache)).sum()
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
