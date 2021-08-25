use actix_web::{get, web, App, Error, HttpResponse, HttpServer};

mod domain;
mod repository;

#[get("/items")]
async fn get_item_list() -> Result<HttpResponse, Error> {
    let items = repository::item_find_all();
    Ok(HttpResponse::Ok().json(items))
}

#[get("/items/{item_id}")]
async fn get_item(
    item_id: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let item = repository::item_find_by_id(*item_id);
    Ok(HttpResponse::Ok().json(item))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind = "127.0.0.1:8080";
    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .service(get_item_list)
            .service(get_item)
    })
    .bind(&bind)?
    .run()
    .await
}
