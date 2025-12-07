fn main() {
    let aoc_day = std::env::var("AOC_DAY").unwrap();
    let aoc_input = std::env::var("AOC_INPUT").unwrap();

    if aoc_day == "1" {
        let input = aoc_2025::file::read(&aoc_input).unwrap();
        let (p1, p2) = aoc_2025::day01::solve(&input);
        println!(r#"{{"part_one": {}, "part_two": {}}}"#, p1, p2);
    } else if aoc_day == "2" {
        let input = std::fs::read_to_string(aoc_input).unwrap();
        let (p1, p2) = aoc_2025::day02::solve(&input);
        println!(r#"{{"part_one": {}, "part_two": {}}}"#, p1, p2);
    } else if aoc_day == "3" {
        let input = aoc_2025::file::read(&aoc_input).unwrap();
        let (p1, p2) = aoc_2025::day03::solve(&input);
        println!(r#"{{"part_one": {}, "part_two": {}}}"#, p1, p2);
    } else if aoc_day == "4" {
        let input = aoc_2025::file::read_no_newlines(&aoc_input).unwrap();
        let (p1, p2) = aoc_2025::day04::solve(&input);
        println!(r#"{{"part_one": {}, "part_two": {}}}"#, p1, p2);
    } else if aoc_day == "5" {
        let input = aoc_2025::file::read(&aoc_input).unwrap();
        let (p1, p2) = aoc_2025::day05::solve(&input);
        println!(r#"{{"part_one": {}, "part_two": {}}}"#, p1, p2);
    } else if aoc_day == "6" {
        let input = aoc_2025::day06::read(&aoc_input).unwrap();
        let (p1, p2) = aoc_2025::day06::solve::<4>(&input);
        println!(r#"{{"part_one": {}, "part_two": {}}}"#, p1, p2);
    } else if aoc_day == "7" {
        let input = aoc_2025::day07::read(&aoc_input).unwrap();
        let (p1, p2) = aoc_2025::day07::solve::<141>(&input);
        println!(r#"{{"part_one": {}, "part_two": {}}}"#, p1, p2);
    } else {
        println!("\"not implemented\"");
        std::process::exit(0);
    }
}
