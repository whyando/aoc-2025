fn main() {
    let aoc_day = std::env::var("AOC_DAY").unwrap();
    let aoc_input = std::env::var("AOC_INPUT").unwrap();
    // let aoc_day = "1";
    // let aoc_input = "inputs/01.txt";

    let input = std::fs::read_to_string(aoc_input).unwrap();

    if aoc_day == "1" {
        let (p1, p2) = aoc_2025::day01::solve(&input);
        println!(r#"{{"part_one": {}, "part_two": {}}}"#, p1, p2);
    } else if aoc_day == "2" {
        let (p1, p2) = aoc_2025::day02::solve(&input);
        println!(r#"{{"part_one": {}, "part_two": {}}}"#, p1, p2);
    } else if aoc_day == "3" {
        let (p1, p2) = aoc_2025::day03::solve(&input);
        println!(r#"{{"part_one": {}, "part_two": {}}}"#, p1, p2);
    } else {
        println!("\"not implemented\"");
        std::process::exit(0);
    }
}
