use actix_web::{test, web, App};
use fws::handlers::metrics::{metrics, init_metrics};

#[actix_web::test]
async fn test_metrics_endpoint() {
    init_metrics();
    let app = test::init_service(App::new().route("/metrics", web::get().to(metrics))).await;
    let req = test::TestRequest::get().uri("/metrics").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
