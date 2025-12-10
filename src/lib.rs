use anyhow::Result;

pub fn find_matches(
    mut reader: impl std::io::BufRead,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> Result<()> {
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        if line.contains(pattern) {
            write!(writer, "{}", line)?;
        }
        line.clear();
    }
    Ok(())
}
