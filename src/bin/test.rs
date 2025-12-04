use aoc_2025::day01;

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

fn main() {
    let aoc_input = std::env::var("AOC_INPUT").unwrap();
    let input = std::fs::read_to_string(aoc_input).unwrap();
    let (p1, p2) = solve_asm_wrapper(&input);
    // Touch `day01::solve` so this binary depends on the Rust crate, which
    // pulls in the build script's native assembly library (day01_solve_asm).
    let _ = day01::solve as fn(&[u8]) -> (i32, i32);
    println!(r#"{{"part_one": {}, "part_two": {}}}"#, p1, p2);
}
