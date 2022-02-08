use avro_rs::schema::Schema;
use borsh::BorshDeserialize;
use serde::Serialize;
use step_token_swap::instruction::SwapInstruction;
use tracing::error;

use crate::{Instruction, TableData, TypedDatum};

pub const PROGRAM_ADDRESS: &str = "stdcqm7Cc8Bj1JZfBPYY8Hyzqqjabm7ugk6k74QEm1B";

pub const RAYDIUM_AMM_INITALIZE_AMM_TABLE: &str = "raydium_amm_initialize_amm";
pub const RAYDIUM_AMM_DEPOSIT_TABLE: &str = "raydium_amm_deposit";
pub const RAYDIUM_AMM_WITHDRAW_TABLE: &str = "raydium_amm_withdraw";
pub const RAYDIUM_AMM_SWAP_IN_TABLE: &str = "raydium_amm_swap_in";
pub const RAYDIUM_AMM_SWAP_OUT_TABLE: &str = "raydium_amm_swap_out";


#[derive(Serialize)]
pub enum ZetaFuzeVaultDatum {
    InitializeMarginAccount,
    Deposit,
    Withdraw,
    InitializeOpenOrders,
    MarketAccounts,
    PlaceOrder,
    CancelAccounts,
    CancelOrder,
}

