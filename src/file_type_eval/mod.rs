use crate::mbits;
use std::fs::File;
use std::io::{Read, Result, self};
use std::path::Path;


// GOING TO NEED TO INCLUDE LOGIC TO REDUCE MULTIPLE MATCHES, MABYE COMPARE MATCHES TO EXTENTIONS IN DATA FILE kinda fixed with lowercase
// Coordination function with main.rs
pub fn file_bridge(path: &str, mbits_key: Vec<mbits>) -> Result<String> {
    let fbits: String = read_bytes(path)?.iter().map(|b| format!("{:02x}", b)).collect();
    println!("File bits: {:?}", fbits);

    if Path::new(path).is_file() && fbits != ""{
        for bits_type in mbits_key {
            let temp_bits       = bits_type.Signature.unwrap().clone();
            //println!("Compared bits: {:?}", temp_bits);
            
            let temp_bits_len   = temp_bits.len();
            let fbit_trunc      = &fbits[..temp_bits_len.min(fbits.len())];

            if temp_bits.to_lowercase() == fbit_trunc.to_lowercase() {

                println!("File Bits:{}, Matching Bits: {:?}", fbit_trunc, &temp_bits);
                println!("DETECTED: File: {}, Detected Type: {}, Extension: {}", path, bits_type.Name, bits_type.Extension.unwrap_or("0".into()));
            }
        }
    }
    //println!("{:?}", fbits);

    Ok(fbits.to_string())
}


fn read_bytes(tgt: &str) -> io::Result<Vec<u8>> {
    //println!("Evaluating file type for path: {}", tgt);

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

    Ok(buff_return) // Return buffer
}