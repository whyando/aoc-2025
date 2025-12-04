pub fn read(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut out = Vec::new();
    let mut buf = [0u8; 8192];

    loop {
        let n = std::io::Read::read(&mut file, &mut buf)?;
        if n == 0 {
            break;
        }

        out.extend(buf[..n].iter().copied());
    }

    // Remove trailing newline
    if out.last() == Some(&b'\n') {
        out.pop();
    }
    Ok(out)
}

pub fn read_no_newlines(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut out = Vec::new();
    let mut buf = [0u8; 8192];

    loop {
        let n = std::io::Read::read(&mut file, &mut buf)?;
        if n == 0 {
            break;
        }

        out.extend(
            buf[..n]
                .iter()
                .copied()
                .filter(|b| *b != b'\n' && *b != b'\r'),
        );
    }

    Ok(out)
}
