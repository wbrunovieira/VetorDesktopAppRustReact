use jsonwebtoken::{decode, DecodingKey, Validation, errors::Error};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: String, 
}

pub fn decode_jwt(token: &str, public_key: &[u8]) -> Result<Claims, Error> {
    let token_decoded = decode::<Claims>(
        token,
        &DecodingKey::from_rsa_pem(public_key)?, 
        &Validation::new(jsonwebtoken::Algorithm::RS256), 
    )?;
    Ok(token_decoded.claims)
}


