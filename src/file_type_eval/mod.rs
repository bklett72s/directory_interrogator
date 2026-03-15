use crate::mbits;
use std::fs::File;
use std::io::{Read, Result, self};
use std::path::Path;



pub fn file_bridge(path: &str, mbits_key: Vec<mbits>) -> Result<String> {
    let fbits: String = read_bytes(path)?.iter().map(|b| format!("{:02x}", b)).collect();

    println!("{:?}", fbits);


    Ok("placeholder".to_string())
}

/*
fn detect_file_type(path: &str) -> Result<String> {

    let path = Path::new(path);
    if path.is_file() {
        let file_type = FileFormat::from_file(path)?;
        println!("File evaluated: {:?}", path);
        println!("File type: {:?}", file_type.name());
        println!("File type: {:?}", file_type.short_name());
        println!("File type: {:?}", file_type.extension());
        
    }
    Ok("placeholder".to_string())
}
    */
fn read_bytes(tgt: &str) -> io::Result<Vec<u8>> {
    println!("Evaluating file type for path: {}", tgt);

    let path = Path::new(tgt);
    let mut buff_return: Vec<u8> = Vec::new();
    if path.is_file() {
        let mut file = std::fs::File::open(&path)?;
        let mut buffer: [u8; 16] = [0u8; 16]; // enough for most signatures
        let bytes_read: usize = std::io::Read::read(&mut file, &mut buffer)?;
        // Display bytes, 
        //println!(
            //"{} -> {:x?}",
            //path.display(),
            //&buffer[..bytes_read]
        //);
        buff_return = buffer.to_vec()
    }

    Ok(buff_return) // Placeholder, should return actual magic bytes
}