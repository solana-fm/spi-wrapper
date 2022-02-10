use anchor_lang::AnchorSerialize;
use avro_rs::schema::Schema;
use borsh::BorshDeserialize;
use mpl_auction_house::Deposit;
use serde::Serialize;
use solana_vote_program::vote_instruction::authorize;
use step_token_swap::instruction::{SwapInstruction, unpack};
use tracing::error;
use zeta_cpi::zeta_cpi;

use crate::utils::anchor::unpack_ix_to_sighash;
use crate::{Instruction, TableData, TypedDatum};

pub const PROGRAM_ADDRESS: &str = "stdcqm7Cc8Bj1JZfBPYY8Hyzqqjabm7ugk6k74QEm1B";

pub const ZETA_FUZE_INIT_MARGIN_ACCOUNT_TABLE: &str = "zeta_fuze_init_margin_account";
pub const ZETA_FUZE_LIQUIDITY_TABLE: &str = "zeta_fuze_liquidity";
pub const ZETA_FUZE_INIT_OPEN_ORDERS_TABLE: &str = "zeta_fuze_init_open_orders";
pub const ZETA_FUZE_MARKET_ACCOUNTS_TABLE: &str = "zeta_fuze_market_accounts";
pub const ZETA_FUZE_PLACE_ORDER_TABLE: &str = "zeta_fuze_place_order";
pub const ZETA_FUZE_CANCEL_ACCOUNTS_TABLE: &str = "zeta_fuze_cancel_accounts";
pub const ZETA_FUZE_CANCEL_ORDER_TABLE: &str = "zeta_fuze_cancel_order";

