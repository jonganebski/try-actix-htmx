use actix_web::{web::Data, HttpRequest, Responder, Result};
use actix_web_lab::respond::Html;
use askama::Template;
use reqwest::Client;

#[derive(Template)]
#[template(path = "index.html")]
struct UserTemplate<'a> {
    name: &'a str,
    text: &'a str,
}

pub async fn page(_: Data<Client>, req: HttpRequest) -> Result<impl Responder> {
    println!("{req:?}");

    let html = UserTemplate {
        name: "Name",
        text: "Text",
    }
    .render()
    .expect("Failed to render");
    Ok(Html(html))
}
