use anyhow::Result;

pub fn read_lines(path: &str) -> Result<Vec<String>> {
    let file_contents = std::fs::read_to_string(path)?;
    let mut lines = file_contents
        .split("\n")
        .map(|r| r.to_owned())
        .collect::<Vec<_>>();
    lines.truncate(lines.len().saturating_sub(1));
    Ok(lines)
}
