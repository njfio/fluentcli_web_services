use actix_web::{test, web, App};
use crate::handlers::metrics::{metrics, init_metrics, set_scheduled_jobs, SCHEDULED_JOBS_GAUGE};

#[actix_web::test]
async fn test_metrics_endpoint() {
    init_metrics();
    set_scheduled_jobs(5);
    let app = test::init_service(App::new().route("/metrics", web::get().to(metrics))).await;
    let req = test::TestRequest::get().uri("/metrics").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    assert_eq!(SCHEDULED_JOBS_GAUGE.get(), 5);
}
