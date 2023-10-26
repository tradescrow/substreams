// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(enumeration="AssetType", tag="4")]
    pub asset_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trades {
    #[prost(message, repeated, tag="1")]
    pub trades: ::prost::alloc::vec::Vec<Trade>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trade {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration="Status", tag="2")]
    pub status: i32,
    #[prost(string, tag="3")]
    pub party: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub counterparty: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub fee: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="6")]
    pub party_assets: ::prost::alloc::vec::Vec<Asset>,
    #[prost(message, repeated, tag="7")]
    pub counterparty_assets: ::prost::alloc::vec::Vec<Asset>,
    #[prost(uint64, tag="30")]
    pub log_ordinal: u64,
    #[prost(uint32, tag="31")]
    pub log_index: u32,
    #[prost(string, tag="32")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="33")]
    pub created_at_timestamp: u64,
    #[prost(uint64, tag="34")]
    pub created_at_block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub trade_events: ::prost::alloc::vec::Vec<events::TradeEvent>,
    #[prost(message, repeated, tag="2")]
    pub fee_events: ::prost::alloc::vec::Vec<events::FeeEvent>,
    #[prost(message, repeated, tag="3")]
    pub transactions: ::prost::alloc::vec::Vec<events::Transaction>,
}
/// Nested message and enum types in `Events`.
pub mod events {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TradeEvent {
        #[prost(uint64, tag="100")]
        pub log_ordinal: u64,
        #[prost(uint32, tag="101")]
        pub log_index: u32,
        #[prost(string, tag="102")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(uint64, tag="103")]
        pub timestamp: u64,
        #[prost(uint64, tag="104")]
        pub block_number: u64,
        #[prost(oneof="trade_event::Type", tags="1, 2, 3, 4")]
        pub r#type: ::core::option::Option<trade_event::Type>,
    }
    /// Nested message and enum types in `TradeEvent`.
    pub mod trade_event {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TradeCreated {
            #[prost(string, tag="1")]
            pub from: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub trade_id: ::prost::alloc::string::String,
            #[prost(string, tag="3")]
            pub counterparty: ::prost::alloc::string::String,
            #[prost(message, repeated, tag="4")]
            pub party_assets: ::prost::alloc::vec::Vec<super::super::Asset>,
            #[prost(message, repeated, tag="5")]
            pub counterparty_assets: ::prost::alloc::vec::Vec<super::super::Asset>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TradeAccepted {
            #[prost(string, tag="1")]
            pub from: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub trade_id: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TradeRejected {
            #[prost(string, tag="1")]
            pub from: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub trade_id: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TradeCanceled {
            #[prost(string, tag="1")]
            pub from: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub trade_id: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            #[prost(message, tag="1")]
            Created(TradeCreated),
            #[prost(message, tag="2")]
            Accepted(TradeAccepted),
            #[prost(message, tag="3")]
            Rejected(TradeRejected),
            #[prost(message, tag="4")]
            Canceled(TradeCanceled),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FeeEvent {
        #[prost(uint64, tag="100")]
        pub log_ordinal: u64,
        #[prost(uint32, tag="101")]
        pub log_index: u32,
        #[prost(string, tag="102")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(uint64, tag="103")]
        pub timestamp: u64,
        #[prost(uint64, tag="104")]
        pub block_number: u64,
        #[prost(oneof="fee_event::Type", tags="1, 2, 3, 4, 5")]
        pub r#type: ::core::option::Option<fee_event::Type>,
    }
    /// Nested message and enum types in `FeeEvent`.
    pub mod fee_event {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FeePaid {
            #[prost(string, tag="1")]
            pub from: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub amount: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FeeRefunded {
            #[prost(string, tag="1")]
            pub from: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub amount: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FeeReleased {
            #[prost(string, tag="1")]
            pub amount: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FeeUpdated {
            #[prost(string, tag="1")]
            pub from: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub token: ::prost::alloc::string::String,
            #[prost(string, tag="3")]
            pub fee: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TreasuryUpdated {
            #[prost(string, tag="1")]
            pub from: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub treasury: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            #[prost(message, tag="1")]
            Paid(FeePaid),
            #[prost(message, tag="2")]
            Refunded(FeeRefunded),
            #[prost(message, tag="3")]
            Released(FeeReleased),
            #[prost(message, tag="4")]
            Updated(FeeUpdated),
            #[prost(message, tag="5")]
            TreasuryUpdated(TreasuryUpdated),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Transaction {
        #[prost(string, tag="1")]
        pub id: ::prost::alloc::string::String,
        #[prost(uint64, tag="2")]
        pub block_number: u64,
        #[prost(uint64, tag="3")]
        pub timestamp: u64,
        #[prost(uint64, tag="4")]
        pub gas_used: u64,
        /// Integer
        #[prost(string, tag="5")]
        pub gas_price: ::prost::alloc::string::String,
        #[prost(uint64, tag="6")]
        pub log_ordinal: u64,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetType {
    Erc20 = 0,
    Erc1155 = 1,
    Erc721 = 2,
}
impl AssetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AssetType::Erc20 => "ERC20",
            AssetType::Erc1155 => "ERC1155",
            AssetType::Erc721 => "ERC721",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ERC20" => Some(Self::Erc20),
            "ERC1155" => Some(Self::Erc1155),
            "ERC721" => Some(Self::Erc721),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Open = 0,
    Accepted = 1,
    Rejected = 2,
    Canceled = 3,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::Open => "Open",
            Status::Accepted => "Accepted",
            Status::Rejected => "Rejected",
            Status::Canceled => "Canceled",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Open" => Some(Self::Open),
            "Accepted" => Some(Self::Accepted),
            "Rejected" => Some(Self::Rejected),
            "Canceled" => Some(Self::Canceled),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
