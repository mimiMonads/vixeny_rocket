use actix_web::{get, web, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryParams {
    q: Option<String>,
}


#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/get/{id}")]
async fn hello(id: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &id)
}

#[get("/get/{name}/{id}/{num}")]
async fn multi_param(info: web::Path<(String, String, String)>) -> impl Responder {
    let (name, id, num) = info.into_inner();
    format!("Name: {}, ID: {}, Num: {}", name, id, num)
}

#[get("/query")]
async fn query(info: web::Query<QueryParams>) -> impl Responder {
    let query_param = info.q.as_deref().unwrap_or_default();
    format!("Recived 'q': {}", query_param)
}

#[get("/a/b/c/d/e/f")]
async fn nested() -> impl Responder {
    "You've reached /a/b/c/d/e/f!"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
           .service(index)
           .service(hello)
           .service(multi_param)
           .service(query)
            .service(nested)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}



