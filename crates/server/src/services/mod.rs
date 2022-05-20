mod page;

use crate::{middlewares::api_key::ApiKeyMiddlewareFactory, services::page::page_service};
use actix_web::{
  dev::{ServiceFactory, ServiceRequest, ServiceResponse},
  get, web, HttpResponse, Responder, Scope,
};
use serde_json::json;

#[get("/ping")]
async fn ping() -> impl Responder {
  HttpResponse::Ok().json(json!({
    "message": "Hello LyonKit API !"
  }))
}

pub fn api_services() -> Scope<
  impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse,
    Error = actix_web::Error,
    Config = (),
    InitError = (),
  >,
> {
  web::scope("/api")
    .app_data(web::JsonConfig::default().error_handler(|err, _req| {
      actix_web::error::InternalError::from_response(
        "",
        HttpResponse::BadRequest()
          .content_type("application/json")
          .body(format!(r#"{{"code": "JSNER", "message":"{}"}}"#, err)),
      )
      .into()
    }))
    .wrap(ApiKeyMiddlewareFactory::new())
    .service(ping)
    .service(page_service())
}
