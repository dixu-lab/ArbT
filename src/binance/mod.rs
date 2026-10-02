use serde::{Deserialize, Serialize};

pub mod reader;
pub mod writer;
mod bbo;

#[derive(Serialize,Deserialize,Debug)]
pub struct Subscription<'a>{
    method: & 'a str,
    params:Vec<&'a str>,
    id:i32
}

impl<'a>  Subscription <'a>{
    pub fn new(params:Vec<&'a str>,id:i32)->Self{
        Self{
           method: "SUBSCRIBE",
            params,
            id
        }
    }
}

