use crate::db::{create_db_pool, setup_database, DbPool};
use crate::services::agent_service::AgentService;
use uuid::Uuid;

fn setup() -> DbPool {
    dotenv::dotenv().ok();
    let pool = create_db_pool().expect("create pool");
    setup_database(&pool).expect("setup db");
    pool
}

#[test]
fn test_create_session_and_add_agent() {
    let pool = setup();
    let session = AgentService::create_session(&pool, Uuid::new_v4(), "Test".into()).unwrap();
    let agent = AgentService::add_agent(&pool, session.id, Uuid::new_v4(), "planner".into(), None).unwrap();
    assert_eq!(agent.session_id, session.id);
}
