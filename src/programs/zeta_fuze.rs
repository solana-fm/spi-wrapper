use avro_rs::schema::Schema;
use borsh::BorshDeserialize;
use mpl_auction_house::Deposit;
use serde::Serialize;
use step_token_swap::instruction::SwapInstruction;
use tracing::error;

use crate::{Instruction, TableData, TypedDatum};

pub const PROGRAM_ADDRESS: &str = "stdcqm7Cc8Bj1JZfBPYY8Hyzqqjabm7ugk6k74QEm1B";

pub const ZETA_FUZE_INIT_MARGIN_ACCOUNT_TABLE: &str = "zeta_fuze_init_margin_account";
pub const ZETA_FUZE_DEPOSIT_TABLE: &str = "zeta_fuze_deposit";
pub const ZETA_FUZE_WITHDRAW_TABLE: &str = "zeta_fuze_withdraw";
pub const ZETA_FUZE_INIT_OPEN_ORDERS_TABLE: &str = "zeta_fuze_init_open_orders";
pub const ZETA_FUZE_MARKET_ACCOUNTS_TABLE: &str = "zeta_fuze_market_accounts";
pub const ZETA_FUZE_PLACE_ORDER_TABLE: &str = "zeta_fuze_place_order";
pub const ZETA_FUZE_CANCEL_ACCOUNTS_TABLE: &str = "zeta_fuze_cancel_accounts";
pub const ZETA_FUZE_CANCEL_ORDER_TABLE: &str = "zeta_fuze_cancel_order";

lazy_static!{
    pub static ref ZETA_FUZE_INIT_MARGIN_ACCOUNT_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
       "type": "record",
       "name": "zeta_fuze_init_margin_account",
       "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "margin_account", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();

     pub static ref ZETA_FUZE_DEPOSIT_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
       "type": "record",
       "name": "zeta_fuze_deposit",
       "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "margin_account", "type": "string"},
            {"name": "vault", "type": "string"},
            {"name": "user_token_account", "type": "string"},
            {"name": "socialized_loss_account", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "state", "type": "string"},
            {"name": "greeks", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();


    pub static ref ZETA_FUZE_WITHDRAW_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
       "type": "record",
       "name": "zeta_fuze_withdraw",
       "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "state", "type": "string"},
            {"name": "vault", "type": "string"},
            {"name": "margin_account", "type": "string"},
            {"name": "user_token_account", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "greeks", "type": "string"},
            {"name": "oracle", "type": "string"},
            {"name": "socialized_loss_account", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();

    pub static ref ZETA_FUZE_INIT_OPEN_ORDER_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
       "type": "record",
       "name": "zeta_fuze_init_open_order",
       "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "state", "type": "string"},
            {"name": "dex_program", "type": "string"},
            {"name": "open_orders", "type": "string"},
            {"name": "margin_account", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "market", "type": "string"},
            {"name": "open_orders_map", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();


     pub static ref ZETA_FUZE_MARKET_ACCOUNTS_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
       "type": "record",
       "name": "zeta_fuze_market_account",
       "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "event_queue", "type": "string"},
            {"name": "bids", "type": "string"},
            {"name": "asks", "type": "string"},
            {"name": "order_payer_token_account", "type": "string"},
            {"name": "coin_vault", "type": "string"},
            {"name": "pc_vault", "type": "string"},
            {"name": "coin_wallet", "type": "string"},
            {"name": "pc_wallet", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();

     pub static ref ZETA_FUZE_PLACE_ORDER_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
       "type": "record",
       "name": "zeta_fuze_place_order",
       "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "state", "type": "string"},
            {"name": "margin_account", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "dex_program", "type": "string"},
            {"name": "greeks", "type": "string"},
            {"name": "open_orders", "type": "string"},
            {"name": "market_accounts", "type": "string"},
            {"name": "oracle", "type": "string"},
            {"name": "market_node", "type": "string"},
            {"name": "market_mint", "type": "string"},
            {"name": "mint_authority", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();

    pub static ref ZETA_FUZE_CANCEL_ACCOUNT_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
       "type": "record",
       "name": "zeta_fuze_cancel_accounts",
       "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "margin_account", "type": "string"},
            {"name": "dex_program", "type": "string"},
            {"name": "open_orders", "type": "string"},
            {"name": "market", "type": "string"},
            {"name": "bids", "type": "string"},
            {"name": "asks", "type": "string"},
            {"name": "event_queue", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();

     pub static ref ZETA_FUZE_CANCEL_ORDER_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
       "type": "record",
       "name": "zeta_fuze_cancel_accounts",
       "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "cancel_accounts", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();

}

#[derive(Serialize)]
pub enum ZetaFuzeVaultDatum {
    InitializeMarginAccount(InitMarginAccount),
    Deposit(DepositTokens),
    Withdraw(WithdrawTokens),
    InitializeOpenOrders(InitOpenOrders),
    MarketAccounts(ZetaMarketAccounts),
    PlaceOrder(ZetaPlaceOrder),
    CancelAccounts(CancelSharedAccount),
    CancelOrder(ZetaCancelOrder),
}

#[derive(Serialize)]
pub struct InitMarginAccount {
    pub tx_hash : String,
    pub margin_account: String,
    pub authority: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct DepositTokens {
    pub tx_hash : String,
    /// Contains Balance and positions
    pub margin_account: String,
    pub vault: String,
    pub user_token_account: String,
    pub socialized_loss_account: String,
    pub authority: String,
    pub state: String,
    pub greeks: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct WithdrawTokens {
    pub tx_hash : String,
    pub state : String,
    pub vault : String,
    pub margin_account: String,
    pub user_token_account: String,
    pub authority: String,
    pub greeks: String,
    pub oracle: String,
    pub socialized_loss_account: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct InitOpenOrders {
    pub tx_hash : String,
    pub state: String,
    pub dex_program: String,
    pub open_orders: String,
    pub margin_account: String,
    pub authority: String,
    pub market: String,
    pub open_orders_map: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct ZetaMarketAccounts {
    pub tx_hash : String,
    pub event_queue: String,
    pub bids: String,
    pub asks: String,
    pub order_payer_token_account: String,
    // Also known as the "base" currency. For a given A/B market,
    // this is the vault for the A mint.
    pub coin_vault: String,
    // Also known as the "quote" currency. For a given A/B market,
    // this is the vault for the B mint.
    pub pc_vault: String,
    pub coin_wallet: String,
    pub pc_wallet: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct ZetaPlaceOrder {
    pub tx_hash : String,
    pub state: String,
    pub margin_account: String,
    pub authority: String,
    pub dex_program: String,
    pub greeks: String,
    pub open_orders: String,
    pub market_accounts: String,
    pub oracle: String,
    pub market_node:String,
    pub market_mint: String,
    pub mint_authority: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct CancelSharedAccount {
    pub tx_hash : String,
    pub state: String,
    pub margin_account: String,
    pub dex_program: String,
    pub open_orders: String,
    pub market: String,
    pub bids: String,
    pub asks: String,
    pub event_queue: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct ZetaCancelOrder {
    pub tx_hash : String,
    pub authority: String,
    pub cancel_accounts: String,
    pub timestamp: i64
}

