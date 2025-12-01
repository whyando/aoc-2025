
fn main() {
    let aoc_day = std::env::var("AOC_DAY").unwrap();
    let aoc_input = std::env::var("AOC_INPUT").unwrap();

    let input = std::fs::read_to_string(aoc_input).unwrap();

    if aoc_day == "1" {
        let (p1, p2) = aoc_2025::day01::solve(&input);
        println!(r#"{{"part_one": {}, "part_two": {}}}"#, p1, p2);
    } else {
        println!("\"not implemented\"");
        std::process::exit(0);
    }
}
