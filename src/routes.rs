use actix_web::{delete,get,post,web::Json,HttpResponse,Responder};
use crate::{input::CreateOrderInput, output::CreateOrderResponse};
#[post{"/order"}]
pub async fn create_order(body:Json<CreateOrderInput>)->impl Responder{
    let price = boddy.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;

    return HttpResponse::Ok().json(CreateOrderResponse{
        order_id:String::from("abs")
    })}