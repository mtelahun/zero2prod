use actix_web::{HttpResponse, Responder, HttpRequest};

pub async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
