use crypto_glue::{
    argon2::{
        self,
        password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
    },
};

pub fn hash_password(password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let salt = SaltString::generate(&mut OsRng);


    let argon2 = argon2::Argon2::default();

    let hash_string = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    Ok(hash_string)
}

pub fn verify_password(password: &str, hash_password: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash_password) {
        Ok(h) => h,
        Err(_) => return false,
    };

    argon2::Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
