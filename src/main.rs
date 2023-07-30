mod api;
mod pages;
mod schemas;

use actix_files::Files;
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use dotenv;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE},
    Client,
};

use crate::pages::index;

fn build_client() -> reqwest::Result<Client> {
    let anon_key = dotenv::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY missing");

    let mut headers = HeaderMap::new();

    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", &anon_key).parse().unwrap(),
    );
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("apikey", anon_key.parse().unwrap());

    reqwest::Client::builder().default_headers(headers).build()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Data::new(build_client().expect("Reqwest builder failed"));

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(Files::new("/css", "static/css/").show_files_listing())
            .service(web::resource("/api/todo/{id}/toggle").to(api::todo::toggle))
            .service(web::resource("/").to(index::page))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
