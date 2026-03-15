use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};

// Coordination function with main.rs
pub fn hash_file_bridge(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let hash_val =hash_file(file_path)?;
    Ok(hash_val)
}

// Function to hash a file using SHA-256
fn hash_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}