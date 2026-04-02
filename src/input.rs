#[derive(Deserialize,Serialize)]
pub struct CreateOrderInput{
    pub price:u32,
    pub quantity : u32,
    pub user_id : u32,
    pub side : Side
}
#[derive(Deserialize,debug)]
pub enum side{
     Buy,
     Sell
}

#[derive(Deserialize,Serialize)]
pub struct Deleteorder{
    pub order_id:String
}
