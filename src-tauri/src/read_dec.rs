// use std::fs::File;
// use std::io::Read;
// use serde_json::json;
use std::ffi::OsString;
use std::env;
use tauri::command;


// pub fn read_dec_file(file_path: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
//   // Open the file
//   let mut file = File::open(file_path)?;

//   // Read the file contents into a string
//   let mut contents = String::new();
//   file.read_to_string(&mut contents)?;

//   // Convert the contents to JSON
//   let json_value = json!({
//     "file_path": file_path,
//     "contents": contents,
//   });

//   Ok(json_value)
// }
#[command]
pub fn obter_caminho_usuario() -> Option<String> {
  let caminho = if let Some(caminho_home) = env::var_os("HOME") {
      Some(caminho_home.into_string().unwrap_or_else(|_| String::new()))
  } else {
      None
  };
  dbg!(&caminho); 
  caminho
}



#[command]
pub fn criar(caminho: &str) {
  dbg!(caminho); 
  println!("criando caminho: {}", caminho);
}