use std::fs::File;
use std::io::Read;
use serde_json::json;

pub fn read_dec_file(file_path: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
  // Open the file
  let mut file = File::open(file_path)?;

  // Read the file contents into a string
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  // Convert the contents to JSON
  let json_value = json!({
    "file_path": file_path,
    "contents": contents,
  });

  Ok(json_value)
}