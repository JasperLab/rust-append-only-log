use crate::log::{Log, Record};
use actix_web::{web, HttpResponse, Responder};

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

pub(crate) async fn produce(data: web::Data<Log>) -> impl Responder {
    println!("Produced");
    HttpResponse::Ok().body("Produced")
}

pub(crate) async fn consume(data: web::Data<Log>) -> impl Responder {
    println!("Consumed");
    HttpResponse::Ok().body("Consumed")
}
