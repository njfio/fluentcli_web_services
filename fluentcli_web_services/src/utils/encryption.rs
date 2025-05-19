use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex;
use rand::Rng;
use std::env;
use std::str;
use crate::error::AppError;
use bcrypt::{hash, verify, DEFAULT_COST};


type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn get_key() -> Vec<u8> {
    let key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set");
    hex::decode(key).expect("Invalid ENCRYPTION_KEY")
}

pub fn encrypt_data(data: &str) -> String {
    let key = get_key();
    let iv = rand::thread_rng().gen::<[u8; 16]>();
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
    let ciphertext = cipher.encrypt_vec(data.as_bytes());
    format!("{}:{}", hex::encode(iv), hex::encode(ciphertext))
}

pub fn decrypt_data(encrypted_data: &str) -> String {
    let key = get_key();
    let parts: Vec<&str> = encrypted_data.split(':').collect();
    let iv = hex::decode(parts[0]).unwrap();
    let ciphertext = hex::decode(parts[1]).unwrap();
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
    let decrypted_data = cipher.decrypt_vec(&ciphertext).unwrap();
    str::from_utf8(&decrypted_data).unwrap().to_string()
}

pub fn hash_secure_key(secure_key: &str) -> Result<String, AppError> {
    hash(secure_key, DEFAULT_COST).map_err(|_| AppError::InternalServerError)
}

pub fn verify_secure_key(secure_key: &str, hash: &str) -> Result<bool, AppError> {
    verify(secure_key, hash).map_err(|_| AppError::InternalServerError)
}