use std::sync::Arc;
use dashmap::DashMap;
// use serde_json::Value::String;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Exchange{
    HyperLiquid,Binance
}


#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Asset{
    BTC,ETH,BNB
}

#[derive( Clone, Debug)]
pub struct Quote{
  
    bid:f64,
    ask:f64,
    ask_qty:f64,
    bid_qty:f64,
    timestamp:u64

}
#[derive( Clone, Debug)]
pub struct MarketState{
        quotes:Arc<DashMap<(Asset, Exchange),Quote >>     //str is name of the given coin
}

impl MarketState{
    pub fn new()->Self{
        Self{
           quotes: Arc::new(DashMap::new()),
        }
    }

    pub fn update(
        &self,
        exchange: Exchange,
        asset:Asset,
        quote: Quote,

    ){
        self.quotes.insert((asset,exchange),quote);
    }
}


impl Quote{
    pub fn new(bid:f64,
               ask:f64,
               ask_qty:f64,
               bid_qty:f64,
               timestamp:u64,
    )->Self {
        Self {
            bid,
            ask,
            ask_qty,
            bid_qty,
            timestamp,
        }
    }
}


impl Asset {
    pub fn match_binance(symbol: &str) -> Option<Self> {
        match symbol {
            "BTCUSDT" => Some(Self::BTC),
            "ETHUSDT" => Some(Self::ETH),
            "BNBUSDT" => Some(Self::BNB),
            _ => None,
        }
    }

    pub fn match_hyperliquid(symbol: &str) -> Option<Self> {
        match symbol {
            "BTC" => Some(Self::BTC),
            "ETH" => Some(Self::ETH),
            "BNB" => Some(Self::BNB),
            _ => None,
        }
    }
}