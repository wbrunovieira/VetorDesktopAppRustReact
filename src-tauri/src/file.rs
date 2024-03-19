use std::{env, fs::{self, File}, io::Read, path::Path};
use std::io::Write;

#[derive(Debug)]
struct DadosDec {
    cpf: String,
    nome: String,
    exercicio: String,
    rend_tributaveis: String,
    rend_isentos: String,
    rend_exclusivos: String,
    juros: String,
    doacoes_politicas: String,
    pagamentos_doacoes_outros: String,
}


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
                              let dados = DadosDec {
                                  cpf: content[21..32].to_string(),
                                  nome: content[39..99].trim().to_string(),
                                  exercicio: content[8..12].to_string(),
                                  rend_tributaveis: content[695..708].to_string(),
                                  rend_isentos: content[736..749].to_string(),
                                  rend_exclusivos: content[749..762].to_string(),
                                  juros: content[193..206].to_string(),
                                  doacoes_politicas: content[482..495].to_string(),
                                  pagamentos_doacoes_outros: content[762..775].to_string(),
                              };
                              
                              // Aqui, você pode fazer o que precisar com os dados, como imprimir ou armazenar em algum lugar
                              println!("{:?}", dados);
                          },
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
