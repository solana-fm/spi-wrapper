use avro_rs::schema::Schema;
use borsh::BorshDeserialize;
use serde::Serialize;
use step_token_swap::instruction::SwapInstruction;
use tracing::error;

use crate::{Instruction, TableData, TypedDatum};

pub const PROGRAM_ADDRESS: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

pub const RAYDIUM_AMM_INITALIZE_AMM_TABLE: &str = "raydium_amm_initialize_amm";
pub const RAYDIUM_AMM_DEPOSIT_TABLE: &str = "raydium_amm_deposit";
pub const RAYDIUM_AMM_WITHDRAW_TABLE: &str = "raydium_amm_withdraw";
pub const RAYDIUM_AMM_SWAP_IN_TABLE: &str = "raydium_amm_swap_in";
pub const RAYDIUM_AMM_SWAP_OUT_TABLE: &str = "raydium_amm_swap_out";

lazy_static!{
    pub static ref RAYDIUM_AMM_INITALIZE_AMM_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
       "type": "record",
       "name": "raydium_amm_initalize_amm",
       "fields": [
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

    pub static ref RAYDIUM_AMM_DEPOSIT_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "raydium_amm_deposit",
        "fields": [
            {"name": "amm_account", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "open_orders_account", "type": "string"},
            {"name": "target_orders_account", "type": "string"},
            {"name": "pool_lp_mint_address", "type": "string"},
            {"name": "pool_token_authority", "type": "string"},
            {"name": "pool_token_pc_authority", "type": "string"},
            {"name": "user_coin_token_account", "type": "string"},
            {"name": "user_pc_token_account", "type": "string"},
            {"name": "user_lp_token_account", "type": "string"},
            {"name": "user_main_account", "type": "string"},
            {"name": "max_coin_amount", "type": "long"},
            {"name": "max_pc_amount", "type": "long"},
            {"name": "base_side", "type": "long"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();

     pub static ref RAYDIUM_AMM_WITHDRAW_SCHEMA: Schema = Schema::parse_str(
            r#"
        {
            "type": "record",
            "name": "raydium_amm_withdraw",
            "fields": [
                {"name": "amm_account", "type": "string"},
                {"name": "authority", "type": "string"},
                {"name": "open_orders_account", "type": "string"},
                {"name": "target_orders_account", "type": "string"},
                {"name": "pool_lp_mint_address", "type": "string"},
                {"name": "pool_token_withdraw_coin_account", "type": "string"},
                {"name": "pool_token_withdraw_pc_account", "type": "string"},
                {"name": "withdraw_queue_account", "type": "string"},
                {"name": "token_temp_lp_account", "type": "string"},
                {"name": "coin_vault_account", "type": "string"},
                {"name": "pc_vault_account", "type": "string"},
                {"name": "user_lp_token_account", "type": "string"},
                {"name": "user_coin_token_account", "type": "string"},
                {"name": "user_pc_token_account", "type": "string"},
                {"name": "amount", "type": "long"},
                {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
            ]
        }
        "#
        )
    .unwrap();

    pub static ref RAYDIUM_AMM_SWAP_IN_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "raydium_amm_swap_in",
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
            {"name": "minimum_amount_out", "type": "long"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();

     pub static ref RAYDIUM_AMM_SWAP_OUT_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "raydium_amm_swap_out",
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
            {"name": "max_amount_in", "type": "long"},
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

    Deposit(DepositToken),

    Withdraw(WithdrawToken),

    SwapBaseIn(SwapInstructionIn),

    SwapBaseOut(SwapInstructionOut),
}



#[derive(Serialize)]
pub struct InitializeAMM {
    ///   Initializes a new AmmInfo.
    pub tx_hash : String,
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
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct DepositToken {
    ///   Deposit some tokens into the pool.  The output is a "pool" token representing ownership
    ///   into the pool. Inputs are converted to the current ratio.
    pub tx_hash : String,
    ///   0. `[]` Spl Token program d
    pub amm_account: String,
    /// 2. `[]` $authority
    pub authority: String,
    ///   3. `[]` amm open_orders Account
    pub open_orders_account: String,
    ///   4. `[writable]` amm target_orders Account. To store plan orders infomations.
    pub target_orders_account: String,
    ///   5. `[writable]` pool lp mint address. Must be empty, owned by $authority.
    pub pool_lp_mint_address: String,
    ///   6. `[writable]` pool_token_coin $authority can transfer amount,
    pub pool_token_authority: String,
    ///   7. `[writable]` pool_token_pc $authority can transfer amount,
    pub pool_token_pc_authority: String,
    ///   9. `[writable]` user coin token Base Account to deposit into.
    pub user_coin_token_account: String,
    ///   10. `[writable]` user pc token Base Account to deposit into.
    pub user_pc_token_account: String,
    ///   11. `[writable]` user lp token. To deposit the generated tokens, user is the owner.
    pub user_lp_token_account: String,
    ///   12. '[signer]` user owner Account
    pub user_main_account: String,
    /// Pool token amount to transfer. token_a and token_b amount are set by the current exchange rate and size of the pool
    pub max_coin_amount: i64,
    pub max_pc_amount: i64,
    pub base_side: i64,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct WithdrawToken {
    ///   Withdraw the token from the pool at the current ratio.
    pub tx_hash : String,
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` amm Account
    pub amm_account: String,
    ///   2. `[]` $authority
    pub authority: String,
    ///   3. `[writable]` amm open_orders Account
    pub open_orders_account: String,
    ///   4. `[writable]` amm target_orders Account
    pub target_orders_account: String,
    ///   5. `[writable]` pool lp mint address. Must be empty, owned by $authority.
    pub pool_lp_mint_address: String,
    ///   6. `[writable]` pool_token_coin Amm Account to withdraw FROM,
    pub pool_token_withdraw_coin_account: String,
    ///   7. `[writable]` pool_token_pc Amm Account to withdraw FROM,
    pub pool_token_withdraw_pc_account: String,
    ///   8. `[writable]` withdraw queue Account
    pub withdraw_queue_account: String,
    ///   9. `[writable]` token_temp_lp Account
    pub token_temp_lp_account: String,
    ///   10. `[]` serum dex program id
    ///   11. `[writable]` serum market Account. serum_dex program is the owner.
    ///   12. `[writable]` coin_vault Account
    pub coin_vault_account: String,
    ///   13. `[writable]` pc_vault Account
    pub pc_vault_account: String,
    ///   14. '[]` vault_signer Account
    ///   15. `[writable]` user lp token Account. Source lp, amount is transferable by $authority.
    pub user_lp_token_account: String,
    ///   16. `[writable]` user token coin Account. user Account to credit.
    pub user_coin_token_account: String,
    ///   17. `[writable]` user token pc Account. user Account to credit.
    pub user_pc_token_account: String,
    ///   18. `[singer]` user owner Account
    /// Pool token amount to transfer. token_a and token_b amount are set by
    /// the current exchange rate and size of the pool
    pub amount: i64,

    pub timestamp: i64
}

#[derive(Serialize)]
pub struct SwapInstructionIn {
    /// Swap coin or pc from pool
    pub tx_hash : String,
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
    pub minimum_amount_out: i64,

    pub timestamp: i64
}

#[derive(Serialize)]
pub struct SwapInstructionOut {
    /// Swap coin or pc from pool, base amount_out with a slippage of max_amount_in
    pub tx_hash : String,
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
    pub max_amount_in: i64,
    /// Minimum amount of DESTINATION token to output, prevents excessive slippage
    pub amount_out: i64,

    pub timestamp: i64
}

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
// pub async fn fragment_instruction(
//     // The instruction
//     instruction: Instruction
// ) -> Option<Vec<TableData>> {
//     let raydium_amm_dr = AMMInstruction::try_from_slice(&instruction.data);
//
//     return match token_metadata_dr {
//         Ok(ref bld) => {
//             let deserialized_mtm_ix = bld.clone();
//             let mut response: Vec<TableData> = Vec::new();
//             return match deserialized_mtm_ix {
//                 MetadataInstruction::CreateMetadataAccount(ref mtm_ix) => {
//                     response.push(TableData {
//                         schema: (*METAPLEX_TOKEN_METADATA_CREATED_METADATA_SCHEMA).clone(),
//                         table_name: METAPLEX_TOKEN_METADATA_CREATED_METADATA_TABLE.to_string(),
//                         data: vec![TypedDatum::MetaplexTokenMetadata(
//                             MetaplexTokenMetadataDatum::CreateMetadataAccount(CreatedMetadata {
//                                 metadata: instruction.accounts[0].account.to_string(),
//                                 mint: instruction.accounts[1].account.to_string(),
//                                 mint_authority: instruction.accounts[2].account.to_string(),
//                                 payer: instruction.accounts[3].account.to_string(),
//                                 update_authority: instruction.accounts[4].account.to_string(),
//                                 name: mtm_ix.data.name.to_string(),
//                                 symbol: mtm_ix.data.symbol.to_string(),
//                                 uri: mtm_ix.data.uri.to_string(),
//                                 seller_fee_bips: mtm_ix.data.seller_fee_basis_points as i32,
//                                 timestamp: instruction.timestamp,
//                             })
//                         )],
//                     });
//
//                     let mut creator_data = Vec::new();
//
//                     if let Some(creators) = &mtm_ix.data.creators {
//                         for creator in creators {
//                             creator_data.push(TypedDatum::MetaplexTokenMetadata(
//                                 MetaplexTokenMetadataDatum::CreatorData(Creator {
//                                     token_metadata: instruction.accounts[0].account.to_string(),
//                                     address: creator.address.to_string(),
//                                     verified: creator.verified,
//                                     share: creator.share as i16,
//                                     timestamp: instruction.timestamp
//                                 })));
//                         }
//                     }
//
//                     response.push(TableData {
//                         schema: (*METAPLEX_CREATOR_SCHEMA).clone(),
//                         table_name: METAPLEX_TOKEN_METADATA_CREATOR_TABLE.to_string(),
//                         data: creator_data
//                     });
//
//                     Some(response)
//                 }
//                 MetadataInstruction::UpdateMetadataAccount(ref mtm_ix) => {
//                     response.push(TableData {
//                         schema: (*METAPLEX_UPDATE_METADATA_ACCOUNT).clone(),
//                         table_name: METAPLEX_UPDATE_METADATA_ACCOUNT_TABLE.to_string(),
//                         data: vec![TypedDatum::MetaplexTokenMetadata(
//                             MetaplexTokenMetadataDatum::UpdateMetadataAccount(UpdatedMetadata {
//                                 metadata: instruction.accounts[0].account.to_string(),
//                                 update_authority: instruction.accounts[1].account.to_string(),
//                                 name: if let Some(data) = &mtm_ix.data {
//                                     Some(data.name.to_string())
//                                 } else {
//                                     None
//                                 },
//                                 symbol: if let Some(data) = &mtm_ix.data {
//                                     Some(data.symbol.to_string())
//                                 } else {
//                                     None
//                                 },
//                                 uri: if let Some(data) = &mtm_ix.data {
//                                     Some(data.uri.to_string())
//                                 } else {
//                                     None
//                                 },
//                                 seller_fee_bips: if let Some(data) = &mtm_ix.data {
//                                     Some(data.seller_fee_basis_points as i32)
//                                 } else {
//                                     None
//                                 },
//                                 timestamp: instruction.timestamp,
//                             })
//                         )],
//                     });
//                     Some(response)
//                 }
//                 MetadataInstruction::DeprecatedCreateMasterEdition(ref mtm_ix) => {
//                     response.push(TableData {
//                         schema: (*METAPLEX_DEPRECATED_CREATE_MASTER_EDITION_SCHEMA).clone(),
//                         table_name: METAPLEX_DEPRECATED_CREATE_MASTER_EDITION_TABLE.to_string(),
//                         data: vec![TypedDatum::MetaplexTokenMetadata(
//                             MetaplexTokenMetadataDatum::DeprecatedCreateMasterEdition(CreatedMasterEdition {
//                                 account: instruction.accounts[0].account.to_string(),
//                                 metadata_mint: instruction.accounts[1].account.to_string(),
//                                 printing_mint: instruction.accounts[2].account.to_string(),
//                                 one_time_authorization_printing_mint: instruction.accounts[3].account.to_string(),
//                                 update_authority: instruction.accounts[4].account.to_string(),
//                                 printing_mint_authority: instruction.accounts[5].account.to_string(),
//                                 metadata_mint_authority: instruction.accounts[6].account.to_string(),
//                                 metadata: instruction.accounts[7].account.to_string(),
//                                 payer: instruction.accounts[8].account.to_string(),
//                                 one_time_authorization_printing_mint_authority: instruction.accounts[9].account.to_string(),
//                                 max_supply: if let Some(ms) = mtm_ix.max_supply {
//                                     Some(ms as i64)
//                                 } else {
//                                     None
//                                 },
//                                 timestamp: instruction.timestamp,
//                             })
//                         )],
//                     });
//                     Some(response)
//                 }
//             }
