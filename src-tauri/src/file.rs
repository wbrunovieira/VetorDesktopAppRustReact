use std::{env, fs::{self, File}, path::Path};

pub fn get_path_from_user( ) -> Option<String> {
  if let Some(path_home) = env::var_os("HOME") {
    let path = Path::new(&path_home).join("IR-CONFERIR");
    if !path.exists() {
     
      match fs::create_dir_all(&path) {
          Ok(_) => println!("Diretório criado: {:?}", path),
          Err(e) => println!("Erro ao criar diretório: {}", e),
      }
  }
    Some(path.to_str().unwrap().to_string())
  } else {
    None
  }
}

pub fn create_file(path: &str, file_name: &str) {
  println!("Creating file at: {}", path);
  println!("File Name: {}", file_name);

  let full_path = Path::new(path).join(file_name);
  let full_path_str = full_path.to_str().unwrap_or("Invalid path");

 match File::create(&full_path) {
    Ok(_) => { 
      println!("File created successfully at {}", full_path_str)
    },
    Err(e) => {
      println!("Error creating file at {}: {}", full_path_str, e)
      }
}
}