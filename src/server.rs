use crate::log::{Log, Record};
use actix_web::{HttpResponse, Responder};

struct ProduceRequest {
    record: Record,
}

struct ProduceResponse {
    offset: u64,
}

struct ConsumeRequest {
    offset: u64,
}

struct ConsumeResponse {
    record: Record,
}

pub(crate) async fn produce() -> impl Responder {
    HttpResponse::Ok().body("Produced")
}

pub(crate) async fn consume() -> impl Responder {
    HttpResponse::Ok().body("Consumed")
}
