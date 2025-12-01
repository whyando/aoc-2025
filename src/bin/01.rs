use std::io::Read as _;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap(); // read all of stdin

    let p1 = part1(&input);
    let p2 = part2(&input);
    println!(r#"{{"part_one": {}, "part_two": {}}}"#, p1, p2);
}

fn parse(input_string: &str) -> Vec<(char, i32)> {
    input_string
        .lines()
        .map(|line| {
            let (direction, steps) = line.split_at(1);
            let direction = direction.chars().next().unwrap();
            let steps = steps.parse::<i32>().unwrap();
            (direction, steps)
        })
        .collect::<Vec<_>>()
}

fn part1(input_string: &str) -> i32 {
    let rotations = parse(input_string);

    let mut result = 0;
    let mut x = 50;
    for (direction, steps) in rotations {
        match direction {
            'L' => x -= steps,
            'R' => x += steps,
            _ => panic!("Invalid direction: {}", direction),
        }
        x = (x + 100) % 100;
        if x == 0 {
            result += 1;
        }
    }
    result
}

fn part2(input_string: &str) -> i32 {
    let rotations = parse(input_string);

    let mut result = 0;
    let mut x = 50;
    for (direction, steps) in rotations {
        let dir = match direction {
            'L' => -1,
            'R' => 1,
            _ => panic!("Invalid direction: {}", direction),
        };

        for _ in 0..steps {
            x += dir;
            if x % 100 == 0 {
                result += 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = std::fs::read_to_string("inputs/01.txt").unwrap();
        assert_eq!(part1(&input), 1120);
        assert_eq!(part2(&input), 6554);
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
        assert_eq!(part1(test_input), 3);
        assert_eq!(part2(test_input), 6);
    }
}
