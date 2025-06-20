use std::collections::HashMap;

use std::io::{ErrorKind, Result};
use std::path::PathBuf;

pub type Projects = HashMap<String, usize>;

fn get_state_path() -> Result<PathBuf> {
    let mut home_dir = std::env::home_dir().ok_or(std::io::Error::new(
        ErrorKind::NotFound,
        "Couldn't find home directory",
    ))?;

    home_dir.push(".local");
    home_dir.push("state");
    home_dir.push("st_projects.json");

    Ok(home_dir)
}

pub fn load_projects() -> Result<Projects> {
    let path = get_state_path()?;
    let result = match std::fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                HashMap::default()
            } else {
                return Err(err);
            }
        }
    };

    Ok(result)
}

pub fn write_projects(input: &Projects) -> Result<()> {
    let path = get_state_path()?;
    let content = serde_json::to_string_pretty(input).unwrap();
    std::fs::write(path, content)
}
