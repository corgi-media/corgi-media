use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    exp: i64,
    iat: i64,
    sub: String,
}

impl Claims {
    pub fn new(audience: String, subject: String, valid_days: i32) -> Self {
        let now = chrono::Utc::now();

        Self {
            aud: audience,
            exp: (now + chrono::Duration::days(valid_days.into())).timestamp(),
            iat: now.timestamp(),
            sub: subject,
        }
    }
}

impl Claims {
    pub fn encode(&self, key: &EncodingKey) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(&Header::new(Algorithm::EdDSA), self, key)
    }

    pub fn decode(token: &str, key: &DecodingKey) -> Result<Self, jsonwebtoken::errors::Error> {
        Ok(jsonwebtoken::decode::<Claims>(token, key, &Validation::new(Algorithm::EdDSA))?.claims)
    }
}
