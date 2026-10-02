use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug)]
pub struct Data{
   pub u:u64,
  pub  s:String,
   pub b:String, //best bid price
   pub B:String,// bid Qty
   pub a:String,//best ask price
   pub A:String,//ask Qty

}
#[derive(Deserialize,Serialize,Debug)]
pub struct BinanceState{
   pub stream:String,
    pub data: Data  ,
}