use serde::{Deserialize, Serialize};
// No arquivo system.rs
use sys_info;
use mac_address::get_mac_address;



#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceInfo {
    pub name: String,
    pub userId: String,  
    pub os: String,      
    pub version: String, 
    pub hostname: String,
    pub macNumber: String, 
}



#[tauri::command]
pub async fn send_device_data(device_info: DeviceInfo) -> Result<(), String> {
    let client = reqwest::Client::new();
    println!("Enviando dados do dispositivo: {:?}", device_info);  // Logar a informação que está sendo enviada

    let response = client
        .post("http://localhost:3333/devices")
        .json(&device_info)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("Dispositivo registrado com sucesso.");
                Ok(())
            } else {
                let status = resp.status();
                let error_text = resp.text().await.unwrap_or_default();
                println!("Falha ao registrar dispositivo: Status {}, Erro: {}", status, error_text);
                Err(format!("Falha ao registrar dispositivo: Status {}, Erro: {}", status, error_text))
            }
        }
        Err(e) => {
            println!("Erro ao conectar com o servidor: {}", e);
            Err(format!("Erro ao conectar com o servidor: {}", e))
        }
    }
}



pub fn get_system_info(name: String, user_id: String) -> DeviceInfo {
    let os_type = sys_info::os_type().unwrap_or_else(|_| "Desconhecido".to_string());
    let os_release = sys_info::os_release().unwrap_or_else(|_| "Desconhecido".to_string());
    let hostname = sys_info::hostname().unwrap_or_else(|_| "Desconhecido".to_string());
    let mac_address = get_mac_address().map(|mac| mac.map_or("Nenhum".to_string(), |m| m.to_string())).unwrap_or_else(|_| "Erro".to_string());

    DeviceInfo {
        name,
        userId: user_id,
        os: os_type,
        version: os_release, 
        hostname,
        macNumber: mac_address,
    }
}



