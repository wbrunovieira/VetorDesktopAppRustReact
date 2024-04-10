
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod file;
mod system;
use file::{create_file, get_path_from_user,insert_dados_dec,create_table, get_users};

use reqwest;


use crate::file::{read_file, read_files_dec};
use rusqlite::{Connection, Result};

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
    let informations = format!("Receiveid email: {} , password: {}", email, password);
    println!("{}", informations);
    io::stdout().flush().unwrap();
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:3333/auth")
        .json(&serde_json::json!({ "email": email, "password": password }))
        .send()
        .await;

    match res {
        Ok(response) => {
            let status = response.status();
            if response.status().is_success() {
              let body = response.text().await;
              match body {
                Ok(token)  =>{
                    println!("Token {}", token);
                    Ok(format!("resposta do login: {:?}", status))
                },
                Err(e) => Err(format!("Erro ao extrair o body {}", e))
                  
              }
            } else {
                Err(format!("Erro ao fazer login: {:?}", status))
            }
        },
        Err(e) => Err(format!("Erro ao enviar requisição: {}", e)),
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