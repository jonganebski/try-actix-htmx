use actix_web::{web::Data, HttpRequest, Responder, Result};
use actix_web_lab::respond::Html;
use askama::Template;
use reqwest::Client;

use crate::schemas::Todo;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    todos: &'a Vec<Todo>,
}

pub async fn page(client: Data<Client>, _: HttpRequest) -> Result<impl Responder> {
    let resp = client
        .get("https://fxjvylnmeoaywcatiwcn.supabase.co/rest/v1/todos?select=*")
        .send()
        .await
        .expect("Request failed");

    let todos = resp.json::<Vec<Todo>>().await.unwrap();

    let html = IndexTemplate { todos: &todos }
        .render()
        .expect("Failed to render");
    Ok(Html(html))
}
