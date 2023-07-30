use actix_web::{
    http::{header::ContentType, StatusCode},
    web::Data,
    HttpRequest, HttpResponse, Result,
};
use reqwest::Client;

pub async fn page(_: Data<Client>, req: HttpRequest) -> Result<HttpResponse> {
    println!("{req:?}");
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(include_str!("./index.html")))
}
