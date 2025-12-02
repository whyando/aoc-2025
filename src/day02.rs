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

fn sum_from_a_to_b(a: i64, b: i64) -> i64 {
    if a > b {
        return 0;
    }
    sum_to_n(b) - sum_to_n(a - 1)
}

fn sum_to_n(n: i64) -> i64 {
    n * (n + 1) / 2
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
            // Inclusion exclusion fun
            let divisors = match num_digits {
                1 => vec![],
                2 => vec![(1, 2)],
                3 => vec![(1, 3)],
                4 => vec![(1, 2)],
                5 => vec![(1, 5)],
                6 => vec![(1, 2), (1, 3), (-1, 6)],
                7 => vec![(1, 7)],
                8 => vec![(1, 2)],
                9 => vec![(1, 3)],
                10 => vec![(1, 2), (1, 5), (-1, 10)],
                _ => panic!(),
            };

            for (sign, divisor) in divisors {
                let sz = num_digits / divisor;

                // start = first 'sz' digits of 'min'
                let start = first_n_digits(min, num_digits, sz);
                let end = first_n_digits(max, num_digits, sz);

                let mut sum = sum_from_a_to_b(start + 1, end - 1);
                // Check inclusion of the start and end values separately
                let start_val = repeat_digits(start, sz, divisor);
                let end_val = repeat_digits(end, sz, divisor);
                if min <= start_val && start_val <= max {
                    sum += start;
                }
                if start != end && min <= end_val && end_val <= max {
                    sum += end;
                }
                sum *= repeat_digits(1, sz, divisor);

                if divisor == 2 {
                    part1 += sum;
                }
                part2 += sign * sum;
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
