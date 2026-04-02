use actix_web::{App, HttpServer, Responder,HttpServer,get,post,delete,web::Json};
use serde::{Deserialize,Serialize};
use crate::routes::create_order;
pub mod routes;
pub mod input;
pub mod output;

#[actix_web::main]
async fn main() -> Result<(),std::io::Error>{
    HttpServer::new(move||{
App::new().service(create_order)
           
    }).bind("127.0.0.1:8080")?.run().await
}