lazy_static! {
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

     pub static ref ZETA_FUZE_LIQUIDITY_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
       "type": "record",
       "name": "zeta_fuze_liquidty",
       "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "liquidity_type", "type": "int"},
            {"name": "state", "type": "string"},
            {"name": "vault", "type": "string"},
            {"name": "margin_account", "type": "string"},
            {"name": "user_token_account", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "greeks", "type": "string"},
            {"name": "socialized_loss_account", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
            {"name": "oracle", "type": ["string", "null"]},
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
pub enum ZetaFuzeDatum {
    InitializeMarginAccount(InitMarginAccount),
    Deposit(ZetaLiquidity),
    Withdraw(ZetaLiquidity),
    InitializeOpenOrders(InitOpenOrders),
    MarketAccounts(ZetaMarketAccounts),
    PlaceOrder(ZetaPlaceOrder),
    CancelAccounts(CancelSharedAccount),
    CancelOrder(ZetaCancelOrder),
}

pub enum LiquidityType {
    Deposit = 0,
    Withdraw = 1,
}

#[derive(Serialize)]
pub struct InitMarginAccount {
    pub tx_hash: String,
    pub margin_account: String,
    pub authority: String,
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct ZetaLiquidity {
    pub tx_hash: String,
    pub liquidity_type: i16,
    pub state: String,
    pub vault: String,
    pub margin_account: String,
    pub user_token_account: String,
    pub authority: String,
    pub greeks: String,
    pub socialized_loss_account: String,
    pub timestamp: i64,
    /// This part onwards only if LiquidityType = 1
    pub oracle: Option<String>,
}

#[derive(Serialize)]
pub struct InitOpenOrders {
    pub tx_hash: String,
    pub state: String,
    pub dex_program: String,
    pub open_orders: String,
    pub margin_account: String,
    pub authority: String,
    pub market: String,
    pub open_orders_map: String,
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct ZetaMarketAccounts {
    pub tx_hash: String,
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
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct ZetaPlaceOrder {
    pub tx_hash: String,
    pub state: String,
    pub margin_account: String,
    pub authority: String,
    pub dex_program: String,
    pub greeks: String,
    pub open_orders: String,
    pub market_accounts: String,
    pub oracle: String,
    pub market_node: String,
    pub market_mint: String,
    pub mint_authority: String,
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct CancelSharedAccount {
    pub tx_hash: String,
    pub state: String,
    pub margin_account: String,
    pub dex_program: String,
    pub open_orders: String,
    pub market: String,
    pub bids: String,
    pub asks: String,
    pub event_queue: String,
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct ZetaCancelOrder {
    pub tx_hash: String,
    pub authority: String,
    pub cancel_accounts: String,
    pub timestamp: i64,
}

pub async fn fragment_instruction(
    instruction: Instruction
) -> Option<Vec<TableData>> {
    /// Convert the instructions into a sighash that anchor uses
    let zeta_sighash = unpack_ix_to_sighash(instruction.data.as_slice());
    let mut response: Vec<TableData> = Vec::new();

    /// Used cargo expand to unpack anchor's macro and find the generated hashes
    /// https://github.com/dtolnay/cargo-expand
    match zeta_sighash {
        [67, 235, 66, 102, 167, 171, 120, 197] => {
            // __private::__global::initialize_margin_account(program_id, accounts, ix_data)
            response.push(TableData {
                schema: (*ZETA_FUZE_INIT_MARGIN_ACCOUNT_SCHEMA).clone(),
                table_name: ZETA_FUZE_INIT_MARGIN_ACCOUNT_TABLE.to_string(),
                data: vec![TypedDatum::ZetaFuze(
                    ZetaFuzeDatum::InitializeMarginAccount(InitMarginAccount {
                        tx_hash: instruction.transaction_hash.to_string(),
                        margin_account: instruction.accounts[0].account.to_string(),
                        authority: instruction.accounts[1].account.to_string(),
                        timestamp: instruction.timestamp,
                    })
                )],
            });
            Some(response)
        }
        [242, 35, 198, 137, 82, 225, 242, 182] => {
            // __private::__global::deposit(program_id, accounts, ix_data)
            response.push(TableData {
                schema: (*ZETA_FUZE_LIQUIDITY_SCHEMA).clone(),
                table_name: ZETA_FUZE_LIQUIDITY_TABLE.to_string(),
                data: vec![TypedDatum::ZetaFuze(
                    ZetaFuzeDatum::Deposit(ZetaLiquidity {
                        tx_hash: instruction.transaction_hash.to_string(),
                        liquidity_type: 0 as i16,
                        state: instruction.accounts[7].account.to_string(),
                        vault: instruction.accounts[1].account.to_string(),
                        margin_account: instruction.accounts[1].account.to_string(),
                        user_token_account: instruction.accounts[3].account.to_string(),
                        authority: instruction.accounts[5].account.to_string(),
                        greeks: instruction.accounts[8].account.to_string(),
                        socialized_loss_account: instruction.accounts[4].account.to_string(),
                        timestamp: instruction.timestamp,
                        oracle: None,
                    })
                )],
            });
            Some(response)
        }
        [183, 18, 70, 156, 148, 109, 161, 34] => {
            // __private::__global::withdraw(program_id, accounts, ix_data)
            response.push(TableData {
                schema: (*ZETA_FUZE_LIQUIDITY_SCHEMA).clone(),
                table_name: ZETA_FUZE_LIQUIDITY_TABLE.to_string(),
                data: vec![TypedDatum::ZetaFuze(
                    ZetaFuzeDatum::Withdraw(ZetaLiquidity {
                        tx_hash: instruction.transaction_hash.to_string(),
                        liquidity_type: 1 as i16,
                        state: instruction.accounts[0].account.to_string(),
                        vault: instruction.accounts[2].account.to_string(),
                        margin_account: instruction.accounts[3].account.to_string(),
                        user_token_account: instruction.accounts[4].account.to_string(),
                        authority: instruction.accounts[6].account.to_string(),
                        greeks: instruction.accounts[7].account.to_string(),
                        socialized_loss_account: instruction.accounts[9].account.to_string(),
                        timestamp: instruction.timestamp,
                        oracle: Some(instruction.accounts[8].account.to_string()),
                    })
                )],
            });
            Some(response)
        }
        [55, 234, 16, 82, 100, 42, 126, 192] => {
            // __private::__global::initialize_open_orders(program_id, accounts, ix_data)
            response.push(TableData {
                schema: (*ZETA_FUZE_INIT_OPEN_ORDER_SCHEMA).clone(),
                table_name: ZETA_FUZE_INIT_OPEN_ORDERS_TABLE.to_string(),
                data: vec![TypedDatum::ZetaFuze(
                    ZetaFuzeDatum::InitializeOpenOrders(InitOpenOrders {
                        tx_hash: instruction.transaction_hash.to_string(),
                        state: instruction.accounts[0].account.to_string(),
                        dex_program: instruction.accounts[2].account.to_string(),
                        open_orders: instruction.accounts[4].account.to_string(),
                        margin_account: instruction.accounts[5].account.to_string(),
                        authority: instruction.accounts[6].account.to_string(),
                        market: instruction.accounts[7].account.to_string(),
                        open_orders_map: instruction.accounts[9].account.to_string(),
                        timestamp: instruction.timestamp,
                    })
                )],
            });
            Some(response)
        }
        [51, 194, 155, 175, 109, 130, 96, 106] => {
            // __private::__global::place_order(program_id, accounts, ix_data)
            response.push(TableData {
                schema: (*ZETA_FUZE_PLACE_ORDER_SCHEMA).clone(),
                table_name: ZETA_FUZE_PLACE_ORDER_TABLE.to_string(),
                data: vec![TypedDatum::ZetaFuze(
                    ZetaFuzeDatum::PlaceOrder(ZetaPlaceOrder {
                        tx_hash: instruction.transaction_hash.to_string(),
                        state: instruction.accounts[0].account.to_string(),
                        margin_account: instruction.accounts[2].account.to_string(),
                        authority: instruction.accounts[3].account.to_string(),
                        dex_program: instruction.accounts[4].account.to_string(),
                        greeks: instruction.accounts[7].account.to_string(),
                        open_orders: instruction.accounts[8].account.to_string(),
                        market_accounts: instruction.accounts[10].account.to_string(),
                        oracle: instruction.accounts[11].account.to_string(),
                        market_node: instruction.accounts[12].account.to_string(),
                        market_mint: instruction.accounts[13].account.to_string(),
                        mint_authority: instruction.accounts[14].account.to_string(),
                        timestamp: instruction.timestamp,
                    })
                )],
            });
            Some(response)
        }
        [95, 129, 237, 240, 8, 49, 223, 132] => {
            // __private::__global::cancel_order(program_id, accounts, ix_data)
            response.push(TableData {
                schema: (*ZETA_FUZE_CANCEL_ACCOUNT_SCHEMA).clone(),
                table_name: ZETA_FUZE_CANCEL_ORDER_TABLE.to_string(),
                data: vec![TypedDatum::ZetaFuze(
                    ZetaFuzeDatum::CancelOrder(ZetaCancelOrder {
                        tx_hash: instruction.transaction_hash.to_string(),
                        authority: instruction.accounts[0].account.to_string(),
                        cancel_accounts: instruction.accounts[1].account.to_string(),
                        timestamp: instruction.timestamp,
                    })
                )],
            });
            Some(response)
        }
        [245, 2, 171, 103, 213, 126, 76, 207] => {
            // __private::__global::read_program_data(program_id, accounts, ix_data)
            None
        }
        _ => {
            error!("[spi-wrapper/programs/zeta_fuze] Error deserializing this system \
        instruction! tx: {}, tx_instruction_id: {}, parent_idx: {}", instruction.transaction_hash,
                                     instruction.tx_instruction_id, instruction.parent_index);

            None
        }
        // _ => Err(anchor_lang::__private::ErrorCode::InstructionFallbackNotFound.into()),
    }
}