use crate::handlers::{
    amber_store, api_key, chat, configuration, docker_file, fluentcli, job, pipeline, secure_vault,
    user, worker,
};
use crate::utils::auth::Auth;
use actix_web::{web, HttpResponse, Scope};

pub fn configure_routes() -> Scope {
    web::scope("")
        // User routes
        .service(
            web::scope("/users")
                .route("/validate-token", web::get().to(user::validate_token))
                .route("/refresh", web::post().to(user::refresh_token))
                .route("", web::post().to(user::create_user))
                .route("", web::get().to(user::list_users))
                .route("/{id}", web::get().to(user::get_user))
                .route("/{id}", web::put().to(user::update_user))
                .route("/{id}", web::delete().to(user::delete_user))
                .route("/login", web::post().to(user::login)),
        )
        // API Key routes
        .service(
            web::scope("/api_keys")
                .route("", web::post().to(api_key::create_api_key))
                .route("", web::get().to(api_key::list_api_keys))
                .route("/{id}", web::delete().to(api_key::delete_api_key)),
        )
        // Job routes
        .service(
            web::scope("/jobs")
                .wrap(Auth)
                .route("", web::post().to(job::create_job))
                .route("", web::get().to(job::list_jobs))
                .route("/{id}", web::get().to(job::get_job))
                .route("/{id}", web::put().to(job::update_job))
                .route("/{id}", web::delete().to(job::delete_job))
                .route("/{id}/data", web::get().to(job::get_job_data))
                .route("/{id}/start", web::post().to(job::start_job))
                .route("/{id}/stop", web::post().to(job::stop_job))
                .route("/{id}/status", web::get().to(job::get_job_status))
                .route("/{id}/output", web::get().to(job::get_job_output))
                .route("/{id}/logs", web::get().to(job::get_job_logs)),
        )
        // Amber Store routes
        .service(
            web::scope("/amber_stores")
                .wrap(Auth)
                .route("", web::post().to(amber_store::create_amber_store))
                .route("", web::get().to(amber_store::list_amber_stores))
                .route("/{id}", web::get().to(amber_store::get_amber_store))
                .route("/{id}", web::put().to(amber_store::update_amber_store))
                .route("/{id}", web::delete().to(amber_store::delete_amber_store)),
        )
        // Vault Store routes
        .service(
            web::scope("/secure_vaults")
                .wrap(Auth)
                .route("", web::post().to(secure_vault::create_secure_vault))
                .route("", web::get().to(secure_vault::list_secure_vaults))
                .route("/{id}", web::get().to(secure_vault::get_secure_vault))
                .route("/{id}", web::put().to(secure_vault::update_secure_vault))
                .route("/{id}", web::delete().to(secure_vault::delete_secure_vault)),
        )
        // Configuration routes
        .service(
            web::scope("/configurations")
                .wrap(Auth)
                .route("", web::post().to(configuration::create_configuration))
                .route("", web::get().to(configuration::list_configurations))
                .route("/{id}", web::get().to(configuration::get_configuration))
                .route("/{id}", web::put().to(configuration::update_configuration))
                .route(
                    "/{id}",
                    web::delete().to(configuration::delete_configuration),
                ),
        )
        // Pipeline routes
        .service(
            web::scope("/pipelines")
                .wrap(Auth)
                .route("", web::post().to(pipeline::create_pipeline))
                .route("", web::get().to(pipeline::list_pipelines))
                .route("/{id}", web::get().to(pipeline::get_pipeline))
                .route("/{id}", web::put().to(pipeline::update_pipeline))
                .route("/{id}", web::delete().to(pipeline::delete_pipeline)),
        )
        // Docker File routes
        .service(
            web::scope("/docker_files")
                .wrap(Auth)
                .route("", web::post().to(docker_file::create_docker_file))
                .route("", web::get().to(docker_file::list_docker_files))
                .route("/{id}", web::get().to(docker_file::get_docker_file))
                .route("/{id}", web::put().to(docker_file::update_docker_file))
                .route("/{id}", web::delete().to(docker_file::delete_docker_file)),
        )
        // Worker routes
        .service(
            web::scope("/workers")
                .wrap(Auth)
                .route("", web::post().to(worker::create_worker))
                .route("", web::get().to(worker::list_workers))
                .route("/{id}", web::get().to(worker::get_worker))
                .route("/{id}", web::put().to(worker::update_worker))
                .route("/{id}", web::delete().to(worker::delete_worker))
                .route("/{id}/activate", web::post().to(worker::activate_worker))
                .route(
                    "/{id}/deactivate",
                    web::post().to(worker::deactivate_worker),
                ),
        )
        .service(
            web::scope("/fluentcli")
                .wrap(Auth)
                .route("/execute", web::post().to(fluentcli::execute_command)),
        )
        // Chat routes
        .service(
            web::scope("/chat")
                .wrap(Auth)
                .route("", web::post().to(chat::send_message))
                .route("/stream", web::get().to(chat::chat_stream)),
        )
}
