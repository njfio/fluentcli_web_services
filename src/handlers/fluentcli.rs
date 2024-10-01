use crate::models::fluentcli::CommandRequest;
use crate::services::fluentcli_service::FluentCLIService;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use log::{debug, error, info};
use uuid::Uuid;

pub async fn execute_command(
    command_request: web::Json<CommandRequest>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    info!("Received execute_command request for user_id: {}", user_id);
    debug!("Command request: {:?}", command_request);

    match FluentCLIService::execute_command(user_id, command_request.into_inner()).await {
        Ok(result) => {
            info!("Command executed successfully");
            debug!("Command result: {:?}", result);
            HttpResponse::Ok().json(result)
        }
        Err(e) => {
            error!("Error executing FluentCLI command: {:?}", e);
            HttpResponse::InternalServerError()
                .body(format!("Failed to execute FluentCLI command: {:?}", e))
        }
    }
}
