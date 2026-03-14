// Crates
use std::fs;
use std::path::Path;

fn os_walk(dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            paths.extend(os_walk(path.to_str().unwrap())?); // Recursive call for subdirectories
        }
        paths.push(path.display().to_string());
    }
    Ok(paths)
}

pub fn os_walk_bridge(dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut paths: Vec<String> = Vec::new();
    paths = os_walk(dir)?;
    Ok(paths)
}