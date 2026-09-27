use tokio_tungstenite::WebSocketStream;
use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
// use std::net::TcpStream;
use futures_util::SinkExt;

use tokio_tungstenite::tungstenite::Message;
// use tokio_tungstenite::tungstenite::stream::MaybeTlsStream;
use tokio_tungstenite::MaybeTlsStream;
use crate::binance::Subscription;

pub async fn writer(mut write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>, massage:Subscription<'_>)->Result<(),Box<dyn std::error::Error>>
{
   let sub_massage = serde_json::to_string(&massage)?;
   let _a = write.send(Message::text( sub_massage ) ).await.expect("ERROR: Failed to subscribe to binance");
   Ok(())
}