use actix_web::{HttpResponse, Responder};
use lazy_static::lazy_static;
use prometheus::{Encoder, TextEncoder, Registry, CounterVec, opts};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref HTTP_COUNTER: CounterVec = CounterVec::new(
        opts!("http_requests_total", "Number of HTTP requests"),
        &["method", "path"]
    ).expect("create counter");
}

pub fn init_metrics() {
    let _ = REGISTRY.register(Box::new(HTTP_COUNTER.clone()));
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
