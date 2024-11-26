use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum Audience {
    User,
    ApiKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: Audience,
    pub exp: i64,
    pub iat: i64,
    pub iss: Uuid,
    pub sub: String,
}

impl Claims {
    pub fn new(audience: Audience, issuer: Uuid, subject: String, valid_days: i32) -> Self {
        let now = Utc::now();
        let exp = now + Duration::days(valid_days.into());

        Self {
            aud: audience,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            sub: subject,
            iss: issuer,
        }
    }
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
