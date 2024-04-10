
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod resources;
mod file;
mod jwt;
mod system;
use file::{create_file, get_path_from_user,insert_dados_dec,create_table, get_users};

use reqwest;


use crate::file::{read_file, read_files_dec};
use crate::resources::public_key::load_public_key;
use crate::jwt::decode_jwt;

use rusqlite::{Connection, Result};
use serde_json::{Value, Error}; 

use std::io::{self, Write};


#[tokio::main]
async fn main() -> Result<()> {

   
    
   

    system::print_system_info();

    #[tauri::command]
    fn test_connection() -> String {
        let message = "Conexão com o backend estabelecida!".to_string();
        println!("{}", message);
        io::stdout().flush().unwrap(); 
        message 
}
 

#[tauri::command]
async fn authenticate_login(email: &str, password: &str) -> Result<String, String> {
    // Carrega a chave pública
    let public_key = load_public_key().expect("erro a carregar public key");
    println!("public_key: {:?}", public_key);

    println!("iniciou a authenticate");
    let informations = format!("Received email: {} , password: {}", email, password);
    println!("{}", informations);

    let client = reqwest::Client::new();
    let res = client.post("http://localhost:3333/auth")
        .json(&serde_json::json!({ "email": email, "password": password }))
        .send()
        .await;

    if let Ok(response) = res {
        println!("entrando no res");
        let status = response.status();
        if status.is_success() {
            println!("entrando no is success");
            if let Ok(response_text) = response.text().await {
                println!("Token (JSON): {}", response_text);

                // Deserializa a string JSON para acessar o access_token
                let v: Value = serde_json::from_str(&response_text).map_err(|_| "Erro ao parsear JSON")?;
                
                if let Some(access_token) = v["access_token"].as_str() {
                    println!("O token de acesso é: {}", access_token);
                    let public_key_bytes = public_key.as_bytes(); 
                    println!("Tentando decodificar o JWT...");

                    match decode_jwt(access_token, public_key_bytes) {
                        Ok(claims) => {
                            println!("ID do usuário: {}", claims.sub);
                            return Ok(access_token.to_string());
                        },
                        Err(err) => {
                            println!("Erro ao decodificar o token: {:?}", err);
                            return Err("Erro ao decodificar o token".into());
                        },
                    }
                } else {
                    return Err("Token de acesso não encontrado.".into());
                }
            } else {
                return Err("Erro ao extrair o corpo da resposta".into());
            }
        } else {
            return Err(format!("Erro ao fazer login: {:?}", status));
        }
    } else {
        return Err("Erro ao enviar requisição".into());
    }
}




    
    let path_to_db = "/Users/walterbrunopradovieira/Projects/danielprojects/Vetor/ir-conferir/src-tauri/dados_dec.db";
    println!("Tentando abrir o banco de dados em: {}", path_to_db);

 
     let conn = Connection::open("dados_dec.db")?;
    create_table()?;
   
    let path = get_path_from_user().unwrap();
    create_file(&path,&"declaracao.txt");
    read_file("/Users/walterbrunopradovieira/IR-CONFERIR/declaracao.txt");
    let dados_dec = read_files_dec("/Users/walterbrunopradovieira/IR-CONFERIR");


    println!("{:#?}", dados_dec);

    for dado in &dados_dec {
        println!("{}", dado);
    }


    let dados_dec = read_files_dec("/Users/walterbrunopradovieira/IR-CONFERIR");
    for dado in dados_dec {
        insert_dados_dec(&conn, &dado)?;
    }

    let users = get_users();
    print!("{:#?}", users);

    


   
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test_connection, authenticate_login, get_users])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

        
        

        Ok(())
}