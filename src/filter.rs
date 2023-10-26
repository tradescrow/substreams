use substreams::Hex;
use substreams::store::StoreGetProto;
use substreams_ethereum::pb::eth::v2::Log;
use crate::abi;
use crate::pb::tradescrow::types::v1::{events, Trade, Trades};
use crate::pb::tradescrow::types::v1::events::trade_event::{TradeAccepted, TradeCanceled, TradeRejected, Type};
use crate::pb::tradescrow::types::v1::events::trade_event::Type::{Accepted, Canceled, Rejected};
use crate::pb::tradescrow::types::v1::events::{FeeEvent, TradeEvent};

pub fn extract_trade_events(
    trade_events: &mut Vec<TradeEvent>,
    trades_store: &StoreGetProto<Trade>,
    log: &Log,
    timestamp_seconds: u64,
    block_number: u64,
    transaction_id: &String,
) {
    if let Some(accepted) = abi::tradescrow::events::TradeAccepted::match_and_decode(log) {
        trade_events.push(TradeEvent {
            log_ordinal: log.ordinal,
            log_index: log.block_index,
            transaction_id: transaction_id.to_string(),
            timestamp: timestamp_seconds,
            block_number,
            r#type: Some(Accepted(TradeAccepted {
                trade_id: accepted.trade_id.to_string(),
                from: Hex(&accepted.from).to_string(),
            })),
        })
    } else if let Some(rejected) = abi::tradescrow::events::TradeRejected::match_and_decode(log) {
        trade_events.push(TradeEvent {
            log_ordinal: log.ordinal,
            log_index: log.block_index,
            transaction_id: transaction_id.to_string(),
            timestamp: timestamp_seconds,
            block_number,
            r#type: Some(Rejected(TradeRejected {
                trade_id: rejected.trade_id.to_string(),
                from: Hex(&rejected.from).to_string(),
            })),
        })
    } else if let Some(canceled) = abi::tradescrow::events::TradeCanceled::match_and_decode(log) {
        trade_events.push(TradeEvent {
            log_ordinal: log.ordinal,
            log_index: log.block_index,
            transaction_id: transaction_id.to_string(),
            timestamp: timestamp_seconds,
            block_number,
            r#type: Some(Canceled(TradeCanceled {
                trade_id: canceled.trade_id.to_string(),
                from: Hex(&canceled.from).to_string(),
            })),
        })
    }
}


pub fn extract_fee_events(
    fee_events: &mut Vec<FeeEvent>,

) {

}
