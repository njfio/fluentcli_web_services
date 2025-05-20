use crate::db::DbPool;
use crate::error::AppError;
use crate::models::llm_template::{CreateUnifiedLLMConfigRequest, UnifiedLLMConfig};
use crate::services::llm_template_service::LLMTemplateService;
use actix_web::{web, HttpResponse, Responder};
use log::{debug, error, info};
use uuid::Uuid;

/// Get all available LLM templates
pub async fn get_templates() -> Result<impl Responder, AppError> {
    info!("Getting all LLM templates");
    let templates = LLMTemplateService::get_templates();
    Ok(HttpResponse::Ok().json(templates))
}

/// Get a specific LLM template by ID
pub async fn get_template(path: web::Path<String>) -> Result<impl Responder, AppError> {
    let template_id = path.into_inner();
    info!("Getting LLM template with ID: {}", template_id);
    
    match LLMTemplateService::get_template_by_id(&template_id) {
        Some(template) => Ok(HttpResponse::Ok().json(template)),
        None => Err(AppError::NotFoundError(format!("Template with ID {} not found", template_id))),
    }
}

/// Create a new unified LLM configuration
pub async fn create_unified_config(
    pool: web::Data<DbPool>,
    req: web::Json<CreateUnifiedLLMConfigRequest>,
    user_id: web::ReqData<Uuid>,
) -> Result<impl Responder, AppError> {
    let user_id = *user_id;
    info!("Creating unified LLM configuration for user: {}", user_id);
    
    let config = UnifiedLLMConfig {
        id: None,
        user_id,
        name: req.name.clone(),
        provider_type: req.provider_type.clone(),
        api_endpoint: req.api_endpoint.clone(),
        supported_modalities: req.supported_modalities.clone(),
        configuration: req.configuration.clone(),
        api_key: req.api_key.clone(),
        api_key_description: req.api_key_description.clone(),
        created_at: None,
        updated_at: None,
    };
    
    match LLMTemplateService::create_unified_config(&pool, user_id, config) {
        Ok(response) => {
            debug!("Created unified LLM configuration with ID: {}", response.id);
            Ok(HttpResponse::Created().json(response))
        },
        Err(e) => {
            error!("Failed to create unified LLM configuration: {:?}", e);
            Err(e)
        }
    }
}

/// Get all unified LLM configurations for a user
pub async fn get_unified_configs(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
) -> Result<impl Responder, AppError> {
    let user_id = *user_id;
    info!("Getting unified LLM configurations for user: {}", user_id);
    
    match LLMTemplateService::get_unified_configs(&pool, user_id) {
        Ok(configs) => {
            debug!("Found {} unified LLM configurations", configs.len());
            Ok(HttpResponse::Ok().json(configs))
        },
        Err(e) => {
            error!("Failed to get unified LLM configurations: {:?}", e);
            Err(e)
        }
    }
}

/// Delete a unified LLM configuration
pub async fn delete_unified_config(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let user_id = *user_id;
    let config_id = path.into_inner();
    info!("Deleting unified LLM configuration: {} for user: {}", config_id, user_id);
    
    match LLMTemplateService::delete_unified_config(&pool, user_id, config_id) {
        Ok(_) => {
            debug!("Deleted unified LLM configuration: {}", config_id);
            Ok(HttpResponse::NoContent().finish())
        },
        Err(e) => {
            error!("Failed to delete unified LLM configuration: {:?}", e);
            Err(e)
        }
    }
}
