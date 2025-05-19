use crate::db::db::establish_connection;
use crate::error::AppError;
use crate::models::attachment::Attachment;
use crate::models::conversation::Conversation;
use crate::models::llm_provider::LLMProvider;
use crate::models::message::Message;
use crate::models::user_llm_config::UserLLMConfig;
use crate::services::chat_service::ChatService;
use uuid::Uuid;

fn setup() -> DbPool {
    dotenv::dotenv().ok();
    establish_connection()
}

#[test]
fn test_create_conversation() {
    let pool = setup();
    let user_id = Uuid::new_v4();
    let title = "Test Conversation".to_string();

    let result = ChatService::create_conversation(&pool, user_id, title.clone());
    assert!(result.is_ok());

    let conversation = result.unwrap();
    assert_eq!(conversation.user_id, user_id);
    assert_eq!(conversation.title, title);
}

#[test]
fn test_get_conversation() {
    let pool = setup();
    let user_id = Uuid::new_v4();
    let title = "Test Conversation".to_string();

    let created_conversation =
        ChatService::create_conversation(&pool, user_id, title.clone()).unwrap();
    let result = ChatService::get_conversation(&pool, created_conversation.id);

    assert!(result.is_ok());
    let fetched_conversation = result.unwrap();
    assert_eq!(fetched_conversation.id, created_conversation.id);
    assert_eq!(fetched_conversation.user_id, user_id);
    assert_eq!(fetched_conversation.title, title);
}

#[test]
fn test_create_message() {
    let pool = setup();
    let user_id = Uuid::new_v4();
    let conversation =
        ChatService::create_conversation(&pool, user_id, "Test Conversation".to_string()).unwrap();

    let role = "user".to_string();
    let content = "Hello, AI!".to_string();

    let result = ChatService::create_message(&pool, conversation.id, role.clone(), content.clone());
    assert!(result.is_ok());

    let message = result.unwrap();
    assert_eq!(message.conversation_id, conversation.id);
    assert_eq!(message.role, role);
    assert_eq!(message.content, content);
}

#[test]
fn test_get_message() {
    let pool = setup();
    let user_id = Uuid::new_v4();
    let conversation =
        ChatService::create_conversation(&pool, user_id, "Test Conversation".to_string()).unwrap();
    let created_message = ChatService::create_message(
        &pool,
        conversation.id,
        "user".to_string(),
        "Hello, AI!".to_string(),
    )
    .unwrap();

    let result = ChatService::get_message(&pool, created_message.id);
    assert!(result.is_ok());

    let fetched_message = result.unwrap();
    assert_eq!(fetched_message.id, created_message.id);
    assert_eq!(fetched_message.conversation_id, conversation.id);
    assert_eq!(fetched_message.role, "user");
    assert_eq!(fetched_message.content, "Hello, AI!");
}

#[test]
fn test_get_messages() {
    let pool = setup();
    let user_id = Uuid::new_v4();
    let conversation =
        ChatService::create_conversation(&pool, user_id, "Test Conversation".to_string()).unwrap();

    ChatService::create_message(
        &pool,
        conversation.id,
        "user".to_string(),
        "Hello, AI!".to_string(),
    )
    .unwrap();
    ChatService::create_message(
        &pool,
        conversation.id,
        "ai".to_string(),
        "Hello, human!".to_string(),
    )
    .unwrap();

    let result = ChatService::get_messages(&pool, conversation.id);
    assert!(result.is_ok());

    let messages = result.unwrap();
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0].conversation_id, conversation.id);
    assert_eq!(messages[1].conversation_id, conversation.id);
}

#[test]
fn test_create_attachment() {
    let pool = setup();
    let user_id = Uuid::new_v4();
    let conversation =
        ChatService::create_conversation(&pool, user_id, "Test Conversation".to_string()).unwrap();
    let message = ChatService::create_message(
        &pool,
        conversation.id,
        "user".to_string(),
        "Hello with attachment".to_string(),
    )
    .unwrap();

    let file_type = "image/png".to_string();
    let file_path = "/path/to/image.png".to_string();

    let result =
        ChatService::create_attachment(&pool, message.id, file_type.clone(), file_path.clone());
    assert!(result.is_ok());

    let attachment = result.unwrap();
    assert_eq!(attachment.message_id, message.id);
    assert_eq!(attachment.file_type, file_type);
    assert_eq!(attachment.file_path, file_path);
}

