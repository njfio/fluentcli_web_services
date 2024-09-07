DROP TABLE IF EXISTS jobs;
DROP TABLE IF EXISTS pipelines;
DROP TABLE IF EXISTS configurations;
DROP TABLE IF EXISTS amber_store;
DROP TABLE IF EXISTS api_keys;
DROP TABLE IF EXISTS secure_vaults;
DROP TABLE IF EXISTS secure_vault;
DROP TABLE IF EXISTS workers;
DROP TABLE IF EXISTS docker_files;
DROP TABLE IF EXISTS active_workers;
DROP TABLE IF EXISTS users;


DROP FUNCTION IF EXISTS diesel_manage_updated_at(_tbl regclass);
DROP FUNCTION IF EXISTS diesel_set_updated_at();
DROP FUNCTION IF EXISTS diesel_manage_updated_at() CASCADE;
DROP FUNCTION IF EXISTS diesel_set_updated_at() CASCADE;