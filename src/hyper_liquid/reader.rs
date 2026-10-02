use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::MaybeTlsStream;
use futures_util::StreamExt;
use futures_util::stream::SplitStream;
use tokio::net::TcpStream;
use crate::hyper_liquid::bbo::HyperLiquidState;
use crate::state::{Asset, Exchange, MarketState, Quote};

pub async fn read(mut read:
                  SplitStream<WebSocketStream< MaybeTlsStream<TcpStream>>>,
            state: MarketState  ) -> Result<(),Box<dyn std::error::Error+Send + Sync >>
{
    while let Some(massage) = read.next().await {
        match massage {
            Ok(massage) => {
                let value: serde_json::Value = serde_json::from_str(massage.to_text()?)?;

                let channel = value["channel"].as_str();

                match channel {
                    Some("subscriptionResponse") => {
                        println!("Subscription confirmed hyper liquid");
                    }

                    Some("bbo") => {
                        let data: HyperLiquidState = serde_json::from_value(value).expect("error in hyper liquid serde");

                        let d = data.data;
                        match Asset::match_hyperliquid(d.coin.as_str()) {
                            Some(asset)=>{
                                state.update(Exchange::HyperLiquid,asset,Quote::new(
                                    d.bbo[0].px.parse()?,
                                    d.bbo[1].px.parse()?,
                                    d.bbo[1].sz.parse()?,
                                    d.bbo[0].sz.parse()?,
                                    d.time

                                ))
                            }
                            None=>{

                            }
                        }



                    }

                    _ => {
                        println!("Unknown message: {}", value);
                    }
                }


            }
            Err(e) => {
                println!("Hyper liquid Error {}", e);
            }
        }

    }
    Ok(())
}