use crate::handlers::chat::{
    create_conversation, create_message, delete_conversation, delete_message, get_conversation,
    get_messages, list_conversations,
};
use crate::handlers::llm_provider::{
    create_llm_provider, create_user_llm_config, delete_llm_provider, delete_user_llm_config,
    get_llm_provider, get_user_llm_config, list_user_llm_configs, update_llm_provider,
    update_user_llm_config,
};
use crate::handlers::{
    amber_store, api_key, agent, attachment, configuration, docker_file, fluentcli, job, llm, pipeline,
    secure_vault, stream_chat, temp_image, user, worker,
};
use crate::utils::auth::Auth;
use actix_web::{web, Scope};

pub fn configure_routes() -> Scope {
    web::scope("")
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
        .service(
            web::scope("/api_keys")
                .wrap(Auth)
                .route("", web::post().to(api_key::create_api_key))
                .route("", web::get().to(api_key::list_api_keys))
                .route("/{id}", web::get().to(api_key::get_api_key))
                .route("/{id}", web::put().to(api_key::update_api_key))
                .route("/{id}", web::delete().to(api_key::delete_api_key)),
        )
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
        .service(
            web::scope("/amber_stores")
                .wrap(Auth)
                .route("", web::post().to(amber_store::create_amber_store))
                .route("", web::get().to(amber_store::list_amber_stores))
                .route("/{id}", web::get().to(amber_store::get_amber_store))
                .route("/{id}", web::put().to(amber_store::update_amber_store))
                .route("/{id}", web::delete().to(amber_store::delete_amber_store)),
        )
        .service(
            web::scope("/secure_vaults")
                .wrap(Auth)
                .route("", web::post().to(secure_vault::create_secure_vault))
                .route("", web::get().to(secure_vault::list_secure_vaults))
                .route("/{id}", web::get().to(secure_vault::get_secure_vault))
                .route("/{id}", web::put().to(secure_vault::update_secure_vault))
                .route("/{id}", web::delete().to(secure_vault::delete_secure_vault)),
        )
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
        .service(
            web::scope("/pipelines")
                .wrap(Auth)
                .route("", web::post().to(pipeline::create_pipeline))
                .route("", web::get().to(pipeline::list_pipelines))
                .route("/{id}", web::get().to(pipeline::get_pipeline))
                .route("/{id}", web::put().to(pipeline::update_pipeline))
                .route("/{id}", web::delete().to(pipeline::delete_pipeline)),
        )
        .service(
            web::scope("/docker_files")
                .wrap(Auth)
                .route("", web::post().to(docker_file::create_docker_file))
                .route("", web::get().to(docker_file::list_docker_files))
                .route("/{id}", web::get().to(docker_file::get_docker_file))
                .route("/{id}", web::put().to(docker_file::update_docker_file))
                .route("/{id}", web::delete().to(docker_file::delete_docker_file)),
        )
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
        .service(
            web::scope("/chat")
                .wrap(Auth)
                .route("/conversations", web::post().to(create_conversation))
                .route("/conversations", web::get().to(list_conversations))
                .route("/conversations/{id}", web::get().to(get_conversation))
                .route("/conversations/{id}", web::delete().to(delete_conversation))
                .route("/messages", web::post().to(create_message))
                .route("/conversations/{id}/messages", web::get().to(get_messages))
                .route(
                    "/conversations/{conversation_id}/messages/{message_id}",
                    web::delete().to(delete_message),
                )
                .service(attachment::get_attachment)
                .service(attachment::create_attachment)
                .service(attachment::get_message_attachments)
                .service(attachment::delete_attachment)
                .route("/stream", web::get().to(stream_chat::stream_chat)),
        )
        .service(
            web::scope("/llm")
                .wrap(Auth)
                .route("/providers", web::post().to(create_llm_provider))
                .route("/providers", web::get().to(llm::get_llm_providers))
                .route("/providers/{id}", web::get().to(get_llm_provider))
                .route("/providers/{id}", web::put().to(update_llm_provider))
                .route("/providers/{id}", web::delete().to(delete_llm_provider))
                .route("/chat", web::post().to(llm::llm_chat))
                .route("/stream_chat", web::post().to(llm::llm_stream_chat))
                .route("/user-configs", web::post().to(create_user_llm_config))
                .route("/user-configs", web::get().to(list_user_llm_configs))
                .route("/user-configs/{id}", web::get().to(get_user_llm_config))
                .route("/user-configs/{id}", web::put().to(update_user_llm_config))
                .route(
                    "/user-configs/{id}",
                    web::delete().to(delete_user_llm_config),
                ),
        )
        .service(
            web::scope("/agents")
                .wrap(Auth)
                .route("/sessions", web::post().to(agent::create_session))
                .route(
                    "/sessions/{session_id}/agents",
                    web::post().to(agent::add_agent),
                )
                .route(
                    "/sessions/{session_id}/step",
                    web::post().to(agent::run_step),
                )
                .route(
                    "/sessions/{session_id}",
                    web::delete().to(agent::end_session),
                ),
        )
        .service(temp_image::get_temp_image)
}
