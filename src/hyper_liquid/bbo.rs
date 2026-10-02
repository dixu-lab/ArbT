use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Order{
   pub px:String,//price
   pub  sz:String,//size
   pub  n:u32, //no of orders at price level
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Data{
   pub coin:String,
   pub  time:u64,
   pub bbo:[Order;2],  //first bid second ask

}
#[derive(Serialize,Deserialize,Debug)]
pub struct HyperLiquidState{
   pub channel:String, //channel => bbo (best bid and offer)
    pub data:Data,
}


