// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]

mod read_dec;
use read_dec::{criar, obter_caminho_usuario};


// struct Receita {
//     User: User,
//     data: String,
//     year: u8,

// }

// struct User {
//     name: String,
//     cpf: u8,
//     grupo: String,
//     active: bool,
//     status: String,
// }




fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, obter_caminho_usuario, criar])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
        
        let caminho = obter_caminho_usuario().unwrap();
    criar(&caminho);
  
}
