
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate sys_info;
extern crate mac_address;


use mac_address::get_mac_address;
mod file;
use file::{create_file, get_path_from_user,insert_dados_dec,create_table, get_users};



use crate::file::{read_file, read_files_dec};
use rusqlite::{Connection, Result};

use std::io::{self, Write};



fn main() -> Result<()> {

    #[tauri::command]
    fn test_connection() -> String {
        let message = "Conexão com o backend estabelecida!".to_string();
        println!("{}", message);
        io::stdout().flush().unwrap(); // Força o flush do que foi impresso
        message 
}
   

    #[tauri::command]
     fn authenticate_login(email: &str, password: &str) -> String {
      let information =  format!("Received email: {}, password: {}", email, password);
      io::stdout().flush().unwrap();
        information


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

    match sys_info::os_type() {
        Ok(os_type) => println!("Tipo de SO: {}", os_type),
        Err(e) => println!("Não foi possível obter o tipo de SO: {}", e),
    }
    match sys_info::os_release() {
        Ok(os_release) => println!("release de SO: {}", os_release),
        Err(e) => println!("Não foi possível obter o releaseo de SO: {}", e),
    }
    match sys_info::hostname() {
        Ok(hostname) => println!("hostname: {}", hostname),
        Err(e) => println!("Não foi possível hostname: {}", e),
    }

   

    match get_mac_address() {
        Ok(Some(mac_address)) => println!("Endereço MAC encontrado: {:?}", mac_address),
  
       Ok(None) => println!("Nenhum endereço MAC disponível."),
        Err(  e) => println!("Erro ao obter o endereço MAC: {}" ,e),
    }

    #[tauri::command]
    fn simple_test() {
    println!("Teste simples executado");
    simple_test();
}

   
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test_connection, authenticate_login, get_users])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

 
        
        

        Ok(())
}