use AmmInstruction::PreInitialize;
use avro_rs::Codec::Null;
use avro_rs::schema::Schema;
use borsh::BorshDeserialize;
use serde::Serialize;
use step_token_swap::instruction::SwapInstruction;
use raydium_contract_instructions::amm_instruction::AmmInstruction;
use serum_dex::error::DexError::ProgramError;
use tracing::error;

use crate::{Instruction, TableData, TypedDatum};

pub const PROGRAM_ADDRESS: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

pub const RAYDIUM_AMM_INITALIZE_AMM_TABLE: &str = "raydium_amm_initialize_amm";
pub const RAYDIUM_AMM_LIQUIDITY_TABLE: &str = "raydium_amm_liquidity";
pub const RAYDIUM_AMM_SWAP_TABLE: &str = "raydium_amm_swap";
// pub const RAYDIUM_AMM_SWAP_IN_TABLE: &str = "raydium_amm_swap_in";
// pub const RAYDIUM_AMM_SWAP_OUT_TABLE: &str = "raydium_amm_swap_out";

/// Missing Raydium IDO Program
/// Missing Raydium LP v1 and v2 programs
/// Missing Raydium Staking Program

lazy_static! {
    pub static ref RAYDIUM_AMM_INITALIZE_AMM_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
       "type": "record",
       "name": "raydium_amm_initalize_amm",
       "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "amm_account", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "open_orders_account", "type": "string"},
            {"name": "pool_lp_mint_address", "type": "string"},
            {"name": "coin_mint_address", "type": "string"},
            {"name": "pc_mint_address", "type": "string"},
            {"name": "pool_token_coin_account", "type": "string"},
            {"name": "pool_token_pc_account", "type": "string"},
            {"name": "withdraw_queue_account", "type": "string"},
            {"name": "token_dest_lp_account", "type": "string"},
            {"name": "token_temp_lp_account", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();

    ///If amount is not null, it is a withdraw instruction
    pub static ref RAYDIUM_AMM_LIQUIDITY_SCHEMA: Schema = Schema::parse_str(
            r#"
    {
        "type": "record",
        "name": "raydium_amm_liquidity",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "liquidity_type", "type": "int"},
            {"name": "amm_account", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "open_orders_account", "type": "string"},
            {"name": "target_orders_account", "type": "string"},
            {"name": "pool_lp_mint_address", "type": "string"},
            {"name": "pool_token_coin", "type": "string"},
            {"name": "pool_token_pc", "type": "string"},
            {"name": "user_coin_token_account", "type": "string"},
            {"name": "user_pc_token_account", "type": "string"},
            {"name": "user_lp_token_account", "type": "string"},
            {"name": "user_main_account", "type": "string"},
            {"name": "withdraw_queue_account", "type": ["string","null"]},
            {"name": "token_temp_lp_account", "type": ["string","null"]},
            {"name": "coin_vault_account", "type": ["string","null"]},
            {"name": "pc_vault_account", "type": ["string","null"]},
            {"name": "max_coin_amount", "type": ["int","null"]},
            {"name": "max_pc_account", "type": ["int","null"]},
            {"name": "withdraw_amount", "type": ["int","null"]},
            {"name": "base_side", "type": ["int", "null"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();

     pub static ref RAYDIUM_AMM_SWAP_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "raydium_amm_swap",
        "fields": [
            {"name": "amm_account", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "open_orders_account", "type": "string"},
            {"name": "amm_target_account", "type": "string"},
            {"name": "pool_token_swap_coin_account", "type": "string"},
            {"name": "pool_token_withdraw_coin_account", "type": "string"},
            {"name": "pool_token_swap_pc_account", "type": "string"},
            {"name": "bids_account", "type": "string"},
            {"name": "asks_account", "type": "string"},
            {"name": "event_q", "type": "string"},
            {"name": "coin_vault_account", "type": "string"},
            {"name": "pc_vault_account", "type": "string"},
            {"name": "user_source_token_account", "type": "string"},
            {"name": "user_destination_token_account", "type": "string"},
            {"name": "user_main_account", "type": "string"},
            {"name": "amount_in", "type": "long"},
            {"name": "amount_out", "type": "long"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
}

#[derive(Serialize)]
pub enum RaydiumAMMDatum {
    Initialize(InitializeAMM),

    Reserved,

    Reserved0,

    Deposit(RaydiumLiquidity),

    Withdraw(RaydiumLiquidity),

    Reserved1,

    Reserved2,

    Reserved3,

    Reserved4,

    SwapBaseIn(RaydiumSwapInstruction),

    PreInitialize,

    SwapBaseOut(RaydiumSwapInstruction),

    Reserved5,
}

pub enum LiquidityType {
    Deposit = 0,
    Withdraw = 1,
}

#[derive(Serialize)]
pub struct InitializeAMM {
    ///   Initializes a new AmmInfo.
    pub tx_hash: String,
    ///   0. `[]` Spl Token program id
    ///   1. `[writable, signer]` New amm Account to create.
    pub amm_account: String,
    ///   2. `[]` $authority derived from `create_program_address(&[amm Account])`
    pub authority: String,
    ///   3. `[]` amm open_orders Account
    pub open_orders_account: String,
    ///   4. `[writable]` pool lp mint address. Must be empty, owned by $authority.
    pub pool_lp_mint_address: String,
    ///   5. `[]` coin mint address
    pub coin_mint_address: String,
    ///   6. `[]` pc mint address
    pub pc_mint_address: String,
    ///   7. `[]` pool_token_coin Account. Must be non zero, owned by $authority.
    pub pool_token_coin_account: String,
    ///   8. `[]` pool_token_pc Account. Must be non zero, owned by $authority.
    pub pool_token_pc_account: String,
    ///   9. '[writable]` withdraw queue Account. To save withdraw dest_coin & dest_pc account with must cancel orders.
    pub withdraw_queue_account: String,
    ///   10. `[writable]` token_dest_lp Account. To deposit the initial pool token supply, user is the owner.
    pub token_dest_lp_account: String,
    ///   11. `[writable]` token_temp_lp Account. To save withdraw lp with must cancle orders as temp to transfer later.
    pub token_temp_lp_account: String,
    ///   12. `[]` serum dex program id
    ///   13. `[]` serum market Account. serum_dex program is the owner.
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct RaydiumLiquidity {
    pub tx_hash: String,
    pub liquidity_type: i16,
    /// Mixed Liquidity
    pub amm_account: String,
    pub authority: String,
    pub open_orders_account: String,
    pub target_orders_account: String,
    pub pool_lp_mint_address: String,
    /// If deposit, this is the account that has the authority to move the token coin.
    /// If withdraw, this is amm account to withdraw pool_token_coin from.
    pub pool_token_coin: String,
    /// The account that has the authority
    pub pool_token_pc: String,
    pub user_coin_token_account: String,
    pub user_pc_token_account: String,
    pub user_lp_token_account: String,
    pub user_main_account: String,

    /// Accounts from here diverges
    pub withdraw_queue_account: Option<String>,
    pub token_temp_lp_account: Option<String>,
    pub coin_vault_account: Option<String>,
    pub pc_vault_account: Option<String>,

    /// Deposit Instructions
    pub max_coin_amount: Option<i64>,
    pub max_pc_amount: Option<i64>,
    pub base_side: Option<i64>,

    ///If amount is not null, it is a withdraw instruction
    pub withdraw_amount: Option<i64>,

    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct RaydiumSwapInstruction {
    /// Swap coin or pc from pool
    pub tx_hash: String,
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` amm Account
    pub amm_account: String,
    ///   2. `[]` $authority
    pub authority: String,
    ///   3. `[writable]` amm open_orders Account
    pub open_orders_account: String,
    ///   4. `[writable]` amm target_orders Account
    pub target_orders_account: String,
    ///   5. `[writable]` pool_token_coin Amm Account to swap FROM or To,
    pub pool_token_swap_coin_account: String,
    ///   6. `[writable]` pool_token_pc Amm Account to swap FROM or To,
    pub pool_token_swap_pc_account: String,
    ///   7. `[]` serum dex program id
    ///   8. `[writable]` serum market Account. serum_dex program is the owner.
    ///   9. `[writable]` bids Account
    pub bids_account: String,
    ///   10. `[writable]` asks Account
    pub asks_account: String,
    ///   11. `[writable]` event_q Account
    pub event_q: String,
    ///   12. `[writable]` coin_vault Account
    pub coin_vault_account: String,
    ///   13. `[writable]` pc_vault Account
    pub pc_vault_account: String,
    ///   14. '[]` vault_signer Account
    ///   15. `[writable]` user source token Account. user Account to swap from.
    pub user_source_token_account: String,
    ///   16. `[writable]` user destination token Account. user Account to swap to.
    pub user_destination_token_account: String,
    ///   17. `[singer]` user owner Account
    pub user_main_account: String,
    // SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
    pub amount_in: i64,
    /// Minimum amount of DESTINATION token to output, prevents excessive slippage
    pub amount_out: i64,

    pub timestamp: i64,
}

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    instruction: Instruction
) -> Option<Vec<TableData>> {
    let raydium_amm_dr = AmmInstruction::unpack(
        instruction.data.as_slice());

    return match raydium_amm_dr {
        Ok(ref rald) => {
            let deserialized_raydium_amm_ix = rald.clone();
            let mut response: Vec<TableData> = Vec::new();
            return match deserialized_raydium_amm_ix {
                AmmInstruction::Initialize(ref raydium_amm_ix) => {
                    response.push(TableData {
                        schema: (*RAYDIUM_AMM_INITALIZE_AMM_SCHEMA).clone(),
                        table_name: RAYDIUM_AMM_INITALIZE_AMM_TABLE.to_string(),
                        data: vec![TypedDatum::RaydiumAMM(
                            RaydiumAMMDatum::Initialize(InitializeAMM {
                                tx_hash: instruction.transaction_hash.to_string(),
                                amm_account: instruction.accounts[1].account.to_string(),
                                authority: instruction.accounts[2].account.to_string(),
                                open_orders_account: instruction.accounts[3].account.to_string(),
                                pool_lp_mint_address: instruction.accounts[4].account.to_string(),
                                coin_mint_address: instruction.accounts[5].account.to_string(),
                                pc_mint_address: instruction.accounts[6].account.to_string(),
                                pool_token_coin_account: instruction.accounts[7].account.to_string(),
                                pool_token_pc_account: instruction.accounts[8].account.to_string(),
                                withdraw_queue_account: instruction.accounts[9].account.to_string(),
                                token_dest_lp_account: instruction.accounts[10].account.to_string(),
                                token_temp_lp_account: instruction.accounts[11].account.to_string(),
                                timestamp: instruction.timestamp,
                            })
                        )],
                    });

                    Some(response)
                }
                AmmInstruction::Deposit(ref raydium_amm_ix) => {
                    response.push(TableData {
                        schema: (*RAYDIUM_AMM_LIQUIDITY_SCHEMA).clone(),
                        table_name: RAYDIUM_AMM_LIQUIDITY_TABLE.to_string(),
                        data: vec![TypedDatum::RaydiumAMM(
                            RaydiumAMMDatum::Deposit(RaydiumLiquidity {
                                tx_hash: instruction.transaction_hash.to_string(),
                                liquidity_type: 0 as i16,
                                amm_account: instruction.accounts[1].account.to_string(),
                                authority: instruction.accounts[2].account.to_string(),
                                open_orders_account: instruction.accounts[3].account.to_string(),
                                target_orders_account: instruction.accounts[4].account.to_string(),
                                pool_lp_mint_address: instruction.accounts[5].account.to_string(),
                                pool_token_coin: instruction.accounts[6].account.to_string(),
                                pool_token_pc: instruction.accounts[7].account.to_string(),
                                user_coin_token_account: instruction.accounts[9].account.to_string(),
                                user_pc_token_account: instruction.accounts[10].account.to_string(),
                                user_lp_token_account: instruction.accounts[11].account.to_string(),
                                user_main_account: instruction.accounts[12].account.to_string(),
                                withdraw_queue_account: None,
                                token_temp_lp_account: None,
                                coin_vault_account: None,
                                pc_vault_account: None,
                                max_coin_amount: Some(raydium_amm_ix.max_coin_amount as i64),
                                max_pc_amount: Some(raydium_amm_ix.max_pc_amount as i64),
                                base_side: Some(raydium_amm_ix.base_side as i64),
                                withdraw_amount: None,
                                timestamp: instruction.timestamp,
                            })
                        )],
                    });

                    Some(response)
                }

                AmmInstruction::Withdraw(ref raydium_amm_ix) => {
                    response.push(TableData {
                        schema: (*RAYDIUM_AMM_LIQUIDITY_SCHEMA).clone(),
                        table_name: RAYDIUM_AMM_LIQUIDITY_TABLE.to_string(),
                        data: vec![TypedDatum::RaydiumAMM(
                            RaydiumAMMDatum::Withdraw(RaydiumLiquidity {
                                tx_hash: instruction.transaction_hash.to_string(),
                                liquidity_type: 1 as i16,
                                amm_account: instruction.accounts[1].account.to_string(),
                                authority: instruction.accounts[2].account.to_string(),
                                open_orders_account: instruction.accounts[3].account.to_string(),
                                target_orders_account: instruction.accounts[4].account.to_string(),
                                pool_lp_mint_address: instruction.accounts[5].account.to_string(),
                                pool_token_coin: instruction.accounts[6].account.to_string(),
                                pool_token_pc: instruction.accounts[7].account.to_string(),
                                user_coin_token_account: instruction.accounts[16].account.to_string(),
                                user_pc_token_account: instruction.accounts[17].account.to_string(),
                                user_lp_token_account: instruction.accounts[15].account.to_string(),
                                user_main_account: instruction.accounts[18].account.to_string(),
                                withdraw_queue_account: Some(instruction.accounts[8].account.to_string()),
                                token_temp_lp_account: Some(instruction.accounts[9].account.to_string()),
                                coin_vault_account: Some(instruction.accounts[12].account.to_string()),
                                pc_vault_account: Some(instruction.accounts[13].account.to_string()),
                                max_coin_amount: None,
                                max_pc_amount: None,
                                base_side: None,
                                withdraw_amount: Some(raydium_amm_ix.amount as i64),
                                timestamp: instruction.timestamp,
                            })
                        )],
                    });

                    Some(response)
                }
                AmmInstruction::SwapBaseIn(ref raydium_amm_ix) => {
                    response.push(TableData {
                        schema: (*RAYDIUM_AMM_SWAP_SCHEMA).clone(),
                        table_name: RAYDIUM_AMM_SWAP_TABLE.to_string(),
                        data: vec![TypedDatum::RaydiumAMM(
                            RaydiumAMMDatum::SwapBaseIn(RaydiumSwapInstruction {
                                tx_hash: instruction.transaction_hash.to_string(),
                                amm_account: instruction.accounts[1].account.to_string(),
                                authority: instruction.accounts[2].account.to_string(),
                                open_orders_account: instruction.accounts[3].account.to_string(),
                                target_orders_account: instruction.accounts[4].account.to_string(),
                                pool_token_swap_coin_account: instruction.accounts[5].account.to_string(),
                                pool_token_swap_pc_account: instruction.accounts[6].account.to_string(),
                                bids_account: instruction.accounts[9].account.to_string(),
                                asks_account: instruction.accounts[10].account.to_string(),
                                event_q: instruction.accounts[11].account.to_string(),
                                coin_vault_account: instruction.accounts[12].account.to_string(),
                                pc_vault_account: instruction.accounts[13].account.to_string(),
                                user_source_token_account: instruction.accounts[15].account.to_string(),
                                user_destination_token_account: instruction.accounts[16].account.to_string(),
                                user_main_account: instruction.accounts[17].account.to_string(),
                                amount_in: raydium_amm_ix.amount_in as i64,
                                amount_out: raydium_amm_ix.minimum_amount_out as i64,
                                timestamp: instruction.timestamp,
                            })
                        )],
                    });

                    Some(response)
                }
                AmmInstruction::SwapBaseOut(ref raydium_amm_ix) => {
                    response.push(TableData {
                        schema: (*RAYDIUM_AMM_SWAP_SCHEMA).clone(),
                        table_name: RAYDIUM_AMM_SWAP_TABLE.to_string(),
                        data: vec![TypedDatum::RaydiumAMM(
                            RaydiumAMMDatum::SwapBaseOut(RaydiumSwapInstruction {
                                tx_hash: instruction.transaction_hash.to_string(),
                                amm_account: instruction.accounts[1].account.to_string(),
                                authority: instruction.accounts[2].account.to_string(),
                                open_orders_account: instruction.accounts[3].account.to_string(),
                                target_orders_account: instruction.accounts[4].account.to_string(),
                                pool_token_swap_coin_account: instruction.accounts[5].account.to_string(),
                                pool_token_swap_pc_account: instruction.accounts[6].account.to_string(),
                                bids_account: instruction.accounts[9].account.to_string(),
                                asks_account: instruction.accounts[10].account.to_string(),
                                event_q: instruction.accounts[11].account.to_string(),
                                coin_vault_account: instruction.accounts[12].account.to_string(),
                                pc_vault_account: instruction.accounts[13].account.to_string(),
                                user_source_token_account: instruction.accounts[15].account.to_string(),
                                user_destination_token_account: instruction.accounts[16].account.to_string(),
                                user_main_account: instruction.accounts[17].account.to_string(),
                                amount_in: raydium_amm_ix.max_amount_in as i64,
                                amount_out: raydium_amm_ix.amount_out as i64,
                                timestamp: instruction.timestamp,
                            })
                        )],
                    });

                    Some(response)
                }
                AmmInstruction::Reserved => {
                    None
                }
                AmmInstruction::Reserved0 => {
                    None
                }
                AmmInstruction::Reserved1 => {
                    None
                }
                AmmInstruction::Reserved2 => {
                    None
                }
                AmmInstruction::Reserved3 => {
                    None
                }
                AmmInstruction::Reserved4 => {
                    None
                }
                AmmInstruction::Reserved5 => {
                    None
                }
                AmmInstruction::PreInitialize(raydium_amm_ix) => {
                    None
                }
            };
        }
        Err(_) => {
            // Error provided has no utility at the moment.
            error!("[spi-wrapper/programs/raydium_amm] Error deserializing this system \
        instruction! tx: {}, tx_instruction_id: {}, parent_idx: {}", instruction.transaction_hash,
                                     instruction.tx_instruction_id, instruction.parent_index);

            None
        }
    };
}
