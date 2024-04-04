// No arquivo system.rs
use sys_info;
use mac_address::get_mac_address;

pub fn print_system_info() {
    match sys_info::os_type() {
        Ok(os_type) => println!("Tipo de SO: {}", os_type),
        Err(e) => println!("Não foi possível obter o tipo de SO: {}", e),
    }

    match sys_info::os_release() {
        Ok(os_release) => println!("Release de SO: {}", os_release),
        Err(e) => println!("Não foi possível obter o release de SO: {}", e),
    }

    match sys_info::hostname() {
        Ok(hostname) => println!("Hostname: {}", hostname),
        Err(e) => println!("Não foi possível obter o hostname: {}", e),
    }

    match get_mac_address() {
        Ok(Some(mac_address)) => println!("Endereço MAC encontrado: {:?}", mac_address),
        Ok(None) => println!("Nenhum endereço MAC disponível."),
        Err(e) => println!("Erro ao obter o endereço MAC: {}", e),
    }
}
