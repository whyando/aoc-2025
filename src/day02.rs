use std::collections::HashSet;

pub fn decimal_length(mut x: i64) -> u32 {
    if x == 0 {
        return 1;
    }
    let mut length = 0;
    while x > 0 {
        x /= 10;
        length += 1;
    }
    length
}

fn min_value(num_digits: u32) -> i64 {
    10_i64.pow(num_digits - 1)
}

fn max_value(num_digits: u32) -> i64 {
    10_i64.pow(num_digits) - 1
}

fn partition_range(range: Range) -> Vec<(i64, i64, u32)> {
    let num_digits_1 = decimal_length(range.start);
    let num_digits_2 = decimal_length(range.end);

    let mut sections = Vec::new();
    for num_digits in num_digits_1..=num_digits_2 {
        let min = std::cmp::max(range.start, min_value(num_digits));
        let max = std::cmp::min(range.end, max_value(num_digits));
        sections.push((min, max, num_digits));
    }
    sections
}

fn repeat_digits(x: i64, x_len: u32, n: u32) -> i64 {
    let mut result = x;
    for _ in 0..n - 1 {
        result = result * 10_i64.pow(x_len) + x;
    }
    result
}

fn first_n_digits(x: i64, x_len: u32, n: u32) -> i64 {
    let mut y = x;
    for _ in 0..x_len - n {
        y /= 10;
    }
    y
}

pub fn solve(input: &str) -> (i64, i64) {
    let ranges = parse_input(input);

    let mut part1 = 0;
    let mut part2 = 0;
    for range in ranges {
        // 1. Partition range into sections based on decimal length
        let sections = partition_range(range);
        // println!("sections: {:?}", sections);

        // 2. consider divisors of 'num_digits'
        for (min, max, num_digits) in sections {
            let mut invalid_p2 = HashSet::new();

            for divisor in 2..=num_digits {
                if num_digits % divisor != 0 {
                    continue;
                }
                let sz = num_digits / divisor;

                // start = first 'sz' digits of 'min'
                let start = first_n_digits(min, num_digits, sz);
                let end = first_n_digits(max, num_digits, sz);

                for i in start..=end {
                    // let x = i repeated 'divisor' times
                    let x = repeat_digits(i, sz, divisor);
                    let in_range = x >= min && x <= max;
                    // println!("i: {} in_range: {}", i, in_range);
                    // if !in_range && (i != start && i != end) {
                    //     panic!("this shouldn't happen: i: {} start: {} end: {}", i, start, end);
                    // }
                    if in_range {
                        if divisor == 2 {
                            part1 += x;
                        }
                        invalid_p2.insert(x);
                    }
                }

                // println!("sz: {}, start: {}, end: {}", sz, start, end);
            }
            for invalid in invalid_p2 {
                part2 += invalid;
            }
        }
    }
    (part1, part2)
}

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

fn parse_input(input: &str) -> Vec<Range> {
    input
        .split(',')
        .filter_map(|chunk| {
            let chunk = chunk.trim();
            if chunk.is_empty() {
                return None;
            }

            // Expect "start-end"
            let (start_str, end_str) = chunk.split_once('-')?;
            let start = start_str.trim().parse().ok()?;
            let end = end_str.trim().parse().ok()?;

            Some(Range { start, end })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = std::fs::read_to_string("inputs/02.txt").unwrap();
        assert_eq!(solve(&input), (24043483400, 38262920235));
    }

    #[test]
    fn test_part1() {
        let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(solve(&test_input), (1227775554, 4174379265));
    }
}
