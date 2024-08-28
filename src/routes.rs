use actix_web::{web, HttpResponse, Scope};
use crate::handlers::{user, job, api_key, amber_store, vault_store, configuration, pipeline, docker_file, worker};

pub fn configure_routes() -> Scope {
    web::scope("")
        // User routes
        .service(
            web::scope("/users")
                .route("", web::post().to(user::create_user))
                .route("", web::get().to(user::list_users))
                .route("/{id}", web::get().to(user::get_user))
                .route("/{id}", web::put().to(user::update_user))
                .route("/{id}", web::delete().to(user::delete_user))
                .route("/login", web::post().to(user::login))
                .route("/refresh", web::post().to(user::refresh_token))
        )
        // API Key routes
        .service(
            web::scope("/api_keys")
                .route("", web::post().to(api_key::create_api_key))
                .route("", web::get().to(api_key::list_api_keys))
                .route("/{id}", web::delete().to(api_key::delete_api_key))
        )
        // Job routes
        .service(
            web::scope("/jobs")
                .route("", web::post().to(job::create_job))
                .route("", web::get().to(job::list_jobs))
                .route("/{id}", web::get().to(job::get_job))
                .route("/{id}", web::put().to(job::update_job))
                .route("/{id}", web::delete().to(job::delete_job))
                .route("/{id}/start", web::post().to(job::start_job))
                .route("/{id}/stop", web::post().to(job::stop_job))
                .route("/{id}/status", web::get().to(job::get_job_status))
                .route("/{id}/output", web::get().to(job::get_job_output))
                .route("/{id}/logs", web::get().to(job::get_job_logs))
        )
        // Amber Store routes
        .service(
            web::scope("/amber_store")
                .route("", web::post().to(amber_store::create_amber_store))
                .route("", web::get().to(amber_store::list_amber_stores))
                .route("/{id}", web::get().to(amber_store::get_amber_store))
                .route("/{id}", web::put().to(amber_store::update_amber_store))
                .route("/{id}", web::delete().to(amber_store::delete_amber_store))
        )
        // Vault Store routes
        .service(
            web::scope("/vault_store")
                .route("", web::post().to(vault_store::create_vault_store))
                .route("", web::get().to(vault_store::list_vault_stores))
                .route("/{id}", web::get().to(vault_store::get_vault_store))
                .route("/{id}", web::put().to(vault_store::update_vault_store))
                .route("/{id}", web::delete().to(vault_store::delete_vault_store))
        )
        // Configuration routes
        .service(
            web::scope("/configurations")
                .route("", web::post().to(configuration::create_configuration))
                .route("", web::get().to(configuration::list_configurations))
                .route("/{id}", web::get().to(configuration::get_configuration))
                .route("/{id}", web::put().to(configuration::update_configuration))
                .route("/{id}", web::delete().to(configuration::delete_configuration))
        )
        // Pipeline routes
        .service(
            web::scope("/pipelines")
                .route("", web::post().to(pipeline::create_pipeline))
                .route("", web::get().to(pipeline::list_pipelines))
                .route("/{id}", web::get().to(pipeline::get_pipeline))
                .route("/{id}", web::put().to(pipeline::update_pipeline))
                .route("/{id}", web::delete().to(pipeline::delete_pipeline))
        )
        // Docker File routes
        .service(
            web::scope("/docker_files")
                .route("", web::post().to(docker_file::create_docker_file))
                .route("", web::get().to(docker_file::list_docker_files))
                .route("/{id}", web::get().to(docker_file::get_docker_file))
                .route("/{id}", web::put().to(docker_file::update_docker_file))
                .route("/{id}", web::delete().to(docker_file::delete_docker_file))
        )
        // Worker routes
        .service(
            web::scope("/workers")
                .route("", web::get().to(worker::list_workers))
                .route("/{id}/activate", web::post().to(worker::activate_worker))
                .route("/{id}/deactivate", web::post().to(worker::deactivate_worker))
        )
}
