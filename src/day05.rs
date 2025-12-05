#[inline(always)]
fn f(part2: &mut i64, x: &mut i64, i: (i64, i64)) {
    if i.1 > *x {
        *part2 += i.1 - std::cmp::max(i.0, *x + 1) + 1;
        *x = i.1;
    }
}

pub fn solve(bytes: &[u8]) -> (i64, i64) {
    let mut part1 = 0;
    let mut part2 = 0;

    let (mut ranges, mut ingredients) = parse_input(bytes);
    ranges.sort_unstable_by_key(|r| r.0);
    ingredients.sort_unstable();

    let mut x = 0;
    let mut range_iter = ranges.into_iter();
    let mut i = range_iter.next().unwrap();
    f(&mut part2, &mut x, i);
    for y in ingredients {
        // Increment range iterator, until s <= x <= t, OR s > x
        loop {
            if i.0 <= y && i.1 >= y {
                part1 += 1;
                break;
            }
            if i.0 > y {
                break;
            }
            let Some(i2) = range_iter.next() else {
                return (part1, part2);
            };
            i = i2;
            f(&mut part2, &mut x, i);
        }
    }
    for i in range_iter {
        f(&mut part2, &mut x, i);
    }
    (part1, part2)
}

fn parse_i64(bytes: &[u8]) -> i64 {
    let mut num = 0i64;
    for &b in bytes {
        num = num * 10 + (b - b'0') as i64;
    }
    num
}

fn parse_input(input: &[u8]) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut ranges = Vec::new();
    let mut numbers = Vec::new();

    // split by newlines, section 1
    let mut lines = input.split(|&b| b == b'\n');
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let range = line.split(|&b| b == b'-').collect::<Vec<&[u8]>>();
        ranges.push((parse_i64(range[0]), parse_i64(range[1])));
    }

    // split by newlines, section 2
    for line in lines {
        numbers.push(parse_i64(line));
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
