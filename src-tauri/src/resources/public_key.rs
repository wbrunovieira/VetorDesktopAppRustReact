

pub fn load_public_key() -> Result<String, std::io::Error> {

  let key = include_str!("./public_key.pem");

  Ok(key.to_string())
}