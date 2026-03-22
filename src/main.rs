use actix_web::{App, HttpServer, Responder};
use serde_json::error;

#[actix_web::main]
async fn main() -> Result<(),std::io::Error>{
    HttpServer::new(move||{
App::new().service(create_order())
           .service(delete_order())
           .service(get_depth())
    }).bind("127.0.0.1:8000")?.run().await
}
#[post("/order")]
async fn create_order()-> impl Responder{
    "Hello world"
}
#[delete("/delete")]
async fn delete_order()-> impl Responder{
    "delete user ep"
}
#[get("/deposit")]
async fn get_depth()->impl Responder{
    "deposit user ep"
}