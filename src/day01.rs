fn parse_i32_from_bytes(bytes: &[u8]) -> i32 {
    let mut n = 0i32;
    for &b in bytes {
        if b'0' <= b && b <= b'9' {
            n = n * 10 + (b - b'0') as i32;
        }
    }
    n
}

pub fn solve(input: &[u8]) -> (i32, i32) {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut x = 100_050;
    for line in input.split(|&b| b == b'\n') {
        if line.is_empty() {
            continue;
        }
        let steps = parse_i32_from_bytes(&line[1..]);
        let dir = match line[0] {
            b'L' => -1,
            _ => 1,
        };

        let x1 = x + steps * dir;
        let passes = if dir == -1 {
            (((x - 1) / 100) - ((x1 - 1) / 100)).abs()
        } else {
            ((x1 / 100) - (x / 100)).abs()
        };

        if x1 % 100 == 0 {
            part1 += 1;
        }
        part2 += passes;
        x = x1;
    }
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe extern "C" {
        // From `day01_asm.s`:
        // void day01_solve_asm(const unsigned char *ptr, unsigned long len,
        //                       int *out_part1, int *out_part2);
        fn day01_solve_asm(ptr: *const u8, len: usize, out_part1: *mut i32, out_part2: *mut i32);
    }

    fn solve_asm_wrapper(input: &str) -> (i32, i32) {
        let mut part1 = 0i32;
        let mut part2 = 0i32;
        unsafe {
            day01_solve_asm(
                input.as_ptr(),
                input.len(),
                &mut part1 as *mut i32,
                &mut part2 as *mut i32,
            );
        }
        (part1, part2)
    }

    #[test]
    fn test() {
        let input = std::fs::read("inputs/01.txt").unwrap();
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

        assert_eq!(solve(test_input.as_bytes()), (3, 6));
    }

    #[test]
    fn test_asm() {
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
        let rust_result = solve(test_input.as_bytes());
        let asm_result = solve_asm_wrapper(&test_input);
        println!("Rust: {:?}, ASM: {:?}", rust_result, asm_result);
        assert_eq!(asm_result, rust_result);

        let input = std::fs::read("inputs/01.txt").unwrap();
        let rust_result = solve(&input);
        let input_str = String::from_utf8_lossy(&input);
        let asm_result = solve_asm_wrapper(&input_str);
        println!("Rust: {:?}, ASM: {:?}", rust_result, asm_result);
        assert_eq!(asm_result, rust_result);
    }
}
