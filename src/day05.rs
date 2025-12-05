pub fn solve(bytes: &[u8]) -> (i64, i64) {
    let mut part1 = 0;
    let mut part2 = 0;

    let (mut ranges, mut ingredients) = parse_input(bytes);
    ranges.sort_unstable_by_key(|r| r.0);
    ingredients.sort_unstable();

    // part1
    let mut i = 0;
    let n = ranges.len();
    for x in ingredients {
        // Increment range iterator, until s <= x <= t, OR s > x
        while i != n {
            if ranges[i as usize].0 <= x && ranges[i as usize].1 >= x {
                part1 += 1;
                break;
            }
            if ranges[i as usize].0 > x {
                break;
            }
            i += 1;
        }
    }

    // part2
    let mut x = 0;
    for (s, t) in ranges {
        if t > x {
            part2 += t - std::cmp::max(s, x + 1) + 1;
            x = t;
        }
    }

    (part1, part2)
}

fn parse_i64(bytes: &[u8]) -> i64 {
    let mut num = 0i64;
    for &b in bytes {
        if b.is_ascii_digit() {
            num = num * 10 + (b - b'0') as i64;
        }
    }
    num
}

fn parse_input(input: &[u8]) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut ranges = Vec::new();
    let mut numbers = Vec::new();

    // split by newlines
    let mut lines = input.split(|&b| b == b'\n');
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let range = line.split(|&b| b == b'-').collect::<Vec<&[u8]>>();
        ranges.push((parse_i64(range[0]), parse_i64(range[1])));
    }

    // split by spaces
    for line in lines {
        let x = parse_i64(line);
        numbers.push(x);
    }
    (ranges, numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = crate::file::read("inputs/05.txt").unwrap();
        assert_eq!(solve(&input), (643, 342018167474526));
    }

    #[test]
    fn test_part1() {
        let test_input = b"3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(solve(&test_input.to_vec()), (3, 14));
    }
}
