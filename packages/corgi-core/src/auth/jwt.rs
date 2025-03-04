use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub iat: i64,
    pub sub: Uuid,
    pub jti: Uuid,
}

impl Claims {
    pub fn encode(&self, key: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let key = EncodingKey::from_ed_pem(key.as_bytes())?;
        let header = Header::new(Algorithm::EdDSA);

        jsonwebtoken::encode(&header, self, &key)
    }

    pub fn decode(token: &str, key: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let key = DecodingKey::from_ed_pem(key.as_bytes())?;
        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.validate_aud = false;

        Ok(jsonwebtoken::decode(token, &key, &validation)?.claims)
    }
}
