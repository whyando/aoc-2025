use std::collections::HashMap;

pub fn read(path: &str) -> Result<Vec<u8>, std::io::Error> {
    Ok(std::fs::read(path)?)
}

fn parse(bytes: &[u8]) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for line in bytes.split(|&b| b == b'\n').filter(|line| !line.is_empty()) {
        let colon_idx = line.iter().position(|&b| b == b':').unwrap();
        let key = &line[..colon_idx];
        let value = &line[colon_idx + 2..];
        let split = value.split(|&b| b == b' ').collect::<Vec<&[u8]>>();
        let values = split
            .iter()
            .map(|s| String::from_utf8(s.to_vec()).unwrap())
            .collect();
        map.insert(String::from_utf8(key.to_vec()).unwrap(), values);
    }
    map
}

pub fn solve(bytes: &[u8]) -> (u64, u64) {
    let edges = parse(bytes);
    let edges_inv: HashMap<String, Vec<String>> = {
        let mut map = HashMap::new();
        for (key, values) in edges.iter() {
            for value in values {
                map.entry(value.clone()).or_insert(vec![]).push(key.clone());
            }
        }
        map
    };

    let part1 = count_paths(&edges_inv, "you", "out");
    let route1 = count_paths(&edges_inv, "svr", "fft")
        * count_paths(&edges_inv, "fft", "dac")
        * count_paths(&edges_inv, "dac", "out");
    let route2 = count_paths(&edges_inv, "svr", "dac")
        * count_paths(&edges_inv, "dac", "fft")
        * count_paths(&edges_inv, "fft", "out");
    (part1, route1 + route2)
}

fn count_paths(edges_inv: &HashMap<String, Vec<String>>, start: &str, end: &str) -> u64 {
    let mut cache = HashMap::new();
    f(edges_inv, start, end, &mut cache)
}

fn f(
    edges_inv: &HashMap<String, Vec<String>>,
    start: &str,
    x: &str,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if let Some(&result) = cache.get(x) {
        return result;
    }
    let result = f_inner(edges_inv, &start, x, cache);
    cache.insert(x.to_string(), result);
    result
}

fn f_inner(
    edges_inv: &HashMap<String, Vec<String>>,
    start: &str,
    x: &str,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if x == start {
        return 1;
    }
    let edges = match edges_inv.get(x) {
        Some(edges) => edges,
        None => return 0,
    };
    edges.iter().map(|y| f(edges_inv, start, y, cache)).sum()
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
