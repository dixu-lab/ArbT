use std::sync::Arc;
use dashmap::DashMap;
use tokio::sync::Notify;
// use serde_json::Value::String;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Exchange{
    HyperLiquid,Binance
}


#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Asset{
    BTC,ETH,BNB
}
#[derive(Debug)]
pub struct  AssetState{
    pub  asset:Asset,
    pub notify: Notify,
}

#[derive( Clone, Debug)]
pub struct Quote{
  
   pub bid:f64,
   pub ask:f64,
   pub ask_qty:f64,
   pub bid_qty:f64,
   pub timestamp:u64

}
#[derive( Clone, Debug)]
pub struct MarketState{
     pub   quotes:Arc<DashMap<(Asset, Exchange),Quote >>,
      pub assets: Arc<DashMap<Asset, Arc<AssetState>>>,
}

impl MarketState{
    pub fn new()->Self{
        let assets=Arc::new(DashMap::new());
        for asset in [Asset::BTC,Asset::ETH,Asset::BNB]{
            assets.insert(
                asset.clone(),
                Arc::new(AssetState{
                    asset,
                    notify:Notify::new(),
                })
            );
        }
        Self{
           quotes: Arc::new(DashMap::new()),
            assets,
        }
    }

    pub fn update(
        &self,
        exchange: Exchange,
        asset:Asset,
        quote: Quote,

    ){
        self.quotes.insert((asset.clone(),exchange),quote);
        if let Some(asset_state) = self.assets.get(&asset) {
            asset_state.notify.notify_one();
        }
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

