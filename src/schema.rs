// @generated automatically by Diesel CLI.

diesel::table! {
    active_workers (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        worker_type -> Varchar,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    amber_store (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        data -> Text,
        #[max_length = 255]
        secure_key_hash -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    api_keys (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        key_value -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        expires_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    attachments (id) {
        id -> Uuid,
        message_id -> Uuid,
        #[max_length = 255]
        file_type -> Varchar,
        #[max_length = 255]
        file_path -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    configurations (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        data -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    conversations (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    docker_files (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        content -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    jobs (id) {
        id -> Uuid,
        user_id -> Uuid,
        uri -> Uuid,
        config -> Uuid,
        amber_id -> Nullable<Uuid>,
        state_file_content -> Nullable<Jsonb>,
        data_path -> Nullable<Text>,
        worker_type -> Uuid,
        triggers -> Nullable<Jsonb>,
        timers -> Nullable<Jsonb>,
        #[max_length = 255]
        status -> Varchar,
        results -> Nullable<Jsonb>,
        pipeline_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        started_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    llm_providers (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        provider_type -> Varchar,
        #[max_length = 255]
        api_endpoint -> Varchar,
        supported_modalities -> Jsonb,
        configuration -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    messages (id) {
        id -> Uuid,
        conversation_id -> Uuid,
        #[max_length = 255]
        role -> Varchar,
        content -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    pipelines (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        data -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    secure_vault (id) {
        id -> Uuid,
        user_id -> Uuid,
        data -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    secure_vaults (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        encrypted_data -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_llm_configs (id) {
        id -> Uuid,
        user_id -> Uuid,
        provider_id -> Uuid,
        api_key_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    workers (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        worker_type -> Uuid,
        active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(active_workers -> users (user_id));
diesel::joinable!(amber_store -> users (user_id));
diesel::joinable!(api_keys -> users (user_id));
diesel::joinable!(configurations -> users (user_id));
diesel::joinable!(conversations -> users (user_id));
diesel::joinable!(docker_files -> users (user_id));
diesel::joinable!(jobs -> amber_store (amber_id));
diesel::joinable!(jobs -> configurations (config));
diesel::joinable!(jobs -> docker_files (worker_type));
diesel::joinable!(jobs -> users (user_id));
diesel::joinable!(llm_providers -> users (user_id));
diesel::joinable!(messages -> conversations (conversation_id));
diesel::joinable!(pipelines -> users (user_id));
diesel::joinable!(secure_vault -> users (user_id));
diesel::joinable!(secure_vaults -> users (user_id));
diesel::joinable!(user_llm_configs -> api_keys (api_key_id));
diesel::joinable!(user_llm_configs -> llm_providers (provider_id));
diesel::joinable!(user_llm_configs -> users (user_id));
diesel::joinable!(workers -> docker_files (worker_type));

diesel::allow_tables_to_appear_in_same_query!(
    active_workers,
    amber_store,
    api_keys,
    attachments,
    configurations,
    conversations,
    docker_files,
    jobs,
    llm_providers,
    messages,
    pipelines,
    secure_vault,
    secure_vaults,
    user_llm_configs,
    users,
    workers,
);
