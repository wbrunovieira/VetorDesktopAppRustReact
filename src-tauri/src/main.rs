
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod resources;
mod file;
mod jwt;
mod db;
mod system;
use file::{create_file, get_path_from_user,insert_dados_dec,create_table, get_users, get_user_by_cpf};

use system::{send_device_data, get_system_info, check_device_registered};
use reqwest;



use crate::db::UserToken;
use crate::file::{read_file, read_files_dec};
use crate::resources::public_key::load_public_key;
use crate::jwt::decode_jwt;

use rusqlite::{Connection, Result};

use serde_json::Value; 

use std::io::{self, Write};


#[tokio::main]
async fn main() -> Result<()> {


    

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
        .await;
        // .map_err(|_| "Erro ao enviar requisição")?;
        if response.is_err() {
            return Err("Erro ao enviar requisição".into());
        }
        let resp = response.unwrap();
        if !resp.status().is_success() {
            return Err("Email ou senha incorretos".into());
        }
    let response_text = resp.text().await.map_err(|_| "Erro ao extrair o corpo da resposta")?;
    let v: Value = serde_json::from_str(&response_text).map_err(|_| "Erro ao parsear JSON")?;
    let access_token = v["access_token"].as_str().ok_or("Token de acesso não encontrado.")?;

    let public_key_bytes = public_key.as_bytes();
    let claims = decode_jwt(access_token, public_key_bytes).map_err(|_| "Erro ao decodificar o token.")?;

    let conn = Connection::open("dados_dec.db").map_err(|e| e.to_string())?;
    UserToken::create_user_table(&conn).map_err(|e| e.to_string())?;

    let user_id = claims.sub.clone();

    let device_name = format!("{} - {}", sys_info::hostname().
    unwrap_or_default(), sys_info::os_type().unwrap_or_default());
    println!("Nome do dispositivo: {}", device_name);
    let device_info = get_system_info(device_name, user_id.clone());
    println!("Informações do dispositivo: {:?}", device_info);
    let is_registered = check_device_registered(&device_info.macNumber, &user_id).await?;
    println!("Dispositivo registrado: {}", is_registered);
    if is_registered {
        println!("Dispositivo já registrado. Permitindo login.");
        return Ok(true);
    }

    let device_count_url = format!("http://localhost:3333/users/{}/devices-count", user_id);
    println!("device url: {}", device_count_url);

    let device_count_response = client.get(&device_count_url).send().await.map_err(|_| "Erro ao consultar a quantidade de dispositivos")?;
    println!("device count response: {:?}", device_count_response);

    let device_count_text = device_count_response.text().await.unwrap_or_default();
    println!( "device count text: {}", device_count_text);
    let device_count_json: Value = serde_json::from_str(&device_count_text).unwrap_or_default();
    println!("device count json: {:?}", device_count_json);
    let device_count: i32 = device_count_json["devicesCount"].as_i64().unwrap_or(0) as i32;
    println!("Quantidade de dispositivos: {}", device_count);

    if device_count >= 5 {
        return Err("Número de dispositivos excedeu o limite".into());
    }

    let user_token = UserToken {
        user_id: claims.sub,
        email: email.to_string(),
        token: access_token.to_string(),
        expire: claims.exp.to_string(),
    };

    user_token.insert_to_user(&conn).map_err(|e| e.to_string())?;


 


    println!("Informações do dispositivo coletadas: {:?}", device_info);
    println!("Preparando para enviar dados do dispositivo...");

    println!("Verificando se o dispositivo já está registrado...");
    let is_registered = check_device_registered(&device_info.macNumber, &user_id).await?;
    if is_registered {
        println!("Dispositivo já registrado. Não será criado novamente.");
        return Ok(true); 
    } else {
        println!("Dispositivo não registrado. Criando novo registro...");
        let send_result = send_device_data(device_info).await;
        if let Err(e) = send_result {
            println!("Erro ao enviar dados do dispositivo: {}", e);
        } else {
            println!("Dados do dispositivo enviados com sucesso.");
        }
    }
 

    
    Ok(true)
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
        .invoke_handler(tauri::generate_handler![test_connection, authenticate_login, get_users, get_user_by_cpf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

        
        

        Ok(())
}