use actix_web::{App, HttpServer, Responder};
use serde_json::error;

#[actix_web::main]
async fn main() -> Result<(),std::io::Error>{
    HttpServer::new(move||{
App::new().service(create_order())
           .service(dele))
    }).bind("127.0.0.1:8000")?.run().await
}
#[post("/order")]
async fn create_order()-> impl Responder{
    "Hello world"
}
#[delete("/delete")]
async fn delete_order()-> impl Responder{
    "delete user"
}
#[get("/deposit")]
async fn get_depthh()->impl Responder{
    "deposit user"
}