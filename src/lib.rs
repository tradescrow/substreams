mod abi;
mod pb;
mod utils;
mod filter;

use substreams_ethereum::pb::eth;
use crate::pb::tradescrow::types::v1::{Asset, Events, events, Trade, Trades};
use substreams::errors::Error;
use crate::utils::{asset_mapper, TRADESCROW};
use substreams::{log, Hex};
use substreams::store::{StoreGetProto, StoreSetProto};
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
fn map_trades_created(block: eth::v2::Block) -> Result<Trades, Error> {
    use abi::tradescrow::events::TradeCreated;

    Ok(Trades {
        trades: block
            .events::<TradeCreated>(&[&TRADESCROW])
            .filter_map(|(event, log)| {
                log::info!("Trade id: {}", event.trade_id);

                Some(Trade {
                    id: event.trade_id.to_string(),
                    party: Hex(&event.from).to_string(),
                    counterparty: Hex(&event.counterparty).to_string(),
                    party_assets: asset_mapper(&event.party_assets),
                    counterparty_assets: asset_mapper(&event.counterparty_assets),
                    status: 0,
                    fee: "0".to_string(),
                    transaction_id: Hex(&log.receipt.transaction.hash).to_string(),
                    created_at_block_number: block.number,
                    created_at_timestamp: block.timestamp_seconds(),
                    log_ordinal: log.ordinal(),
                    log_index: log.block_index(),
                })
            }).collect()
    })
}

#[substreams::handlers::store]
pub fn store_trades_created(trades: Trades, store: StoreSetProto<Trade>) {
    for trade in trades.trades {
        let id = &trade.id;
        store.set(trade.log_ordinal, format!("trade:{id}"), &trade);
    }
}

#[substreams::handlers::map]
pub fn map_extract_data_types(block: Block, trades_store: StoreGetProto<Trade>) -> Result<Events, Error> {
    let mut events = Events::default();

    let mut trade_events: Vec<events::TradeEvent> = vec![];
    let mut fee_events: Vec<events::FeeEvent> = vec![];
    let mut transactions: Vec<events::Transaction> = vec![];

    let timestamp = block.timestamp_seconds();

    for trx in block.transactions() {
        let transaction_id = Hex(&trx.hash).to_string();
        for (log, call_view) in trx.logs_with_calls() {
            let pool_address = &Hex(log.clone().address).to_string();
            let transactions_id = Hex(&trx.hash).to_string();

            filter::extract_trade_events(&mut trade_events, &trades_store, log, timestamp, block.number, &transactions_id);
            // TODO: Add other filters
        }
    }

    events.trade_events = trade_events;

    Ok(events)
}
