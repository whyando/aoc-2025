pub fn solve(input: &str) -> (i32, i32) {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut x = 50;
    for line in input.lines() {
        let (direction, steps) = line.split_at(1);
        let direction = direction.chars().next().unwrap();
        let steps = steps.parse::<i32>().unwrap();
        let dir = match direction {
            'L' => -1,
            'R' => 1,
            _ => panic!("Invalid direction: {}", direction),
        };

        for _ in 0..steps {
            x += dir;
            if x % 100 == 0 {
                part2 += 1;
            }
        }
        if x % 100 == 0 {
            part1 += 1;
        }
    }
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = std::fs::read_to_string("inputs/01.txt").unwrap();
        assert_eq!(solve(&input), (1120, 6554));
    }

    #[test]
    fn test_part1() {
        let test_input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(solve(&test_input), (3, 6));
    }
}
