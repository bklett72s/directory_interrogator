// Crates
use std::fs;

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

// Coordinator function with main.rs
pub fn os_walk_bridge(dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let paths: Vec<String> = os_walk(dir)?;
    Ok(paths)
}