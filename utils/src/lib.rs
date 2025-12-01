pub fn read_input(day: u32, debug: bool) -> Result<String, std::io::Error> {
    let folder = if debug { "debug" } else { "input" };
    let path = format!("{}/day{:02}.txt", folder, day);
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}
