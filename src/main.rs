mod pages;
mod schemas;

use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder, Result,
};
use dotenv;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Client,
};

use crate::pages::index;
use crate::schemas::Todo;

async fn greet(client: Data<Client>, name: web::Path<String>) -> impl Responder {
    let resp = client
        .get("https://fxjvylnmeoaywcatiwcn.supabase.co/rest/v1/todos?select=*")
        .send()
        .await
        .unwrap();

    println!("{:#?}", resp.json::<Vec<Todo>>().await.unwrap());
    format!("Hello {name}!")
}

#[get("/api/data")]
async fn xxx() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body("<div>From Server</div>"))
}

fn build_client() -> reqwest::Result<Client> {
    let anon_key = dotenv::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY missing");

    let mut headers = HeaderMap::new();

    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", &anon_key).parse().unwrap(),
    );
    headers.insert("apikey", anon_key.parse().unwrap());

    reqwest::Client::builder().default_headers(headers).build()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Data::new(build_client().expect("Reqwest builder failed"));

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(web::resource("/mouse_entered/{name}").to(greet))
            .service(xxx)
            .service(web::resource("/").to(index::page))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
