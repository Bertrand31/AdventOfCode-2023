use anyhow::Result;

pub fn read_lines(path: &str) -> Result<Vec<String>> {
    let contents = std::fs::read_to_string(path)?;
    Ok(contents.lines().map(|s| s.to_owned()).collect::<Vec<_>>())
}
