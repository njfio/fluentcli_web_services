use fluentcli_web_services::utils::jwt::{generate_token, validate_token, increment_token_version};
use uuid::Uuid;

#[test]
fn token_generation_and_validation() {
    std::env::set_var("JWT_SECRET", "testsecret");
    let user_id = Uuid::new_v4();
    let token = generate_token(user_id).expect("failed to generate token");
    let (validated_id, _version) = validate_token(&token).expect("failed to validate token");
    assert_eq!(user_id, validated_id);

    // token should fail after version increment
    increment_token_version(&user_id);
    assert!(validate_token(&token).is_err());
}
