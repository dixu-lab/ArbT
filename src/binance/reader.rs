use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::MaybeTlsStream;
use futures_util::StreamExt;
use futures_util::stream::SplitStream;
use tokio::net::TcpStream;
use crate::binance::bbo::BinanceState;
use crate::state::{Asset, Exchange, MarketState, Quote};
use std::time::{SystemTime, UNIX_EPOCH};


pub async fn read(mut read:SplitStream<WebSocketStream< MaybeTlsStream<TcpStream>>>,
                  state: MarketState) ->
Result<(),Box<dyn std::error::Error +Send+ Sync>>
{


    while let Some(massage) = read.next().await {

        match massage {
            Ok(massage) => {
                let value:serde_json::Value = serde_json::from_str(massage.to_text()?)?;

                match value.get("stream") {
                    Some(_)=>{
                        let data:BinanceState= serde_json::from_value(value)?;
                        let d= data.data;
                        match  Asset::match_binance(d.s.as_str()) {
                            Some(asset)=>{
                                let timestamp = SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    ?
                                    .as_millis() as u64;


                                state.update(Exchange::Binance,
                                           asset,
                                           Quote::new(
                                               d.b.parse()?,
                                               d.a.parse()?,
                                               d.A.parse()?,
                                               d.B.parse()?,
                                               timestamp,

                                           )
                                    )
                            }

                            None=>{
                                println!("Eat five star do nothing");
                            }
                        }




                    }
                    None=>{

                    }
                }
            }
           Err(e) => {
            println!("Binance Error {}", e);
           }
        }
    }
    Ok(())
}