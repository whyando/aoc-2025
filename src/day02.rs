

pub fn solve(input: &str) -> (i64, i64) {
    let ranges = parse_input(input);

    let mut part1 = 0;
    let mut part2 = 0;
    for range in ranges {    
        // Trivial solution: Check every value in the range for validity
        for i in range.start..=range.end {
            if is_invalid_part1(i) {
                part1 += i;
            }
            if is_invalid_part2(i) {
                part2 += i;
            }
        }
    }
    (part1, part2)
}

fn is_invalid_part1(x: i64) -> bool {
    let s: String = x.to_string();
    
    if s.len() % 2 != 0 {
        return false;
    }
    let sz = s.len() / 2;

    let x = s[0..sz].parse::<i64>().unwrap();
    let y = s[sz..].parse::<i64>().unwrap();
    x == y
}


fn is_invalid_part2(x: i64) -> bool {
    let s: String = x.to_string();
    
    for n in 2..=s.len() {
        if s.len() % n != 0 {
            continue;
        }
        let sz = s.len() / n;

        let x = s[0..sz].parse::<i64>().unwrap();
        let mut invalid = true;
        for m in 1..n {
            let y = s[m*sz..(m+1)*sz].parse::<i64>().unwrap();
            if x != y {
                invalid = false;
                break; 
            }
        }
        if invalid {
            return true;
        }
    }
    false
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
