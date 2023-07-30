use actix_web::{
    web::{self, Data},
    HttpRequest, Responder, Result,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::schemas::Todo;

#[derive(Debug, Deserialize)]
pub struct UpdateParams {
    id: i32,
}

#[derive(Debug, Serialize)]
pub struct UpdateRequestBody {
    done: bool,
}

pub async fn toggle(
    client: Data<Client>,
    params: web::Path<UpdateParams>,
    _: HttpRequest,
) -> Result<impl Responder> {
    let resp = client
        .get(format!(
            "https://fxjvylnmeoaywcatiwcn.supabase.co/rest/v1/todos?id=eq.{}&select=*",
            params.id
        ))
        .send()
        .await
        .expect("Request failed");

    let todos = resp.json::<Vec<Todo>>().await.unwrap();

    let todo = todos.first().expect("");

    client
        .patch(format!(
            "https://fxjvylnmeoaywcatiwcn.supabase.co/rest/v1/todos?id=eq.{}",
            params.id,
        ))
        .json(&UpdateRequestBody { done: !todo.done })
        .send()
        .await
        .expect("");

    Ok(web::Redirect::to("/"))
}
