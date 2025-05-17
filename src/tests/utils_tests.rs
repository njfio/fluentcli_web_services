use crate::utils::auth::{hash_password, verify_password};
use crate::utils::encryption::{encrypt_data, decrypt_data, hash_secure_key, verify_secure_key};
use crate::utils::jwt::{generate_token, validate_token, get_token_version, increment_token_version};
use uuid::Uuid;
use std::env;

#[test]
fn test_password_hashing_roundtrip() {
    let password = "test_password";
    let hash = hash_password(password).expect("hashing failed");
    assert!(verify_password(password, &hash).expect("verification failed"));
}

#[test]
fn test_encryption_roundtrip() {
    env::set_var("ENCRYPTION_KEY", "00000000000000000000000000000000ffffffffffffffffffffffffffffffff");
    let data = "secret";
    let encrypted = encrypt_data(data);
    let decrypted = decrypt_data(&encrypted);
    assert_eq!(decrypted, data);
}

#[test]
fn test_secure_key_hashing() {
    let key = "mykey";
    let hash = hash_secure_key(key).expect("hashing failed");
    assert!(verify_secure_key(key, &hash).expect("verification failed"));
}

#[test]
fn test_jwt_token_generation_and_validation() {
    env::set_var("JWT_SECRET", "testsecret");
    let user_id = Uuid::new_v4();
    let token = generate_token(user_id).expect("token generation failed");
    let (decoded_id, version) = validate_token(&token).expect("validation failed");
    assert_eq!(decoded_id, user_id);
    assert_eq!(version, get_token_version(&user_id));
    let new_version = increment_token_version(&user_id);
    assert_eq!(new_version, version + 1);
}
