use actix_web::{HttpResponse, Responder, body, delete, get, post, web::Json};
use crate::{input::{CreateOrderInput, Deleteorder}, output::{CreateOrderResponse, DeleteOrderResponse}};
#[post{"/order"}]
pub async fn create_order(body:Json<CreateOrderInput>)->impl Responder{
    let price = boddy.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;

    return HttpResponse::Ok().json(CreateOrderResponse{
        order_id:String::from("abs")
    })}
#[get("/depth")]
pub async fn delete_order(body:Json<Deleteorder>)->impl Responder{
    let order_id = body.0.order_id;

    return HttpResponse::Ok().json(DeleteOrderResponse{
        filled_quantity:0,
        average_price:100
    })
}
pub async fn get_depth()->impl Responder{  
      HttpResponse::Ok().json(Depth{
        bids:Vec![],
        asks:Vec![],
        lastUpdate:String::from("abhi")
      })
}
