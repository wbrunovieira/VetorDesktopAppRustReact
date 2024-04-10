use std::fmt;

use rusqlite::{params,Connection, Result};

pub struct DbConn {
    pub conn: Connection,
}

// user logica
#[derive(Debug)]
pub struct UserToken {
    pub user_id: String,
    pub email: String,
    pub token: String,
    pub expire: String,
}

 impl UserToken {
   
    pub fn create_user_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS user_tokens (
                id INTEGER PRIMARY KEY,
                user_id TEXT NOT NULL,
                email TEXT NOT NULL,
                token TEXT NOT NULL,
                expire TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

   
    pub fn insert_to_user(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO user_tokens (user_id, email, token, expire) VALUES (?1, ?2, ?3, ?4)",
            params![self.user_id, self.email, self.token, self.expire.to_string()],
        )?;
        Ok(())
    }
}



// data logica

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

impl DadosDec {
  // Getter para o campo `nome`
  pub fn nome(&self) -> &str {
      &self.nome
  }

  // Setter para o campo `nome`
  pub fn set_nome(&mut self, nome: String) {
     
      self.nome = nome;
  }
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


impl DbConn {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(DbConn { conn })
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

  
}
