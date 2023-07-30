use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};

#[post("/mouse_entered/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/api/data")]
async fn xxx() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body("<div>From Server</div>"))
}

#[get("/")]
async fn index(req: HttpRequest) -> Result<HttpResponse> {
    println!("{req:?}");
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(include_str!("../views/index.html")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet).service(xxx).service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
