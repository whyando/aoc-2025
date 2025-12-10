pub fn read(path: &str) -> Result<Vec<u8>, std::io::Error> {
    Ok(std::fs::read(path)?)
}

pub fn solve(_bytes: &[u8]) -> (u64, u64) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test() {
        let input = read("inputs/11.txt").unwrap();
        assert_eq!(solve(&input), (0, 0));
    }

    #[test]
    fn test_part1() {
        let test_input = b"";
        assert_eq!(solve(&test_input.to_vec()), (0, 0));
    }
}
