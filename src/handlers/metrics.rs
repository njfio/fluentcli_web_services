use actix_web::{HttpResponse, Responder};
use lazy_static::lazy_static;
use prometheus::{Encoder, TextEncoder, Registry, CounterVec, IntGauge, opts};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref HTTP_COUNTER: CounterVec = CounterVec::new(
        opts!("http_requests_total", "Number of HTTP requests"),
        &["method", "path"]
    ).expect("create counter");
    pub static ref SCHEDULED_JOBS_GAUGE: IntGauge = IntGauge::new(
        "scheduled_jobs_total",
        "Number of scheduled jobs"
    ).expect("create gauge");
}

pub fn init_metrics() {
    let _ = REGISTRY.register(Box::new(HTTP_COUNTER.clone()));
    let _ = REGISTRY.register(Box::new(SCHEDULED_JOBS_GAUGE.clone()));
}

pub fn set_scheduled_jobs(count: i64) {
    SCHEDULED_JOBS_GAUGE.set(count);
}

pub async fn metrics() -> impl Responder {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(buffer)
}
