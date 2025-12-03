
pub fn solve(bytes: &[u8]) -> (i64, i64) {
    let mut part1 = 0;
    let mut part2 = 0;
    
    // Process line by line
    let mut start = 0;
    while start < bytes.len() {
        // Find end of line (newline or end of input)
        let mut end = start;
        while end < bytes.len() && bytes[end] != b'\n' {
            end += 1;
        }
        
        if start < end {
            let line = &bytes[start..end];
            
            // Part 1 - Find largest digit in the line (excluding the last digit)
            let mut largest_byte = b'0';
            let mut largest_index = 0;
            for i in 0..line.len() - 1 {
                if line[i] > largest_byte {
                    largest_byte = line[i];
                    largest_index = i;
                }
            }
            let mut largest2_byte = b'0';
            for i in largest_index + 1..line.len() {
                if line[i] > largest2_byte {
                    largest2_byte = line[i];
                }
            }
            let largest = (largest_byte - b'0') as i64;
            let largest2 = (largest2_byte - b'0') as i64;
            part1 += 10 * largest + largest2;

            // Part 2
            let mut min_index = 0;
            let mut sum = 0;
            for digit in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12] {
                // Find largest digit in line, exclude the last (12-digit) digits
                let mut largest_byte = b'0';
                let mut largest_index = 0;
                for i in min_index..line.len() - (12 - digit) {
                    if line[i] > largest_byte {
                        largest_byte = line[i];
                        largest_index = i;
                    }
                }
                min_index = largest_index + 1;
                let largest = (largest_byte - b'0') as i64;
                sum = sum * 10 + largest;
            }
            part2 += sum;
        }
        
        start = end + 1;  // Move past newline
    }
    
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = std::fs::read("inputs/03.txt").unwrap();
        assert_eq!(solve(&input), (17087, 169019504359949));
    }

    #[test]
    fn test_part1() {
        let test_input = b"987654321111111
811111111111119
234234234234278
818181911112111
";

        assert_eq!(solve(test_input), (357, 3121910778619));
    }
}
