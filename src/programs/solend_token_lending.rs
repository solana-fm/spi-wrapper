// use avro_rs::Schema;
// use std::collections::HashMap;
// use itertools::Itertools;
// use serde::Serialize;
// use solana_program::program_error::ProgramError;
// use solana_sdk::pubkey::Pubkey;
// use crate::programs::solend::instruction::LendingInstruction;
// use tracing::error;
//
// use crate::{Instruction, TableData};
//
// pub const PROGRAM_ADDRESS: &str = "So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo";
//
// pub const SOLEND_TOKEN_LENDING_MARKET_TABLE: &str = "solend_token_lending_markets";
// pub const SOLEND_TOKEN_LENDING_OWNER_STATE_TABLE: &str = "solend_token_lending_owner_states";
// pub const SOLEND_TOKEN_LENDING_MARKET_RESERVE_TABLE: &str = "solend_token_lending_market_reserves";
// pub const SOLEND_TOKEN_LENDING_RESERVE_LIQUIDITY_TABLE: &str = "solend_token_lending_reserve_liquidities";
// pub const SOLEND_TOKEN_LENDING_OBLIGATION_TABLE: &str = "solend_token_lending_obligations";
//
// lazy_static! {
//     pub static ref SOLEND_TOKEN_LENDING_MARKET_SCHEMA: Schema = Schema::parse_str(
//         r#"
//     {
//         "type": "record",
//         "name": "solend_token_lending_market",
//         "fields": [
//             {"name": "market_account", "type": "string"},
//             {"name": "token_program", "type": "string"},
//             {"name": "oracle_program", "type": "string"},
//             {"name": "quote_currency", "type": "string"},
//             {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
//         ]
//     }
//     "#
//     )
//     .unwrap();
//     pub static ref SOLEND_TOKEN_LENDING_OWNER_STATE_SCHEMA: Schema = Schema::parse_str(
//         r#"
//     {
//         "type": "record",
//         "name": "solend_token_lending_owner_state",
//         "fields": [
//             {"name": "account", "type": "string"},
//             {"name": "new_owner", "type": "string"},
//             {"name": "owner", "type": "string"},
//             {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
//         ]
//     }
//     "#
//     )
//     .unwrap();
//     pub static ref SOLEND_TOKEN_LENDING_MARKET_RESERVE_SCHEMA: Schema = Schema::parse_str(
//         r#"
//     {
//         "type": "record",
//         "name": "solend_token_lending_market_reserve",
//         "fields": [
//             {"name": "source_liquidity_account", "type": "string"},
//             {"name": "collateral_account", "type": "string"},
//             {"name": "collateral_mint", "type": "string"},
//             {"name": "account", "type": "string"},
//             {"name": "mint", "type": "string"},
//             {"name": "liquidity_account", "type": "string"},
//             {"name": "liquidity_fee_account", "type": "string"},
//             {"name": "collateral_token_supply", "type": "string"},
//             {"name": "oracle_account", "type": "string"},
//             {"name": "oracle_price_account", "type": "string"},
//             {"name": "lending_market_account", "type": "string"},
//             {"name": "lending_market_authority", "type": "string"},
//             {"name": "lending_market_owner", "type": "string"},
//             {"name": "user_transfer_authority", "type": "string"},
//             {"name": "token", "type": "string"},
//             {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
//         ]
//     }
//     "#
//     )
//     .unwrap();
//     pub static ref SOLEND_TOKEN_LENDING_RESERVE_LIQUIDITY_SCHEMA: Schema = Schema::parse_str(
//         r#"
//     {
//         "type": "record",
//         "name": "solend_token_lending_reserve_liquidity",
//         "fields": [
//             {"name": "source", "type": "string"},
//             {"name": "destination", "type": "string"},
//             {"name": "amount", "type": "long"},
//             {"name": "mint", "type": "string" },
//             {"name": "lending_market_account", "type": "string"},
//             {"name": "lending_market_authority", "type": "string"},
//             {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
//         ]
//     }
//     "#
//     )
//     .unwrap();
//     pub static ref SOLEND_TOKEN_LENDING_OBLIGATION_SCHEMA: Schema = Schema::parse_str(
//         r#"
//     {
//         "type": "record",
//         "name": "solend_token_lending_obligation",
//         "fields": [
//             {"name": "obligation_type", "type": "int"},
//             {"name": "source", "type": "string"},
//             {"name": "destination", "type": "string"},
//             {"name": "amount", "type": "long"},
//             {"name": "lending_market_account", "type": "string"},
//             {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
//         ]
//     }
//     "#
//     )
//     .unwrap();
// }
//
// #[derive(Serialize)]
// pub struct LendingMarket {
//     pub market_account: String,
//     pub token_program: String,
//     pub oracle_program: String,
//     pub quote_currency: String,
//     pub timestamp: i64
// }
//
// #[derive(Serialize)]
// pub struct LendingMarketOwnerState {
//     pub account: String,
//     pub new_owner: String,
//     pub owner: String,
//     pub timestamp: i64
// }
//
// #[derive(Serialize)]
// pub struct LendingMarketReserve {
//     pub source_liquidity_account: String,
//     pub collateral_account: String,
//     pub collateral_mint: String,
//     pub account: String,
//     pub mint: String,
//     pub liquidity_account: String,
//     pub liquidity_fee_account: String,
//     pub collateral_token_supply: String,
//     pub oracle_account: String,
//     pub oracle_price_account: String,
//     pub lending_market_account: String,
//     pub lending_market_authority: String,
//     pub lending_market_owner: String,
//     pub user_transfer_authority: String,
//     pub token: String,
//     /// We're going to push the liquidity amount into another more useful table
//     /// so we can track the amount of liquidity in the reserve historically
//     // pub liquidity_amount: i64,
//     pub timestamp: i64
// }
//
// #[derive(Serialize)]
// pub struct ReserveLiquidity {
//     /// If this is a deposit, this will be the User transfer authority ($authority).
//     /// If this is a withdrawal, this will be the Reserve collateral SPL Token mint.
//     pub source: String,
//     /// If this is a deposit, this will be the Reserve liquidity supply SPL Token account.
//     /// If this is a withdrawal, this will be the User transfer authority ($authority).
//     pub destination: String,
//     pub amount: i64,
//     /// Mint involved in the reserve liquidity transfer (a.k.a Reserve Collateral SPL Token mint)
//     /// If this is a deposit, this is the collateral's mint
//     /// If this is a withdrawal, this is the burnt's mint
//     pub mint: String,
//     pub lending_market_account: String,
//     pub lending_market_authority: String,
//     pub timestamp: i64,
// }
//
// #[derive(Serialize)]
// pub enum ObligationType {
//     /// A user is trying to borrow an obligation
//     Borrow = 1,
//     /// A user is trying to top up an obligation to reduce exposure.
//     Deposit = 2,
//     /// A user is trying to repay an obligation
//     Repay = 3,
//     /// A user is trying to increase his/her obligation's
//     /// exposure level by reducing the collateral size.
//     Withdraw = 4,
//     /// User got margin called for his/her obligation. Kekw
//     Liquidate = 5,
//     /// Flash loan
//     FlashLoan = 6
// }
//
// #[derive(Serialize)]
// pub struct Obligation {
//     pub obligation_type: i16,
//     /// If this is a withdraw, this will be the Source borrow reserve liquidity supply SPL Token account.
//     /// If this is a borrow, this will be the Obligation owner.
//     pub source: String,
//     /// If this is a withdraw, this will be the Source withdraw reserve collateral supply SPL Token account.
//     /// If this is a borrow, this will be the Obligation owner.
//     pub destination: String,
//     /// The amount of collateral tokens to be withdrawn or deposited.
//     /// -ve means the loan (the user is obligated to pay) is being repayed, +ve means the loan is being lent
//     /// 0 if its being initialised.
//     pub amount: i64,
//     pub lending_market_account: String,
//     pub timestamp: i64
// }
//
// pub async fn fragment_instruction<T: Serialize>(
//     // The instruction
//     instruction: Instruction
// ) -> Option<Vec<TableData<T>>> {
//     // Unpack the instruction via the spl_token_swap library
//     let unpack_result = LendingInstruction::unpack(
//         instruction.data.as_slice());
//
//     return match unpack_result {
//         Ok(ref unpacked) => {
//             let mut response: Vec<TableData<T>> = Vec::new();
//             let lending_instruction = unpacked.clone();
//             match lending_instruction {
//                 LendingInstruction::InitLendingMarket {
//                     owner,
//                     quote_currency,
//                 } => {
//                     let market_state_key =
//                         (SOLEND_TOKEN_LENDING_MARKET_TABLE.to_string(), *SOLEND_TOKEN_LENDING_MARKET_SCHEMA);
//                     let market_state = LendingMarket {
//                         market_account: instruction.accounts[0].account.to_string(),
//                         token_program: instruction.accounts[2].account.to_string(),
//                         oracle_program: instruction.accounts[3].account.to_string(),
//                         quote_currency: Pubkey::new(&quote_currency).to_string(),
//                         timestamp: instruction.timestamp
//                     };
//
//                     if response.contains(&market_state_key) {
//                         response[&market_state_key].push(market_state);
//                     } else {
//                         response[&market_state_key] = vec![market_state];
//                     }
//
//                     let owner_state_key =
//                         (SOLEND_TOKEN_LENDING_OWNER_STATE_TABLE.to_string(), *SOLEND_TOKEN_LENDING_OWNER_STATE_SCHEMA);
//                     let owner_state = LendingMarketOwnerState {
//                         account: instruction.accounts[0].account.to_string(),
//                         new_owner: owner.to_string(),
//                         owner: "".to_string(),
//                         timestamp: instruction.timestamp
//                     };
//
//                     if response.contains(&owner_state_key) {
//                         response[&owner_state_key].push(owner_state);
//                     } else {
//                         response[&owner_state_key] = vec![owner_state];
//                     }
//
//                     Some(response)
//                 }
//                 LendingInstruction::SetLendingMarketOwner { new_owner } => {
//                     let owner_state_key =
//                         (SOLEND_TOKEN_LENDING_OWNER_STATE_TABLE.to_string(), *SOLEND_TOKEN_LENDING_OWNER_STATE_SCHEMA);
//                     let owner_state = LendingMarketOwnerState {
//                         account: instruction.accounts[0].account.to_string(),
//                         new_owner: new_owner.to_string(),
//                         owner: instruction.accounts[1].account.to_string(),
//                         timestamp: instruction.timestamp
//                     };
//
//                     if response.contains(&owner_state_key) {
//                         response[&owner_state_key].push(owner_state);
//                     } else {
//                         response[&owner_state_key] = vec![owner_state];
//                     }
//
//                     Some(response)
//                 }
//                 /// Sample transaction
//                 /// https://explorer.solana.com/tx/5Tmy1U59GFJD3sLe98WnKUksz3EHGmBqZKH53oBZhXdXEHcKu3hDhuFraJ43H8c1ezJDjuzZcu14EwcRJGEM4WSj
//                 LendingInstruction::InitReserve {
//                     liquidity_amount,
//                     config, // TODO: Index Reserve Config
//                 } => {
//                     let init_reserve_key =
//                         (SOLEND_TOKEN_LENDING_MARKET_RESERVE_TABLE.to_string(), *SOLEND_TOKEN_LENDING_MARKET_RESERVE_SCHEMA);
//                     let init_reserve = LendingMarketReserve {
//                         source_liquidity_account: instruction.accounts[0].account.to_string(),
//                         collateral_account: instruction.accounts[1].account.to_string(),
//                         collateral_mint: instruction.accounts[6].account.to_string(),
//                         account: instruction.accounts[2].account.to_string(),
//                         mint: instruction.accounts[3].account.to_string(),
//                         liquidity_account: instruction.accounts[3].account.to_string(),
//                         liquidity_fee_account: instruction.accounts[5].account.to_string(),
//                         collateral_token_supply: instruction.accounts[7].account.to_string(),
//                         oracle_account: instruction.accounts[8].account.to_string(),
//                         oracle_price_account: instruction.accounts[9].account.to_string(),
//                         lending_market_account: instruction.accounts[10].account.to_string(),
//                         lending_market_authority: instruction.accounts[11].account.to_string(),
//                         lending_market_owner: instruction.accounts[12].account.to_string(),
//                         user_transfer_authority: instruction.accounts[13].account.to_string(),
//                         token: instruction.accounts[16].account.to_string(),
//                         timestamp: instruction.timestamp.clone()
//                     };
//
//                     if response.contains(&init_reserve_key) {
//                         response[&init_reserve_key].push(init_reserve);
//                     } else {
//                         response[&init_reserve_key] = vec![init_reserve];
//                     }
//
//                     let reserve_liquidity_key =
//                         (SOLEND_TOKEN_LENDING_RESERVE_LIQUIDITY_TABLE.to_string(), *SOLEND_TOKEN_LENDING_RESERVE_LIQUIDITY_SCHEMA);
//                     let reserve_liquidity = ReserveLiquidity {
//                         source: instruction.accounts[13].account.to_string(),
//                         destination: instruction.accounts[4].account.to_string(),
//                         amount: liquidity_amount as i64,
//                         mint: instruction.accounts[3].account.to_string(),
//                         lending_market_account: instruction.accounts[10].account.to_string(),
//                         lending_market_authority: instruction.accounts[11].account.to_string(),
//                         timestamp: instruction.timestamp.clone()
//                     };
//
//                     if response.contains(&reserve_liquidity_key) {
//                         response[&reserve_liquidity_key].push(reserve_liquidity);
//                     } else {
//                         response[&reserve_liquidity_key] = vec![reserve_liquidity];
//                     }
//
//                     Some(response)
//                 }
//                 LendingInstruction::RefreshReserve => {
//                     None
//                 }
//                 LendingInstruction::DepositReserveLiquidity { liquidity_amount } => {
//                     let reserve_liquidity_key =
//                         (SOLEND_TOKEN_LENDING_RESERVE_LIQUIDITY_TABLE.to_string(), *SOLEND_TOKEN_LENDING_RESERVE_LIQUIDITY_SCHEMA);
//                     let reserve_liquidity = ReserveLiquidity {
//                         source: instruction.accounts[7].account.to_string(),
//                         destination: instruction.accounts[3].account.to_string(),
//                         amount: liquidity_amount as i64,
//                         mint: instruction.accounts[4].account.to_string(),
//                         lending_market_account: instruction.accounts[5].account.to_string(),
//                         lending_market_authority: instruction.accounts[6].account.to_string(),
//                         timestamp: instruction.timestamp.clone()
//                     };
//
//                     if response.contains(&reserve_liquidity_key) {
//                         response[&reserve_liquidity_key].push(reserve_liquidity);
//                     } else {
//                         response[&reserve_liquidity_key] = vec![reserve_liquidity];
//                     }
//
//                     Some(response)
//                 }
//                 LendingInstruction::RedeemReserveCollateral { collateral_amount } => {
//                     let reserve_liquidity_key =
//                         (SOLEND_TOKEN_LENDING_RESERVE_LIQUIDITY_TABLE.to_string(), *SOLEND_TOKEN_LENDING_RESERVE_LIQUIDITY_SCHEMA);
//                     let reserve_liquidity = ReserveLiquidity {
//                         source: instruction.accounts[4].account.to_string(),
//                         destination: instruction.accounts[7].account.to_string(),
//                         amount: -1 * (collateral_amount as i64),
//                         mint: instruction.accounts[3].account.to_string(),
//                         lending_market_account: instruction.accounts[5].account.to_string(),
//                         lending_market_authority: instruction.accounts[6].account.to_string(),
//                         timestamp: instruction.timestamp.clone()
//                     };
//
//                     if response.contains(&reserve_liquidity_key) {
//                         response[&reserve_liquidity_key].push(reserve_liquidity);
//                     } else {
//                         response[&reserve_liquidity_key] = vec![reserve_liquidity];
//                     }
//
//                     Some(response)
//                 }
//                 LendingInstruction::InitObligation => {
//                     let obligation_key =
//                         (SOLEND_TOKEN_LENDING_OBLIGATION_TABLE.to_string(), *SOLEND_TOKEN_LENDING_OBLIGATION_SCHEMA);
//                     let obligation = Obligation {
//                         obligation_type: ObligationType::Deposit as i16,
//                         source: instruction.accounts[0].account.to_string(),
//                         destination: instruction.accounts[2].account.to_string(),
//                         amount: 0,
//                         lending_market_account: instruction.accounts[1].account.to_string(),
//                         timestamp: instruction.timestamp.clone()
//                     };
//
//                     if response.contains(&obligation_key) {
//                         response[&obligation_key].push(obligation);
//                     } else {
//                         response[&obligation_key] = vec![obligation];
//                     }
//
//                     Some(response)
//                 }
//                 LendingInstruction::RefreshObligation => None,
//                 LendingInstruction::DepositObligationCollateral { collateral_amount } => {
//                     let obligation_key =
//                         (SOLEND_TOKEN_LENDING_OBLIGATION_TABLE.to_string(), *SOLEND_TOKEN_LENDING_OBLIGATION_SCHEMA);
//                     let obligation = Obligation {
//                         obligation_type: ObligationType::Deposit as i16,
//                         source: instruction.accounts[6].account.to_string(),
//                         destination: instruction.accounts[1].account.to_string(),
//                         amount: collateral_amount as i64,
//                         lending_market_account: instruction.accounts[4].account.to_string(),
//                         timestamp: instruction.timestamp.clone()
//                     };
//
//                     if response.contains(&obligation_key) {
//                         response[&obligation_key].push(obligation);
//                     } else {
//                         response[&obligation_key] = vec![obligation];
//                     }
//
//                     Some(response)
//                 }
//                 LendingInstruction::WithdrawObligationCollateral { collateral_amount } => {
//                     let obligation_key =
//                         (SOLEND_TOKEN_LENDING_OBLIGATION_TABLE.to_string(), *SOLEND_TOKEN_LENDING_OBLIGATION_SCHEMA);
//                     let obligation = Obligation {
//                         obligation_type: ObligationType::Withdraw as i16,
//                         source: instruction.accounts[0].account.to_string(),
//                         destination: instruction.accounts[6].account.to_string(),
//                         amount: -1 * (collateral_amount as i64),
//                         lending_market_account: instruction.accounts[4].account.to_string(),
//                         timestamp: instruction.timestamp.clone()
//                     };
//
//                     if response.contains(&obligation_key) {
//                         response[&obligation_key].push(obligation);
//                     } else {
//                         response[&obligation_key] = vec![obligation];
//                     }
//
//                     Some(response)
//                 }
//                 LendingInstruction::BorrowObligationLiquidity { liquidity_amount } => {
//                     let obligation_key =
//                         (SOLEND_TOKEN_LENDING_OBLIGATION_TABLE.to_string(), *SOLEND_TOKEN_LENDING_OBLIGATION_SCHEMA);
//                     let obligation = Obligation {
//                         obligation_type: ObligationType::Borrow as i16,
//                         source: instruction.accounts[0].account.to_string(),
//                         destination: instruction.accounts[7].account.to_string(),
//                         amount: -1 * (liquidity_amount as i64),
//                         lending_market_account: instruction.accounts[5].account.to_string(),
//                         timestamp: instruction.timestamp.clone()
//                     };
//
//                     if response.contains(&obligation_key) {
//                         response[&obligation_key].push(obligation);
//                     } else {
//                         response[&obligation_key] = vec![obligation];
//                     }
//
//                     Some(response)
//                 }
//                 LendingInstruction::RepayObligationLiquidity { liquidity_amount } => {
//                     let obligation_key =
//                         (SOLEND_TOKEN_LENDING_OBLIGATION_TABLE.to_string(), *SOLEND_TOKEN_LENDING_OBLIGATION_SCHEMA);
//                     let obligation = Obligation {
//                         obligation_type: ObligationType::Repay as i16,
//                         source: instruction.accounts[5].account.to_string(),
//                         destination: instruction.accounts[1].account.to_string(),
//                         amount: liquidity_amount as i64,
//                         lending_market_account: instruction.accounts[4].account.to_string(),
//                         timestamp: instruction.timestamp.clone()
//                     };
//
//                     if response.contains(&obligation_key) {
//                         response[&obligation_key].push(obligation);
//                     } else {
//                         response[&obligation_key] = vec![obligation];
//                     }
//
//                     Some(response)
//                 }
//                 LendingInstruction::LiquidateObligation { liquidity_amount } => {
//                     let obligation_key =
//                         (SOLEND_TOKEN_LENDING_OBLIGATION_TABLE.to_string(), *SOLEND_TOKEN_LENDING_OBLIGATION_SCHEMA);
//                     let obligation = Obligation {
//                         obligation_type: ObligationType::Liquidate as i16,
//                         source: instruction.accounts[6].account.to_string(),
//                         destination: instruction.accounts[3].account.to_string(),
//                         amount: liquidity_amount as i64,
//                         lending_market_account:instruction.accounts[7].account.to_string(),
//                         timestamp: instruction.timestamp.clone()
//                     };
//
//                     if response.contains(&obligation_key) {
//                         response[&obligation_key].push(obligation);
//                     } else {
//                         response[&obligation_key] = vec![obligation];
//                     }
//
//                     Some(response)
//                 }
//                 LendingInstruction::FlashLoan { amount } => {
//                     let obligation_key =
//                         (SOLEND_TOKEN_LENDING_OBLIGATION_TABLE.to_string(), *SOLEND_TOKEN_LENDING_OBLIGATION_SCHEMA);
//                     let obligation = Obligation {
//                         obligation_type: ObligationType::Liquidate as i16,
//                         source: instruction.accounts[0].account.to_string(),
//                         destination: instruction.accounts[1].account.to_string(),
//                         amount: -1 * (amount as i64),
//                         lending_market_account: instruction.accounts[5].account.to_string(),
//                         timestamp: instruction.timestamp.clone()
//                     };
//
//                     if response.contains(&obligation_key) {
//                         response[&obligation_key].push(obligation);
//                     } else {
//                         response[&obligation_key] = vec![obligation];
//                     }
//
//                     Some(response)
//                 }
//                 LendingInstruction::DepositReserveLiquidityAndObligationCollateral { .. } => {}
//                 LendingInstruction::WithdrawObligationCollateralAndRedeemReserveCollateral { .. } => {}
//                 LendingInstruction::UpdateReserveConfig { .. } => {}
//             }
//         }
//         Err(err) => {
//             let err_msg = match err {
//                 ProgramError::Custom(_) => "Custom".to_string(),
//                 ProgramError::InvalidArgument => "InvalidArgument".to_string(),
//                 ProgramError::InvalidInstructionData => "InvalidInstructionData".to_string(),
//                 ProgramError::InvalidAccountData => "InvalidAccountData".to_string(),
//                 ProgramError::AccountDataTooSmall => "AccountDataTooSmall".to_string(),
//                 ProgramError::InsufficientFunds => "InsufficientFunds".to_string(),
//                 ProgramError::IncorrectProgramId => "IncorrectProgramId".to_string(),
//                 ProgramError::MissingRequiredSignature => "MissingRequiredSignature".to_string(),
//                 ProgramError::AccountAlreadyInitialized => "AccountAlreadyInitialized".to_string(),
//                 ProgramError::UninitializedAccount => "UninitializedAccount".to_string(),
//                 ProgramError::NotEnoughAccountKeys => "NotEnoughAccountKeys".to_string(),
//                 ProgramError::AccountBorrowFailed => "AccountBorrowFailed".to_string(),
//                 ProgramError::MaxSeedLengthExceeded => "MaxSeedLengthExceeded".to_string(),
//                 ProgramError::InvalidSeeds => "InvalidSeeds".to_string(),
//                 ProgramError::BorshIoError(_) => "BorshIoError".to_string(),
//                 ProgramError::AccountNotRentExempt => "AccountNotRentExempt".to_string(),
//                 ProgramError::UnsupportedSysvar => "UnsupportedSysvar".to_string(),
//                 ProgramError::IllegalOwner => "IllegalOwner".to_string(),
//             };
//             error!("{} Reason: {}", "[processors/programs/solend_token_lending] FATAL: Unrecognised instruction.".to_string(),
//             err_msg);
//             None
//         }
//     };
// }