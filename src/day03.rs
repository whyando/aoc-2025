pub fn solve(input: &str) -> (i64, i64) {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in lines {
        // Part 1
        // Find largest digit in the line (excluding the last digit)
        let mut largest = 0;
        let mut largest_index = 0;
        for i in 0..line.len() - 1 {
            let digit = line[i..i + 1].parse::<i64>().unwrap();
            if digit > largest {
                largest = digit;
                largest_index = i;
            }
        }
        let mut largest2 = 0;
        for i in largest_index + 1..line.len() {
            largest2 = std::cmp::max(largest2, line[i..i + 1].parse::<i64>().unwrap());
        }
        println!("largest: {}, largest2: {}", largest, largest2);
        part1 += 10 * largest + largest2;

        // Part 2
        let mut min_index = 0;
        let mut sum = 0;
        for digit in 1..=12 {
            // Find largest digit in line, exclude the last (12-digit) digits
            let mut largest = 0;
            let mut largest_index = 0;
            for i in min_index..line.len() - (12 - digit) {
                let digit = line[i..i + 1].parse::<i64>().unwrap();
                if digit > largest {
                    largest = digit;
                    largest_index = i;
                }
            }
            min_index = largest_index + 1;
            sum = sum * 10 + largest;
        }
        println!("sum: {}", sum);
        part2 += sum;
    }
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = std::fs::read_to_string("inputs/03.txt").unwrap();
        assert_eq!(solve(&input), (17087, 169019504359949));
    }

    #[test]
    fn test_part1() {
        let test_input = "987654321111111
811111111111119
234234234234278
818181911112111
";

        assert_eq!(solve(&test_input), (357, 3121910778619));
    }
}
