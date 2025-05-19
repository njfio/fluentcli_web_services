use crate::schema::secure_vaults;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = secure_vaults)]
pub struct SecureVault {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub encrypted_data: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = secure_vaults)]
pub struct NewSecureVault {
    pub user_id: Uuid,
    pub name: String,
    pub encrypted_data: String,
}

#[derive(Deserialize, Debug)]
pub struct NewSecureVaultPayload {
    pub name: String,
    pub data: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = secure_vaults)]
pub struct UpdateSecureVault {
    pub name: Option<String>,
    pub encrypted_data: Option<String>,
}