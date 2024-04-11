use std::{env, fs::{self, File}, io::Read, path::Path};
use std::io::Write;
use std::fmt;
use rusqlite::{Connection, Result};


#[derive(Debug)]
pub struct DadosDec {
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserInfo {
    nome: String,
    cpf: String,
}


impl fmt::Display for DadosDec {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "CPF: {}, Nome: {}, Exercício: {}, Rendimentos Tributáveis: {}, Rendimentos Isentos: {}, Rendimentos Sujeitos à Tributação Exclusiva: {}, Pagamento Anual Total de Juros: {}, Doações a Partidos Políticos: {}, Pagamentos/Doações/Outros: {}",
          self.cpf, self.nome, self.exercicio, self.rend_tributaveis, self.rend_isentos, self.rend_exclusivos, self.juros, self.doacoes_politicas, self.pagamentos_doacoes_outros)
  }
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



pub fn read_files_dec(diretorio: &str) -> Vec<DadosDec> {
  let path = Path::new(diretorio);
  let mut dados_dec = Vec::new();

  if path.exists() && path.is_dir() {
      match fs::read_dir(path) {
          Ok(entries) => {
              for entry in entries.filter_map(Result::ok) {
                  let path = entry.path();
                  if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("DEC") {
                      match fs::read_to_string(&path) {
                          Ok(content) => {
                            if content.len() > 775 {
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
                              dados_dec.push(dados);
                            } else {
                              println!("Arquivo {:?} não possui dados suficientes.", path.file_name().unwrap());
                            }
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

  dados_dec
}

pub fn create_table() -> Result<()> {
  let conn = Connection::open("dados_dec.db")?;

  conn.execute(
      "CREATE TABLE IF NOT EXISTS dados_dec (
          id INTEGER PRIMARY KEY,
          cpf TEXT NOT NULL,
          nome TEXT NOT NULL,
          exercicio TEXT NOT NULL,
          rend_tributaveis TEXT NOT NULL,
          rend_isentos TEXT NOT NULL,
          rend_exclusivos TEXT NOT NULL,
          juros TEXT NOT NULL,
          doacoes_politicas TEXT NOT NULL,
          pagamentos_doacoes_outros TEXT NOT NULL
       )",
      [],
  )?;
  Ok(())
}


pub fn insert_dados_dec(conn: &Connection, dados: &DadosDec) -> Result<()> {
  let mut check_stmt = conn.prepare("SELECT cpf FROM dados_dec WHERE cpf = ?1")?;
  let exists = check_stmt.exists(&[&dados.cpf])?;

  if !exists {
  
  conn.execute(
      "INSERT INTO dados_dec (cpf, nome, exercicio, rend_tributaveis, rend_isentos, rend_exclusivos, juros, doacoes_politicas, pagamentos_doacoes_outros)
       VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
      &[
          &dados.cpf, &dados.nome, &dados.exercicio, &dados.rend_tributaveis,
          &dados.rend_isentos, &dados.rend_exclusivos, &dados.juros,
          &dados.doacoes_politicas, &dados.pagamentos_doacoes_outros,
      ],
  )?;
}
  Ok(())
}
#[tauri::command]
pub fn get_users() -> Result<Vec<UserInfo>, String> {
  let path_to_db = "/Users/walterbrunopradovieira/Projects/danielprojects/Vetor/ir-conferir/src-tauri/dados_dec.db";
    let conn = Connection::open(path_to_db)
        .map_err(|e| e.to_string())?;
    let mut statement = conn
        .prepare("SELECT nome, cpf FROM dados_dec")
        .map_err(|e| e.to_string())?;
    let users_iter = statement
        .query_map([], |row| {
            Ok(UserInfo {
                nome: row.get(0)?,
                cpf: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let users: Result<Vec<_>, _> = users_iter.collect();
    users.map_err(|e| e.to_string())
}

