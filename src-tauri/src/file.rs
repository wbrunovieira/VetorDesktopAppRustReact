use std::{env, fs::{self, File}, io::Read, path::Path};
use std::io::Write;

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
    Ok(mut file) => { 
      file.write_all(b"Hello, world!").unwrap();
      println!("File created successfully at {}", full_path_str)
    },
    Err(e) => {
      println!("Error creating file at {}: {}", full_path_str, e)
      }
}
}

pub fn read_file(full_path: &str) {
  println!("Reading file at: {}", full_path);


  match File::open(&full_path) {
    Ok(mut file) => { 
      
      let mut content = String::new();
      file.read_to_string(&mut content).unwrap();
      println!("File opened successfully at {}", content)
    },
    Err(e) => {
      println!("Error opening file at {}:",  e)
      }
}
}

pub fn read_files_dec(diretorio: &str) {
  let path = Path::new(diretorio);

  if path.exists() && path.is_dir() {
      match fs::read_dir(path) {
          Ok(entries) => {
              for entry in entries.filter_map(Result::ok) {
                  let path = entry.path();
                  if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("DEC") {
                      match fs::read_to_string(&path) {
                          Ok(content) => {
                              println!("Conteúdo do arquivo {:?}:\n{}", path.file_name().unwrap(), content);
                          }
                          Err(e) => {
                              println!("Erro ao ler o arquivo {:?}: {}", path.file_name().unwrap(), e);
                          }
                      }
                  }
              }
          },
          Err(e) => println!("Erro ao ler o diretório: {}", e),
      }
  } else {
      println!("O diretório fornecido não existe ou não é um diretório.");
  }
}