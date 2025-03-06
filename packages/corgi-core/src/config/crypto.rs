use std::{fs, path::Path};

use ed25519_dalek::{
    pkcs8::{
        spki::der::pem::LineEnding, DecodePrivateKey, DecodePublicKey, EncodePrivateKey,
        EncodePublicKey,
    },
    SigningKey, VerifyingKey,
};
use rand_core::OsRng;

const ID_ED25519: &str = "id_ed25519";
const ID_ED25519_PUB: &str = "id_ed25519.pub";

#[derive(Debug, Clone)]
pub struct Keyring {
    pub privite_key: String,
    pub public_key: String,
}

impl Keyring {
    pub fn build(path: &Path) -> Result<Self, KeyringError> {
        tracing::info!("Reading keyring from path: {:?}", path);
        let id_ed25519 = path.join(ID_ED25519);
        let id_ed25519_pub = path.join(ID_ED25519_PUB);

        let privite_key = fs::read_to_string(&id_ed25519);
        let public_key = fs::read_to_string(&id_ed25519_pub);

        match (privite_key, public_key) {
            (Ok(privite_key), Ok(public_key)) => {
                tracing::debug!("Read privite key: \n{}", privite_key);
                tracing::debug!("Read public key: \n{}", public_key);

                Ok(Self {
                    privite_key,
                    public_key,
                })
            }
            _ => Self::generate_keys(path),
        }
    }

    pub fn generate_keys(path: &Path) -> Result<Self, KeyringError> {
        let (privite_key, public_key) = Self::generate_ed25519_keys()?;

        tracing::info!("Generated privite key: \n{}", privite_key);
        tracing::info!("Generated public key: \n{} ", public_key);

        tracing::info!("Writing keys to path: {:?}", path);
        fs::create_dir_all(path)?;
        fs::write(path.join(ID_ED25519), &privite_key)?;
        fs::write(path.join(ID_ED25519_PUB), &public_key)?;

        Ok(Self {
            privite_key,
            public_key,
        })
    }

    fn generate_ed25519_keys() -> Result<(String, String), ed25519_dalek::pkcs8::Error> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let privite_key = signing_key.to_pkcs8_pem(LineEnding::default())?.to_string();
        let public_key = verifying_key
            .to_public_key_pem(LineEnding::default())
            .map_err(ed25519_dalek::pkcs8::Error::PublicKey)?;

        Ok((privite_key, public_key))
    }

    pub fn signing_key(&self) -> Result<SigningKey, ed25519_dalek::pkcs8::Error> {
        SigningKey::from_pkcs8_pem(&self.privite_key)
    }

    pub fn verifying_key(&self) -> Result<VerifyingKey, ed25519_dalek::pkcs8::Error> {
        VerifyingKey::from_public_key_pem(&self.public_key)
            .map_err(ed25519_dalek::pkcs8::Error::PublicKey)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum KeyringError {
    #[error(transparent)]
    Pkcs8(#[from] ed25519_dalek::pkcs8::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