#[test]
fn test_get_attachments() {
    let pool = setup();
    let user_id = Uuid::new_v4();
    let conversation =
        ChatService::create_conversation(&pool, user_id, "Test Conversation".to_string()).unwrap();
    let message = ChatService::create_message(
        &pool,
        conversation.id,
        "user".to_string(),
        "Hello with attachments".to_string(),
    )
    .unwrap();

    ChatService::create_attachment(
        &pool,
        message.id,
        "image/png".to_string(),
        "/path/to/image1.png".to_string(),
    )
    .unwrap();
    ChatService::create_attachment(
        &pool,
        message.id,
        "image/jpg".to_string(),
        "/path/to/image2.jpg".to_string(),
    )
    .unwrap();

    let result = ChatService::get_attachments(&pool, message.id);
    assert!(result.is_ok());

    let attachments = result.unwrap();
    assert_eq!(attachments.len(), 2);
    assert_eq!(attachments[0].message_id, message.id);
    assert_eq!(attachments[1].message_id, message.id);
}

#[test]
fn test_create_llm_provider() {
    let pool = setup();
    let name = "Test Provider".to_string();
    let api_endpoint = "https://api.test-provider.com".to_string();

    let result = ChatService::create_llm_provider(&pool, name.clone(), api_endpoint.clone());
    assert!(result.is_ok());

    let provider = result.unwrap();
    assert_eq!(provider.name, name);
    assert_eq!(provider.api_endpoint, api_endpoint);
}

#[test]
fn test_get_llm_provider() {
    let pool = setup();
    let name = "Test Provider".to_string();
    let api_endpoint = "https://api.test-provider.com".to_string();

    let created_provider =
        ChatService::create_llm_provider(&pool, name.clone(), api_endpoint.clone()).unwrap();
    let result = ChatService::get_llm_provider(&pool, created_provider.id);

    assert!(result.is_ok());
    let fetched_provider = result.unwrap();
    assert_eq!(fetched_provider.id, created_provider.id);
    assert_eq!(fetched_provider.name, name);
    assert_eq!(fetched_provider.api_endpoint, api_endpoint);
}

#[test]
fn test_create_user_llm_config() {
    let pool = setup();
    let user_id = Uuid::new_v4();
    let provider = ChatService::create_llm_provider(
        &pool,
        "Test Provider".to_string(),
        "https://api.test-provider.com".to_string(),
    )
    .unwrap();
    let api_key = "test-api-key".to_string();

    let result = ChatService::create_user_llm_config(&pool, user_id, provider.id, api_key.clone());
    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(config.user_id, user_id);
    assert_eq!(config.provider_id, provider.id);
    assert_eq!(config.api_key, api_key);
}

#[test]
fn test_get_user_llm_config() {
    let pool = setup();
    let user_id = Uuid::new_v4();
    let provider = ChatService::create_llm_provider(
        &pool,
        "Test Provider".to_string(),
        "https://api.test-provider.com".to_string(),
    )
    .unwrap();
    let api_key = "test-api-key".to_string();

    let created_config =
        ChatService::create_user_llm_config(&pool, user_id, provider.id, api_key.clone()).unwrap();
    let result = ChatService::get_user_llm_config(&pool, user_id, provider.id);

    assert!(result.is_ok());
    let fetched_config = result.unwrap();
    assert_eq!(fetched_config.id, created_config.id);
    assert_eq!(fetched_config.user_id, user_id);
    assert_eq!(fetched_config.provider_id, provider.id);
    assert_eq!(fetched_config.api_key, api_key);
}

// Add more tests for error cases
#[test]
fn test_get_nonexistent_conversation() {
    let pool = setup();
    let nonexistent_id = Uuid::new_v4();

    let result = ChatService::get_conversation(&pool, nonexistent_id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AppError::DatabaseError(_)));
}

// Add more error case tests for other functions
