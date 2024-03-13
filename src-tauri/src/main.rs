
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file;
use file::{create_file, get_path_from_user};

use crate::file::{read_file, read_files_dec};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    println!("Hello from Rust!");
    let path = get_path_from_user().unwrap();
    create_file(&path,&"declaracao.txt");
    read_file("/Users/walterbrunopradovieira/IR-CONFERIR/declaracao.txt");
    read_files_dec("/Users/walterbrunopradovieira/IR-CONFERIR");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}