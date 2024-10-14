-- Drop indexes
DROP INDEX IF EXISTS idx_user_llm_configs_provider_id;
DROP INDEX IF EXISTS idx_user_llm_configs_user_id;
DROP INDEX IF EXISTS idx_attachments_message_id;
DROP INDEX IF EXISTS idx_messages_conversation_id;
DROP INDEX IF EXISTS idx_conversations_user_id;
DROP INDEX IF EXISTS idx_jobs_uri;
DROP INDEX IF EXISTS idx_jobs_amber_id;
DROP INDEX IF EXISTS idx_jobs_user_id;
DROP INDEX IF EXISTS idx_active_workers_user_id;
DROP INDEX IF EXISTS idx_docker_files_user_id;
DROP INDEX IF EXISTS idx_pipelines_user_id;
DROP INDEX IF EXISTS idx_configurations_user_id;
DROP INDEX IF EXISTS idx_secure_vault_user_id;
DROP INDEX IF EXISTS idx_amber_store_user_id;
DROP INDEX IF EXISTS idx_api_keys_user_id;

-- Drop tables
DROP TABLE IF EXISTS user_llm_configs;
DROP TABLE IF EXISTS llm_providers;
DROP TABLE IF EXISTS attachments;
DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS conversations;
DROP TABLE IF EXISTS secure_vaults;
DROP TABLE IF EXISTS workers;
DROP TABLE IF EXISTS jobs;
DROP TABLE IF EXISTS active_workers;
DROP TABLE IF EXISTS docker_files;
DROP TABLE IF EXISTS pipelines;
DROP TABLE IF EXISTS configurations;
DROP TABLE IF EXISTS secure_vault;
DROP TABLE IF EXISTS amber_store;
DROP TABLE IF EXISTS api_keys;
DROP TABLE IF EXISTS users;

-- Drop functions
DROP FUNCTION IF EXISTS diesel_manage_updated_at(_tbl regclass);
DROP FUNCTION IF EXISTS diesel_set_updated_at();

-- Drop extensions
DROP EXTENSION IF EXISTS "uuid-ossp";
