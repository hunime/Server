use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub fn set() -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        sub: "JWT Authenticator".to_string(),
        exp: 100,
    };
    let token = encode(
        &Header::new(Algorithm::HS384),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )?;

    Ok(token)
}

pub fn get(token: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS384),
    )?;
    Ok(claims)
}
