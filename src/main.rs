mod hyper_liquid;
mod binance;
// use crate::binance::writer::writer;
use binance::Subscription;
// use std::thread;
use futures_util::{SinkExt, StreamExt};

// Binance : {"e":"aggTrade","E":1790397172156,"s":"BTCUSDT","a":4074288744,"p":"83917.87000000","q":"0.00020000","f":6714723978,"l":6714723978,"T":1790397172156,"m":true,"M":true}

use tokio_tungstenite;
#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>>{
  // let url_binance = "wss://stream.binance.com:9443/ws/btcusdt@aggTrade";
    let url_binance = "wss://stream.binance.com:9443/stream";
    let url_hyperliquid  = "wss://api.hyperliquid-testnet.xyz/ws";



  let (stream_binance,_)=
            tokio_tungstenite::connect_async(url_binance)
            .await
            .expect("failed to connect");


  let (stream_hyper_liquid,_)=
      tokio_tungstenite::connect_async(url_hyperliquid)
          .await
          .expect("failed to connect");


  let ( write_binance,
        read_binance)=stream_binance.split();
  let ( write_hyper_liquid,
     read_hyper_liquid)=stream_hyper_liquid.split();

   let t =  Subscription::new(vec!["bnbusdt@aggTrade"],1);
    let _= binance::writer::writer(write_binance,t).await;
  let binance_handler
      = tokio::spawn(
           binance::reader::read(read_binance)
       );
  let hyperliquid_handler
      = tokio::spawn(
          hyper_liquid::reader::read(read_hyper_liquid)
       );

  let _= tokio::join!(binance_handler,hyperliquid_handler);

Ok(())
}
