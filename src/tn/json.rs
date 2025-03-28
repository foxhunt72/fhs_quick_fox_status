use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json::{self, Error as JsonError};

// A generic function to save any serializable struct to a JSON file
pub fn save_to_json<T: Serialize>(data: &T, path: &Path) -> Result<(), std::io::Error> {
    // Convert the data to a JSON string with pretty formatting
    let json_string = serde_json::to_string_pretty(data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    // Create or open the file with write permissions
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    
    // Write the JSON string to the file
    file.write_all(json_string.as_bytes())?;
    file.flush()?;
    
    Ok(())
}

// A generic function to load a JSON file into a deserializable struct
pub fn load_from_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, std::io::Error> {
    // Open the file in read-only mode
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    // Parse the JSON into the provided type
    let data: T = serde_json::from_reader(reader)
        .map_err(|e: JsonError| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    Ok(data)
}
