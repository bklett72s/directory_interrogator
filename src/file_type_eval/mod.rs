use crate::mbits;
use std::fs::File;
use std::io::{Read, Result, self};
use std::path::Path;


// GOING TO NEED TO INCLUDE LOGIC TO REDUCE MULTIPLE MATCHES, MABYE COMPARE MATCHES TO EXTENTIONS IN DATA FILE kinda fixed with lowercase
// Coordination function with main.rs
pub fn file_bridge(path: &str, mbits_key: Vec<mbits>) -> Result<String> {
    let fbits: String = read_bytes(path)?.iter().map(|b| format!("{:02x}", b)).collect();

    let determination: String = match_probability(fbits, path, mbits_key)?;

    //println!("{:?}", fbits);

    Ok("test holder".to_string())
}


// Function to read the first 16 bytes of a file and return them as a vector of bytes
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

// Function to determine potential matches
fn match_probability(fbits: String, path: &str, mbits_key: Vec<mbits>) -> io::Result<String> {
    let mut pot_matches: Vec<String> = Vec::new();
    let mut match_counter: usize = 0;
    let mut determination: String = "Unknown, Potential Text File".to_string();
    let mut confidence: f32 = 0.0;

    if Path::new(path).is_file() && fbits != ""{
        for bits_type in mbits_key {
            let temp_bits: String       = bits_type.Signature.unwrap().clone();
            let temp_bits_len: usize    = temp_bits.len();
            let fbit_trunc  = &fbits[..temp_bits_len.min(fbits.len())];

            if temp_bits.to_lowercase() == fbit_trunc.to_lowercase() {
                let file_match_name: String = bits_type.Name;
                let file_match_ext: String = bits_type.Extension.unwrap_or("0".into());
                confidence = (fbit_trunc.len() as f32 / fbits.len() as f32) * 100.0;

                pot_matches.push(format!("{}, {}, {:.2}%", file_match_name, file_match_ext, confidence));
            }
        }
        for match_str in &pot_matches {
            let match_counter_temp: usize = count_matches(match_str.to_string(), pot_matches.clone())?;

            if match_counter_temp > match_counter {
                match_counter = match_counter_temp;
                determination = match_str.to_string();
            }
            else if match_counter_temp == match_counter && match_counter != 0 {
                determination = format!("{} // {}", determination, match_str);

            }
            else if match_counter == 0 {
                determination = match_str.to_string();
            }
            
        }
        println!("File: {}", path);
        println!("Determination: {}", determination);
    }

    Ok("Test".to_string())
}

fn count_matches(search_string: String, pot_matches: Vec<String>) -> Result<usize> {
    let count = pot_matches
        .iter()
        .filter(|&x| *x == search_string)
        .count();
    Ok(count)
}