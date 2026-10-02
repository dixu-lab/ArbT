pub mod reader;
pub mod writer;
mod bbo;

use serde::{Serialize,Deserialize};
#[derive(Serialize,Deserialize,Debug)]
pub struct Sub<'a>{

    r#type:&'a str,
    coin:&'a str
}
impl<'a> Sub<'a>{
    pub fn new(coin:&'a str)->Self{
        Self {
            r#type: "bbo",
            coin
        }
    }
}
// impl<'a> Sub<'a>{ pub fn new(coin:&'a str)->Self{} }
#[derive(Serialize,Deserialize,Debug)]
pub struct Subscription<'a>{
  pub  method:&'a str,
   pub  subscription: Sub<'a>,
}

impl<'a> Subscription<'a>{
    pub  fn new(coin:&'a str)->Self{
        let  sub = Sub::new(coin);
        Self{
            method:"subscribe",
            subscription:sub

        }
    }
}

// { "type": "trades", "coin": "<coin_symbol>" }
