use avro_rs::Schema;
use serde::Serialize;
use serum_dex::instruction::MarketInstruction;
use tracing::error;

use crate::{Instruction, TableData, TypedDatum};

pub const PROGRAM_ADDRESS_V1: &str = "BJ3jrUzddfuSrZHXSCxMUUQsjKEyLmuuyZebkcaFp2fg";
pub const PROGRAM_ADDRESS_V2: &str = "EUqojwWA2rd19FZrzeBncJsm38Jm1hEhE3zsmX3bRc2o";
pub const PROGRAM_ADDRESS_V3: &str = "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin";

pub const SERUM_MARKET_TABLE_NAME: &str = "serum_markets";
pub const SERUM_ORDER_TABLE_NAME: &str = "serum_orders";
pub const SERUM_CANCELLED_ORDER_TABLE_NAME: &str = "serum_cancelled_orders";
pub const SERUM_FEE_SWEEP_TABLE_NAME: &str = "serum_fee_sweeps";
pub const SERUM_SEND_TAKE_TABLE_NAME: &str = "serum_send_takes";
pub const SERUM_PRUNE_TABLE_NAME: &str = "serum_prunes";
pub const SERUM_MARKET_DISABLE_TABLE_NAME: &str = "serum_market_disables";

lazy_static! {
    pub static ref SERUM_MARKETS_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "serum_market",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "market", "type": "string"},
            {"name": "request_queue_account", "type": "string"},
            {"name": "event_queue_account", "type": "string"},
            {"name": "bids_account", "type": "string"},
            {"name": "asks_account", "type": "string"},
            {"name": "coin_account", "type": "string"},
            {"name": "coin_mint", "type": "string"},
            {"name": "price_account", "type": "string"},
            {"name": "price_mint", "type": "string"},
            {"name": "open_order_authority", "type": ["null", "string"]},
            {"name": "prune_authority", "type": ["null", "string"]},
            {"name": "crank_authority", "type": ["null", "string"]},
            {"name": "coin_lot_size", "type": "long"},
            {"name": "price_currency_lot_size", "type": "long"},
            {"name": "fee_rate_bps", "type": "long"},
            {"name": "pc_dust_threshold", "type": "long"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref SERUM_ORDERS_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "serum_order",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "client_order_id", "type": "string"},
            {"name": "order_type", "type": "int"},
            {"name": "side", "type": "int"},
            {"name": "limit", "type": ["null", "int"]},
            {"name": "limit_price", "type": "long"},
            {"name": "max_quantity", "type": "long"},
            {"name": "market", "type": "string"},
            {"name": "self_trade_behavior", "type": ["null", "int"]},
            {"name": "paying_account", "type": "string"},
            {"name": "coin_vault", "type": "string"},
            {"name": "pc_vault", "type": "string"},
            {"name": "msrm_discount_account", "type": ["null", "string"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref SERUM_CANCELLED_ORDERS_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "serum_cancelled_order",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "market", "type": "string"},
            {"name": "side", "type": ["null", "int"]},
            {"name": "order_id", "type": "string"},
            {"name": "open_order_owner", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref SERUM_SEND_TAKES_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "serum_send_take",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "market", "type": "string"},
            {"name": "side", "type": "int"},
            {"name": "limit_price", "type": "long"},
            {"name": "max_quantity", "type": "long"},
            {"name": "max_pc_qty_incl_fees", "type": "long"},
            {"name": "min_coin_qty", "type": "long"},
            {"name": "min_pc_qty", "type": "long"},
            {"name": "coin_wallet_account", "type": "string"},
            {"name": "pc_wallet_account", "type": "string"},
            {"name": "coin_vault", "type": "string"},
            {"name": "pc_vault", "type": "string"},
            {"name": "msrm_discount_account", "type": ["null", "string"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref SERUM_PRUNE_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "serum_prune",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "market", "type": "string"},
            {"name": "limit", "type": "int"},
            {"name": "open_orders", "type": "string"},
            {"name": "open_orders_owner", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref SERUM_FEE_SWEEP_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "serum_fee_sweep",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "market", "type": "string"},
            {"name": "pc_vault", "type": "string"},
            {"name": "fee_authority", "type": "string"},
            {"name": "fee_receivable_account", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref SERUM_MARKET_DISABLE_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "serum_market_disable",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "market", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
}

#[derive(Serialize)]
pub enum SerumMarketDatum {
    Market(SerumMarket),
    Disable(MarketDisable),
    FeeSweep(FeeSweep),
    Order(SerumOrder),
    CancelledOrder(CancelledOrder),
    SendTake(SendTake),
    Prune(Prune)
}

#[derive(Serialize)]
pub struct SerumMarket {
    pub tx_hash : String,
    pub market: String,
    pub request_queue_account: String,
    pub event_queue_account: String,
    pub bids_account: String,
    pub asks_account: String,
    /// The token account for the coin currency relevant to this market.
    pub coin_account: String,
    /// The mint of the coin
    pub coin_mint: String,
    /// The token account for the price currency relevant to this market.
    pub price_account: String,
    /// The mint of the price currency
    pub price_mint: String,
    pub open_order_authority: Option<String>,
    /// Account authorised to clear the books.
    /// open_order_authority must be set if prune_authority is set.
    pub prune_authority: Option<String>,
    /// Account authorised to crank the books.
    /// prune_authority must be set if crank_authority is set.
    pub crank_authority: Option<String>,
    pub coin_lot_size: i64,
    pub price_currency_lot_size: i64,
    pub fee_rate_bps: i64,
    pub pc_dust_threshold: i64,
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct MarketDisable {
    pub tx_hash : String,
    pub market: String,
    pub authority: String,
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct FeeSweep {
    pub tx_hash : String,
    pub market: String,
    pub pc_vault: String,
    pub fee_authority: String,
    pub fee_receivable_account: String,
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct SerumOrder {
    /// Legacy = client_id
    pub tx_hash : String,
    pub client_order_id: String,
    pub order_type: i16,
    pub side: i16,
    pub limit: Option<i16>,
    pub limit_price: i64,
    pub max_quantity: i64,
    pub market: String,
    pub self_trade_behavior: Option<i16>,
    /// The account that will receive the order events.
    pub paying_account: String,
    pub coin_vault: String,
    pub pc_vault: String,
    /// Optional MSRM account linked for fee discounts.
    pub msrm_discount_account: Option<String>,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct CancelledOrder {
    pub tx_hash : String,
    pub market: String,
    pub side: Option<i16>,
    pub order_id: String,
    pub open_order_owner: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct SendTake {
    pub tx_hash : String,
    pub market: String,
    pub side: i16,
    pub limit_price: i64,
    /// Max coin quantity
    pub max_quantity: i64,
    pub max_pc_qty_incl_fees: i64,
    pub min_coin_qty: i64,
    pub min_pc_qty: i64,
    pub coin_wallet_account: String,
    pub pc_wallet_account: String,
    pub coin_vault: String,
    pub pc_vault: String,
    /// Optional MSRM account linked for fee discounts.
    pub msrm_discount_account: Option<String>,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct Prune {
    pub tx_hash : String,
    pub market: String,
    pub limit: i16,
    pub open_orders: String,
    pub open_orders_owner: String,
    pub timestamp: i64
}

pub async fn fragment_instruction(
    // The instruction
    instruction: Instruction
) -> Option<Vec<TableData>> {
    // Unpack the instruction via the spl_token_swap library
    let unpack_result = MarketInstruction::unpack(
        instruction.data.as_slice())
        .ok_or(solana_program::program_error::ProgramError::InvalidArgument);

    return if let Ok(market_instruction) = unpack_result {
        let mut response: Vec<TableData> = Vec::new();

        return match market_instruction {
            MarketInstruction::InitializeMarket(imi) => {
                let table_data = TableData {
                    schema: (*SERUM_MARKETS_SCHEMA).clone(),
                    table_name: SERUM_MARKET_TABLE_NAME.to_string(),
                    data: vec![TypedDatum::SerumMarket(
                        SerumMarketDatum::Market(
                            SerumMarket {
                                tx_hash: instruction.transaction_hash.to_string(),
                                market: instruction.accounts[0].account.to_string(),
                                request_queue_account: instruction.accounts[1].account.to_string(),
                                event_queue_account: instruction.accounts[2].account.to_string(),
                                bids_account: instruction.accounts[3].account.to_string(),
                                asks_account: instruction.accounts[4].account.to_string(),
                                coin_account: instruction.accounts[5].account.to_string(),
                                coin_mint: instruction.accounts[7].account.to_string(),
                                price_account: instruction.accounts[6].account.to_string(),
                                price_mint: instruction.accounts[8].account.to_string(),
                                open_order_authority: if instruction.accounts.len() >= 11 {
                                    Some(instruction.accounts[10].account.to_string())
                                } else {
                                    None
                                },
                                prune_authority: if instruction.accounts.len() >= 11 {
                                    Some(instruction.accounts[11].account.to_string())
                                } else {
                                    None
                                },
                                crank_authority: if instruction.accounts.len() >= 11 {
                                    Some(instruction.accounts[12].account.to_string())
                                } else {
                                    None
                                },
                                coin_lot_size: imi.coin_lot_size as i64,
                                price_currency_lot_size: imi.pc_lot_size as i64,
                                fee_rate_bps: imi.fee_rate_bps as i64,
                                pc_dust_threshold: imi.pc_dust_threshold as i64,
                                timestamp: instruction.timestamp
                            }
                        )
                    )]
                };

                response.push(table_data);

                Some(response)
            }
            MarketInstruction::NewOrder(order) => {
                let table_data = TableData {
                    schema: (*SERUM_ORDERS_SCHEMA).clone(),
                    table_name: SERUM_ORDER_TABLE_NAME.to_string(),
                    data: vec![TypedDatum::SerumMarket(
                        SerumMarketDatum::Order(
                            SerumOrder {
                                tx_hash: instruction.transaction_hash.to_string(),
                                client_order_id: order.client_id.to_string(),
                                order_type: order.order_type as i16,
                                side: order.side as i16,
                                limit: None,
                                limit_price: order.limit_price.get() as i64,
                                max_quantity: order.max_qty.get() as i64,
                                market: instruction.accounts[0].account.to_string(),
                                self_trade_behavior: None,
                                paying_account: instruction.accounts[3].account.to_string(),
                                coin_vault: instruction.accounts[5].account.to_string(),
                                pc_vault: instruction.accounts[6].account.to_string(),
                                msrm_discount_account: if instruction.accounts.len() > 9 {
                                    Some(instruction.accounts[9].account.to_string())
                                } else {
                                    None
                                },
                                timestamp: instruction.timestamp
                            }
                        )
                    )]
                };

                response.push(table_data);

                Some(response)
            }
            MarketInstruction::MatchOrders(_) => {
                None
            }
            MarketInstruction::ConsumeEvents(_) => {
                None
            }
            MarketInstruction::CancelOrder(order) => {
                // 0. `[]` market
                // 1. `[writable]` OpenOrders
                // 2. `[writable]` the request queue
                // 3. `[signer]` the OpenOrders owner
                let table_data = TableData {
                    schema: (*SERUM_CANCELLED_ORDERS_SCHEMA).clone(),
                    table_name: SERUM_CANCELLED_ORDER_TABLE_NAME.to_string(),
                    data: vec![TypedDatum::SerumMarket(
                        SerumMarketDatum::CancelledOrder(
                            CancelledOrder {
                                tx_hash: instruction.transaction_hash.to_string(),
                                side: Some(order.side as i16),
                                order_id: order.order_id.to_string(),
                                market: instruction.accounts[0].account.to_string(),
                                timestamp: instruction.timestamp,
                                open_order_owner: instruction.accounts[3].account.to_string(),
                            }
                        )
                    )]
                };

                response.push(table_data);

                Some(response)
            }
            // TODO: Do we need to track this?
            MarketInstruction::SettleFunds => {
                // 0. `[writable]` market
                // 1. `[writable]` OpenOrders
                // 2. `[signer]` the OpenOrders owner
                // 3. `[writable]` coin vault
                // 4. `[writable]` pc vault
                // 5. `[writable]` coin wallet
                // 6. `[writable]` pc wallet
                // 7. `[]` vault signer
                // 8. `[]` spl token program
                // 9. `[writable]` (optional) referrer pc wallet
                None
            }
            MarketInstruction::CancelOrderByClientId(client_id) => {
                // 0. `[]` market
                // 1. `[writable]` OpenOrders
                // 2. `[writable]` the request queue
                // 3. `[signer]` the OpenOrders owner
                let table_data = TableData {
                    schema: (*SERUM_CANCELLED_ORDERS_SCHEMA).clone(),
                    table_name: SERUM_CANCELLED_ORDER_TABLE_NAME.to_string(),
                    data: vec![TypedDatum::SerumMarket(
                        SerumMarketDatum::CancelledOrder(
                            CancelledOrder {
                                tx_hash: instruction.transaction_hash.to_string(),
                                side: None,
                                order_id: client_id.to_string(),
                                market: instruction.accounts[0].account.to_string(),
                                timestamp: instruction.timestamp,
                                open_order_owner: instruction.accounts[3].account.to_string(),
                            }
                        )
                    )]
                };

                response.push(table_data);

                Some(response)
            }
            MarketInstruction::DisableMarket => {
                // 0. `[writable]` market
                // 1. `[signer]` disable authority
                let table_data = TableData {
                    schema: (*SERUM_MARKET_DISABLE_SCHEMA).clone(),
                    table_name: SERUM_MARKET_DISABLE_TABLE_NAME.to_string(),
                    data: vec![TypedDatum::SerumMarket(
                        SerumMarketDatum::Disable(
                            MarketDisable {
                                tx_hash: instruction.transaction_hash.to_string(),
                                market: instruction.accounts[0].account.to_string(),
                                authority: instruction.accounts[1].account.to_string(),
                                timestamp: instruction.timestamp
                            }
                        )
                    )]
                };

                response.push(table_data);

                Some(response)
            }
            MarketInstruction::SweepFees => {
                // 0. `[writable]` market
                // 1. `[writable]` pc vault
                // 2. `[signer]` fee sweeping authority
                // 3. `[writable]` fee receivable account
                // 4. `[]` vault signer
                // 5. `[]` spl token program
                // 0. `[writable]` market
                // 1. `[signer]` disable authority
                let table_data = TableData {
                    schema: (*SERUM_FEE_SWEEP_SCHEMA).clone(),
                    table_name: SERUM_FEE_SWEEP_TABLE_NAME.to_string(),
                    data: vec![TypedDatum::SerumMarket(
                        SerumMarketDatum::FeeSweep(
                            FeeSweep {
                                tx_hash: instruction.transaction_hash.to_string(),
                                market: instruction.accounts[0].account.to_string(),
                                pc_vault: instruction.accounts[1].account.to_string(),
                                fee_authority: instruction.accounts[2].account.to_string(),
                                fee_receivable_account: instruction.accounts[3].account.to_string(),
                                timestamp: instruction.timestamp
                            }
                        )
                    )]
                };

                response.push(table_data);

                Some(response)
            }
            MarketInstruction::NewOrderV2(order) => {
                // 0. `[writable]` the market
                // 1. `[writable]` the OpenOrders account to use
                // 2. `[writable]` the request queue
                // 3. `[writable]` the (coin or price currency) account paying for the order
                // 4. `[signer]` owner of the OpenOrders account
                // 5. `[writable]` coin vault
                // 6. `[writable]` pc vault
                // 7. `[]` spl token program
                // 8. `[]` the rent sysvar
                // 9. `[writable]` (optional) the (M)SRM account used for fee discounts
                let table_data = TableData {
                    schema: (*SERUM_ORDERS_SCHEMA).clone(),
                    table_name: SERUM_ORDER_TABLE_NAME.to_string(),
                    data: vec![TypedDatum::SerumMarket(
                        SerumMarketDatum::Order(
                            SerumOrder {
                                tx_hash: instruction.transaction_hash.to_string(),
                                client_order_id: order.client_id.to_string(),
                                order_type: order.order_type as i16,
                                side: order.side as i16,
                                limit: None,
                                limit_price: order.limit_price.get() as i64,
                                max_quantity: order.max_qty.get() as i64,
                                market: instruction.accounts[0].account.to_string(),
                                self_trade_behavior: Some(order.self_trade_behavior as i16),
                                paying_account: instruction.accounts[3].account.to_string(),
                                coin_vault: instruction.accounts[5].account.to_string(),
                                pc_vault: instruction.accounts[6].account.to_string(),
                                msrm_discount_account: if instruction.accounts.len() >= 13 {
                                    Some(instruction.accounts[9].account.to_string())
                                } else {
                                    None
                                },
                                timestamp: instruction.timestamp
                            }
                        )
                    )]
                };

                response.push(table_data);

                Some(response)
            }
            MarketInstruction::NewOrderV3(order) => {
                let table_data = TableData {
                    schema: (*SERUM_ORDERS_SCHEMA).clone(),
                    table_name: SERUM_ORDER_TABLE_NAME.to_string(),
                    data: vec![TypedDatum::SerumMarket(
                        SerumMarketDatum::Order(
                            SerumOrder {
                                tx_hash: instruction.transaction_hash.to_string(),
                                client_order_id: order.client_order_id.to_string(),
                                order_type: order.order_type as i16,
                                side: order.side as i16,
                                limit: Some(order.limit as i16),
                                limit_price: order.limit_price.get() as i64,
                                max_quantity: order.max_coin_qty.get() as i64,
                                market: instruction.accounts[0].account.to_string(),
                                self_trade_behavior: Some(order.self_trade_behavior as i16),
                                paying_account: instruction.accounts[6].account.to_string(),
                                coin_vault: instruction.accounts[8].account.to_string(),
                                pc_vault: instruction.accounts[9].account.to_string(),
                                msrm_discount_account: if instruction.accounts.len() >= 13 {
                                    Some(instruction.accounts[12].account.to_string())
                                } else {
                                    None
                                },
                                timestamp: instruction.timestamp
                            }
                        )
                    )]
                };

                response.push(table_data);

                Some(response)
            }
            MarketInstruction::CancelOrderV2(order) => {
                // 0. `[writable]` market
                // 1. `[writable]` bids
                // 2. `[writable]` asks
                // 3. `[writable]` OpenOrders
                // 4. `[signer]` the OpenOrders owner
                // 5. `[writable]` event_q
                let table_data = TableData {
                    schema: (*SERUM_CANCELLED_ORDERS_SCHEMA).clone(),
                    table_name: SERUM_CANCELLED_ORDER_TABLE_NAME.to_string(),
                    data: vec![TypedDatum::SerumMarket(
                        SerumMarketDatum::CancelledOrder(
                            CancelledOrder {
                                tx_hash: instruction.transaction_hash.to_string(),
                                side: Some(order.side as i16),
                                order_id: order.order_id.to_string(),
                                market: instruction.accounts[0].account.to_string(),
                                timestamp: instruction.timestamp,
                                open_order_owner: instruction.accounts[4].account.to_string(),
                            }
                        )
                    )]
                };

                response.push(table_data);

                Some(response)
            }
            MarketInstruction::CancelOrderByClientIdV2(client_id) => {
                // 0. `[writable]` market
                // 1. `[writable]` bids
                // 2. `[writable]` asks
                // 3. `[writable]` OpenOrders
                // 4. `[signer]` the OpenOrders owner
                // 5. `[writable]` event_q
                let table_data = TableData {
                    schema: (*SERUM_CANCELLED_ORDERS_SCHEMA).clone(),
                    table_name: SERUM_CANCELLED_ORDER_TABLE_NAME.to_string(),
                    data: vec![TypedDatum::SerumMarket(
                        SerumMarketDatum::CancelledOrder(
                            CancelledOrder {
                                tx_hash: instruction.transaction_hash.to_string(),
                                side: None,
                                order_id: client_id.to_string(),
                                market: instruction.accounts[0].account.to_string(),
                                timestamp: instruction.timestamp,
                                open_order_owner: instruction.accounts[3].account.to_string(),
                            }
                        )
                    )]
                };

                response.push(table_data);

                Some(response)
            }
            MarketInstruction::SendTake(sti) => {
                // 0. `[writable]` market
                // 1. `[writable]` bids
                // 2. `[writable]` asks
                // 3. `[writable]` OpenOrders
                // 4. `[]`
                let table_data = TableData {
                    schema: (*SERUM_SEND_TAKES_SCHEMA).clone(),
                    table_name: SERUM_SEND_TAKE_TABLE_NAME.to_string(),
                    data: vec![TypedDatum::SerumMarket(
                        SerumMarketDatum::SendTake(
                            SendTake {
                                tx_hash: instruction.transaction_hash.to_string(),
                                market: instruction.accounts[0].account.to_string(),
                                side: sti.side as i16,
                                limit_price: sti.limit_price.get() as i64,
                                max_quantity: sti.max_coin_qty.get() as i64,
                                max_pc_qty_incl_fees: sti.max_native_pc_qty_including_fees.get() as i64,
                                min_coin_qty: sti.min_coin_qty as i64,
                                min_pc_qty: sti.min_native_pc_qty as i64,
                                coin_wallet_account: instruction.accounts[5].account.to_string(),
                                pc_wallet_account: instruction.accounts[6].account.to_string(),
                                coin_vault: instruction.accounts[8].account.to_string(),
                                pc_vault: instruction.accounts[9].account.to_string(),
                                msrm_discount_account: if instruction.accounts.len() >= 13 {
                                    Some(instruction.accounts[12].account.to_string())
                                } else {
                                    None
                                },
                                timestamp: instruction.timestamp
                            }
                        )
                    )]
                };

                response.push(table_data);

                Some(response)
            }
            // TODO: Do we need to track this?
            MarketInstruction::CloseOpenOrders => {
                // 0. `[writable]` OpenOrders
                // 1. `[signer]` the OpenOrders owner
                // 2. `[writable]` the destination account to send rent exemption SOL to
                // 3. `[]` market
                None
            }
            MarketInstruction::InitOpenOrders => {
                // 0. `[writable]` OpenOrders
                // 1. `[signer]` the OpenOrders owner
                // 2. `[writable]` the destination account to send rent exemption SOL to
                // 3. `[]` market
                None
            }
            MarketInstruction::Prune(limit) => {
                let table_data = TableData {
                    schema: (*SERUM_PRUNE_SCHEMA).clone(),
                    table_name: SERUM_PRUNE_TABLE_NAME.to_string(),
                    data: vec![TypedDatum::SerumMarket(
                        SerumMarketDatum::Prune(
                            Prune {
                                tx_hash: instruction.transaction_hash.to_string(),
                                market: instruction.accounts[0].account.to_string(),
                                limit: limit as i16,
                                open_orders: instruction.accounts[4].account.to_string(),
                                open_orders_owner: instruction.accounts[5].account.to_string(),
                                timestamp: instruction.timestamp
                            }
                        )
                    )]
                };

                response.push(table_data);

                Some(response)
            }
            MarketInstruction::ConsumeEventsPermissioned(_) => None
        };
    } else {
        error!("[processors/programs/serum_market] FATAL: Unrecognised instruction for tx: {} \
    with tx_instruction_id: {} and parent_idx: {}", instruction.transaction_hash,
        instruction.tx_instruction_id, instruction.parent_index);
        None
    }
}
