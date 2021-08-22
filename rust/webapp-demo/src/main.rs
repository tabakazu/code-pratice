use actix_web::{get, web, App, Error, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
}

#[get("/items/{item_id}")]
async fn get_item(
    item_id: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let item = Item {
        id: *item_id,
        name: "item".to_string(),
    };

    Ok(HttpResponse::Ok().json(item))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind = "127.0.0.1:8080";
    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .service(get_item)
    })
    .bind(&bind)?
    .run()
    .await
}
