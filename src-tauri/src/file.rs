use std::env;

pub fn get_path_from_user( ) -> Option<String> {
  if let Some(path_home) = env::var_os("HOME") {
    Some(path_home.into_string().unwrap())
  } else {
    None
  }
}

pub fn create_file(path: &str){
  println!("Creating file at: {}", path)
}