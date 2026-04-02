use serde::{Deserialize,Serialize};
#[derive(Serialize,Deserialize,Debug)]
pub struct CreateOrderResponse{
    pub order_id : String
}
pub struct DeleteOrderResponse{
    pub filled_quantity : u32,
    pub average_price : u32
}
pub struct Depth{
    pub bids:Vec<[u32;2]>,
    pub asks:Vec<[u32;2]>,
    pub lastupdate : String

}