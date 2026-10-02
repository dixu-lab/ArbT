use std::sync::Arc;
use crate::state::{Asset, Exchange, MarketState};
use crate::state::AssetState;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn calculate_arbitrage(state:&MarketState,asset: &Asset){
    let binance = state.quotes.get(&(asset.clone(),Exchange::Binance));
    let hyperliquid = state.quotes.get(&(asset.clone(),Exchange::HyperLiquid));
    let (Some(binance),Some(hyperliquid))=(binance,hyperliquid) else{
        return;
    };
    let binance_q =binance.value();
    let hyper_q = hyperliquid.value();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    const MAX_QUOTE_AGE: u64 = 250;
    const MAX_TIMESTAMP_DIFF: u64 = 100;

    let binance_age = now.saturating_sub(binance_q.timestamp);
    let hyper_age = now.saturating_sub(hyper_q.timestamp);

    if binance_age > MAX_QUOTE_AGE ||
        hyper_age > MAX_QUOTE_AGE {
        return;
    }

    if binance.timestamp.abs_diff(hyper_q.timestamp) > MAX_TIMESTAMP_DIFF {
        return;
    }

    if binance_q.ask < hyper_q.ask {
        println!(
            "[ARB] {:?} | Binance -> HyperLiquid | Binance ask: {:.4} | HyperLiquid ask: {:.4} | Difference: {:.4}",
            asset,
            binance_q.ask,
            hyper_q.ask,
            hyper_q.ask - binance_q.ask
        );
    }

    if hyper_q.ask < binance_q.ask {
        println!(
            "[ARB] {:?} | HyperLiquid -> Binance | HyperLiquid ask: {:.4} | Binance ask: {:.4} | Difference: {:.4}",
            asset,
            hyper_q.ask,
            binance_q.ask,
            binance_q.ask - hyper_q.ask
        );
    }
}
pub async fn engine(state:MarketState,asset_state:Arc<AssetState>){
    loop {
        asset_state.notify.notified().await;

        calculate_arbitrage(
            &state,
            &asset_state.asset
        );
    }
}

