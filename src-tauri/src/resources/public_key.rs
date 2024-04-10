

pub fn load_public_key() -> Result<String, std::io::Error> {
  // Usando `include_str!` como um exemplo, se o arquivo não for grande e puder ser incluído em tempo de compilação
  // Isso só é viável para arquivos que não mudarão entre o desenvolvimento e a produção
  let key = include_str!("./public_key.pem");

  Ok(key.to_string())
}