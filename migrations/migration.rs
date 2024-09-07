diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    api_keys (id) {
        id -> Uuid,
        user_id -> Uuid,
        key_value -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        expires_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    amber_store (id) {
        id -> Uuid,
        user_id -> Uuid,
        data -> Jsonb,
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
    configurations (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        data -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    pipelines (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        data -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    docker_files (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        content -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    active_workers (id) {
        id -> Uuid,
        user_id -> Uuid,
        worker_type -> Varchar,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    jobs (id) {
        id -> Uuid,
        user_id -> Uuid,
        uri -> Text,
        config -> Jsonb,
        amber_id -> Nullable<Uuid>,
        state_file_content -> Nullable<Text>,
        data_path -> Nullable<Text>,
        worker_type -> Varchar,
        triggers -> Nullable<Jsonb>,
        timers -> Nullable<Jsonb>,
        status -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        started_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    workers (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        worker_type -> Uuid, // Reference to docker_file ID
        active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(api_keys -> users (user_id));
diesel::joinable!(amber_store -> users (user_id));
diesel::joinable!(secure_vault -> users (user_id));
diesel::joinable!(configurations -> users (user_id));
diesel::joinable!(pipelines -> users (user_id));
diesel::joinable!(docker_files -> users (user_id));
diesel::joinable!(active_workers -> users (user_id));
diesel::joinable!(jobs -> users (user_id));
diesel::joinable!(jobs -> amber_store (amber_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    api_keys,
    amber_store,
    secure_vault,
    configurations,
    pipelines,
    docker_files,
    active_workers,
    jobs,
);