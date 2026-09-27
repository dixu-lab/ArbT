use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::MaybeTlsStream;
use futures_util::StreamExt;
use futures_util::stream::SplitStream;
use tokio::net::TcpStream;
// use futures_util::stream::
pub async fn read(mut read:SplitStream<WebSocketStream< MaybeTlsStream<TcpStream>>>)
{
    while let Some(massage) = read.next().await {
        match massage {
            Ok(massage) => {
                println!("Binance : {}", massage);
            }
           Err(e) => {
            println!("Binance Error {}", e);
           }
        }
    }
}