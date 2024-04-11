
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod resources;
mod file;
mod jwt;
mod db;
mod system;
use file::{create_file, get_path_from_user,insert_dados_dec,create_table, get_users};

use reqwest;
use chrono::Utc;

use tokio::task;

use crate::db::UserToken;
use crate::file::{read_file, read_files_dec};
use crate::resources::public_key::load_public_key;
use crate::jwt::decode_jwt;

use rusqlite::{Connection, Result, params};
use rusqlite::OptionalExtension; 
use serde_json::Value; 

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
async fn authenticate_login(email: &str, password: &str) -> Result<bool, String> {
    let public_key = load_public_key().map_err(|_| "Erro ao carregar a chave pública.")?;
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3333/auth")
        .json(&serde_json::json!({ "email": email, "password": password }))
        .send()
        .await
        .map_err(|_| "Erro ao enviar requisição")?;

    if !response.status().is_success() {
        return Err("Erro ao fazer login".into());
    }

    let response_text = response.text().await.map_err(|_| "Erro ao extrair o corpo da resposta")?;
    let v: Value = serde_json::from_str(&response_text).map_err(|_| "Erro ao parsear JSON")?;
    let access_token = v["access_token"].as_str().ok_or("Token de acesso não encontrado.")?;

    let public_key_bytes = public_key.as_bytes();
    let claims = decode_jwt(access_token, public_key_bytes).map_err(|_| "Erro ao decodificar o token .")?;

    let conn = Connection::open("dados_dec.db").map_err(|e| e.to_string())?;
    UserToken::create_user_table(&conn).map_err(|e| e.to_string())?;

    let token_data: Option<(String,)> = conn
        .query_row("SELECT expire FROM user_tokens WHERE email = ?", &[&email], |row| Ok((row.get(0)?,)))
        .optional()
        .map_err(|e| e.to_string())?;

    if let Some((expire_str,)) = token_data {
        let expire = expire_str.parse::<i64>().unwrap_or(0);
        let now = Utc::now().timestamp();
        if now < expire {
            return Ok(false);
        } else {
            conn.execute("DELETE FROM user_tokens WHERE email = ?", &[&email])
                .map_err(|e| e.to_string())?;
        }
    }

    let user_token = UserToken {
        user_id: claims.sub,
        email: email.to_string(),
        token: access_token.to_string(),
        expire: claims.exp.to_string(),
    };

    user_token.insert_to_user(&conn).map_err(|e| e.to_string())?;
    Ok(true)
}



// #[tauri::command]
// async fn authenticate_login(email: &str, password: &str) ->Result<bool, String> {

    
    
//     let public_key = load_public_key().expect("erro a carregar public key");
//     println!("public_key: {:?}", public_key);



//     println!("Nenhum token válido encontrado. Procedendo com a autenticação...");

//     println!("iniciou a authenticate");
//     let informations = format!("Received email: {} , password: {}", email, password);
//     println!("{}", informations);

//     let client = reqwest::Client::new();
//     let res = client.post("http://localhost:3333/auth")
//         .json(&serde_json::json!({ "email": email, "password": password }))
//         .send()
//         .await;

//     if let Ok(response) = res {
//         println!("entrando no res");
//         let status = response.status();
//         if status.is_success() {
//             println!("entrando no is success");
//             if let Ok(response_text) = response.text().await {
//                 println!("Token (JSON): {}", response_text);

               
//                 let v: Value = serde_json::from_str(&response_text).map_err(|_| "Erro ao parsear JSON")?;
                
//                 if let Some(access_token) = v["access_token"].as_str() {
//                     println!("O token de acesso é: {}", access_token);
//                     let public_key_bytes = public_key.as_bytes(); 
//                     println!("Tentando decodificar o JWT...");

//                     match decode_jwt(access_token, public_key_bytes) {
//                         Ok(claims) => {
//                             println!("ID do usuário: {}", claims.sub);
//                             println!("exp: {}", claims.exp);
                            
//                             let conn = match Connection::open("dados_dec.db") {
//                                 Ok(conn) => conn,
//                                 Err(e) => return Err(e.to_string()), 
//                             };
//                             UserToken::create_user_table(&conn).map_err(|e| e.to_string())?;


                            
//                             let user_token = UserToken {
//                                 user_id: claims.sub.to_string(),
//                                 email: email.to_string(),
//                                 token: access_token.to_string(),
//                                 expire:claims.exp.to_string()
//                             };
                            
//                             user_token.insert_to_user(&conn).map_err(|e| e.to_string())?;

                            
//                             return Ok(access_token.to_string());
//                         },
//                         Err(err) => {
//                             println!("Erro ao decodificar o token: {:?}", err);
//                             return Err("Erro ao decodificar o token".into());
//                         },
//                     }
                    
                    
                 
                    
//                 } else {
//                     return Err("Token de acesso não encontrado.".into());
//                 }
//             } else {
//                 return Err("Erro ao extrair o corpo da resposta".into());
//             }
//         } else {
//             return Err(format!("Erro ao fazer login: {:?}", status));
//         }
//     } else {
//         return Err("Erro ao enviar requisição".into());
//     }
    
    
// }




    
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