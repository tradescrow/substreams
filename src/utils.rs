use substreams::{hex, Hex};
use substreams::scalar::BigInt;
use crate::pb::tradescrow::types::v1::{Asset, Trade};

pub const TRADESCROW: [u8; 20] = hex!("4094f921297d986a49dbAD93995D935BE23F1920");

pub fn asset_mapper(assets: &Vec<(Vec<u8>, BigInt, BigInt, BigInt)>) -> Vec<Asset> {
    assets.iter().map(|(addr, id, amount, asset_type)| {
        Asset {
            address: Hex(&addr).to_string(),
            id: id.to_string(),
            amount: amount.to_string(),
            asset_type: asset_type.to_i32(),
        }
    }).collect()
}