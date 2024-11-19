use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash(password: impl AsRef<str>) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_ref().as_bytes(), &salt)
        .map(|hash| hash.to_string())
}

pub fn verify(
    password: impl AsRef<str>,
    hash: impl AsRef<str>,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash.as_ref())?;

    let argon2 = Argon2::default();

    argon2.verify_password(password.as_ref().as_bytes(), &parsed_hash)
}
