use std::collections::HashMap;
use avro_rs::Schema;
use itertools::Itertools;
use serde::Serialize;
use solana_program::program_error::ProgramError;
use spl_token::instruction::TokenInstruction;
use spl_token::solana_program::program_option::COption;
use tracing::error;

use crate::{Instruction, InstructionFunction, InstructionProperty, InstructionSet};

pub const PROGRAM_ADDRESS: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

pub const NATIVE_TOKEN_MINT_STATE_TABLE_NAME: &str = "native_token_mint_states";
pub const NATIVE_TOKEN_MINT_INFLATION_TABLE_NAME: &str = "native_token_mint_inflations";
pub const NATIVE_TOKEN_MINT_MOVEMENT_TABLE_NAME: &str = "native_token_mint_movements";
pub const NATIVE_TOKEN_MINT_DELEGATION_TABLE_NAME: &str = "native_token_mint_delegations";
lazy_static! {
    pub static ref NATIVE_TOKEN_MINT_STATE_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_token_mint_state",
        "fields": [
            {"name": "decimals", "type": "int"},
            {"name": "mint_authority", "type": "string"},
            {"name": "freeze_authority", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref NATIVE_TOKEN_MINT_INFLATION_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_token_mint_inflation",
        "fields": [
            {"name": "account", "type": "string"},
            {"name": "mint", "type": "string"},
            {"name": "amount", "type": "long"},
            {"name": "decimals", "type": ["null", "int"]},
            {"name": "owner", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref NATIVE_TOKEN_MINT_MOVEMENT_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_token_mint_movement",
        "fields": [
            {"name": "source", "type": "string"},
            {"name": "destination", "type": "string"},
            {"name": "mint", "type": ["null", "string"]},
            {"name": "amount", "type": "long"},
            {"name": "decimals", "type": ["null", "int"]},
            {"name": "owner", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref NATIVE_TOKEN_MINT_DELEGATION_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_token_mint_delegation",
        "fields": [
            {"name": "source", "type": "string"},
            {"name": "delegate", "type": "string"},
            {"name": "mint", "type": ["null", "string"]},
            {"name": "owner", "type": "string"},
            {"name": "amount", "type": "long"},
            {"name": "decimals", "type": ["null", "int"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
}

#[derive(Serialize)]
pub struct MintState {
    pub decimals: i16,
    pub mint_authority: String,
    pub freeze_authority: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct MintInflation {
    pub account: String,
    pub mint: String,
    pub amount: i64,
    pub decimals: Option<i16>,
    pub owner: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct MintMovement {
    pub source: String,
    pub destination: String,
    pub mint: Option<String>,
    pub amount: i64,
    pub decimals: Option<i16>,
    pub owner: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct MintDelegation {
    pub source: String,
    pub delegate: String,
    pub mint: Option<String>,
    pub owner: String,
    pub amount: i64,
    pub decimals: Option<i16>,
    pub timestamp: i64
}

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction<T: Serialize>(
    // The instruction
    instruction: Instruction,
) -> Option<HashMap<(String, Schema), Vec<T>>> {
    // We don't have anything to work with
    let tdr = TokenInstruction::unpack(instruction.data.as_slice());

    return match tdr {
        Ok(ref tir) => {
            let mut response: HashMap<(String, Schema), Vec<T>> = HashMap::new();
            let dti = tir.clone();
            match dti {
                TokenInstruction::InitializeMint {
                    decimals,
                    mint_authority,
                    freeze_authority,
                } => {
                    // Source code
                    // msg!("Instruction: InitializeMint");
                    // Self::process_initialize_mint(
                    //     accounts,
                    //     decimals,
                    //     mint_authority,
                    //     freeze_authority,
                    // )
                    let key =
                        (NATIVE_TOKEN_MINT_STATE_TABLE_NAME.to_string(), *NATIVE_TOKEN_MINT_STATE_SCHEMA);
                    let mint_state = MintState {
                        decimals: decimals as i16,
                        mint_authority: mint_authority.to_string(),
                        freeze_authority: freeze_authority.to_string(),
                        timestamp: instruction.timestamp
                    };

                    if response.contains(&key) {
                        response[&key].push(mint_state);
                    } else {
                        response[&key] = vec![mint_state];
                    }

                    Some(response)
                }
                TokenInstruction::InitializeAccount => {
                    // msg!("Instruction: InitializeAccount");
                    // Self::process_initialize_account(accounts)
                    // Some(InstructionSet {
                    //     function: InstructionFunction {
                    //         tx_instruction_id: instruction.tx_instruction_id.clone(),
                    //         transaction_hash: instruction.transaction_hash.clone(),
                    //         parent_index: instruction.parent_index.clone(),
                    //         program: instruction.program.clone(),
                    //         function_name: "initialize-account".to_string(),
                    //         timestamp: instruction.timestamp.clone()
                    //     },
                    //     properties: vec![]
                    // })
                    None
                }
                TokenInstruction::InitializeAccount2 { owner } => {
                    // msg!("Instruction: InitializeAccount2");
                    // Self::process_initialize_account2(accounts, owner)
                    // Some(InstructionSet {
                    //     function: InstructionFunction {
                    //         tx_instruction_id: instruction.tx_instruction_id.clone(),
                    //         transaction_hash: instruction.transaction_hash.clone(),
                    //         parent_index: instruction.parent_index.clone(),
                    //         program: instruction.program.clone(),
                    //         function_name: "initialize-account-2".to_string(),
                    //         timestamp: instruction.timestamp.clone()
                    //     },
                    //     properties: vec![
                    //         InstructionProperty {
                    //             tx_instruction_id: instruction.tx_instruction_id.clone(),
                    //             transaction_hash: instruction.transaction_hash.clone(),
                    //             parent_index: instruction.parent_index.clone(),
                    //             key: "owner".to_string(),
                    //             value: owner.to_string(),
                    //             parent_key: "".to_string(),
                    //             timestamp: instruction.timestamp.clone(),
                    //         }
                    //     ]
                    // })
                    None
                }
                TokenInstruction::InitializeMultisig { m } => {
                    // msg!("Instruction: InitializeMultisig");
                    // Self::process_initialize_multisig(accounts, m)
                    // Some(InstructionSet {
                    //     function: InstructionFunction {
                    //         tx_instruction_id: instruction.tx_instruction_id.clone(),
                    //         transaction_hash: instruction.transaction_hash.clone(),
                    //         parent_index: instruction.parent_index.clone(),
                    //         program: instruction.program.clone(),
                    //         function_name: "initialize-multisig".to_string(),
                    //         timestamp: instruction.timestamp.clone()
                    //     },
                    //     properties: vec![
                    //         InstructionProperty {
                    //             tx_instruction_id: instruction.tx_instruction_id.clone(),
                    //             transaction_hash: instruction.transaction_hash.clone(),
                    //             parent_index: instruction.parent_index.clone(),
                    //             key: "m".to_string(),
                    //             value: m.to_string(),
                    //             parent_key: "".to_string(),
                    //             timestamp: instruction.timestamp.clone(),
                    //         }
                    //     ]
                    // })
                    None
                }
                TokenInstruction::Transfer { amount } => {
                    // msg!("Instruction: Transfer");
                    // Self::process_transfer(program_id, accounts, amount, None)
                    // Some(InstructionSet {
                    //     function: InstructionFunction {
                    //         tx_instruction_id: instruction.tx_instruction_id.clone(),
                    //         transaction_hash: instruction.transaction_hash.clone(),
                    //         parent_index: instruction.parent_index.clone(),
                    //         program: instruction.program.clone(),
                    //         function_name: "transfer".to_string(),
                    //         timestamp: instruction.timestamp.clone()
                    //     },
                    //     properties: vec![
                    //         InstructionProperty {
                    //             tx_instruction_id: instruction.tx_instruction_id.clone(),
                    //             transaction_hash: instruction.transaction_hash.clone(),
                    //             parent_index: instruction.parent_index.clone(),
                    //             key: "amount".to_string(),
                    //             value: amount.to_string(),
                    //             parent_key: "".to_string(),
                    //             timestamp: instruction.timestamp.clone(),
                    //         }
                    //     ]
                    // })
                    let key =
                        (NATIVE_TOKEN_MINT_MOVEMENT_TABLE_NAME.to_string(), *NATIVE_TOKEN_MINT_MOVEMENT_SCHEMA);
                    let mint_burn = MintMovement {
                        destination: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 0)
                            .collect(),
                        source: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 1)
                            .collect(),
                        mint: None,
                        amount: amount as i64,
                        decimals: None,
                        owner: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 2)
                            .collect(),
                        timestamp: instruction.timestamp
                    };

                    if response.contains(&key) {
                        response[&key].push(mint_burn);
                    } else {
                        response[&key] = vec![mint_burn];
                    }

                    Some(response)
                }
                TokenInstruction::Approve { amount } => {
                    // msg!("Instruction: Approve");
                    // Self::process_approve(program_id, accounts, amount, None)
                    let key =
                        (NATIVE_TOKEN_MINT_DELEGATION_TABLE_NAME.to_string(), *NATIVE_TOKEN_MINT_DELEGATION_SCHEMA);
                    let mint_burn = MintDelegation {
                        delegate: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 2)
                            .collect(),
                        source: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 0)
                            .collect(),
                        mint: None,
                        amount: amount as i64,
                        decimals: None,
                        owner: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 2)
                            .collect(),
                        timestamp: instruction.timestamp
                    };

                    if response.contains(&key) {
                        response[&key].push(mint_burn);
                    } else {
                        response[&key] = vec![mint_burn];
                    }

                    Some(response)
                }
                TokenInstruction::Revoke => {
                    // msg!("Instruction: Revoke");
                    // Self::process_revoke(program_id, accounts)
                    None
                }
                // TODO: Do we need this?
                TokenInstruction::SetAuthority {
                    authority_type,
                    new_authority,
                } => {
                    // msg!("Instruction: SetAuthority");
                    // Self::process_set_authority(
                    //     program_id,
                    //     accounts,
                    //     authority_type,
                    //     new_authority,
                    // )
                    // Some(InstructionSet {
                    //     function: InstructionFunction {
                    //         tx_instruction_id: instruction.tx_instruction_id.clone(),
                    //         transaction_hash: instruction.transaction_hash.clone(),
                    //         parent_index: instruction.parent_index.clone(),
                    //         program: instruction.program.clone(),
                    //         function_name: "set-authority".to_string(),
                    //         timestamp: instruction.timestamp.clone()
                    //     },
                    //     properties: vec![
                    //         InstructionProperty {
                    //             tx_instruction_id: instruction.tx_instruction_id.clone(),
                    //             transaction_hash: instruction.transaction_hash.clone(),
                    //             parent_index: instruction.parent_index.clone(),
                    //             key: "authority_type".to_string(),
                    //             value: (authority_type as u8).to_string(),
                    //             parent_key: "".to_string(),
                    //             timestamp: instruction.timestamp.clone(),
                    //         },
                    //         InstructionProperty {
                    //             tx_instruction_id: instruction.tx_instruction_id.clone(),
                    //             transaction_hash: instruction.transaction_hash.clone(),
                    //             parent_index: instruction.parent_index.clone(),
                    //             key: "new_authority".to_string(),
                    //             value: if let COption::Some(na) = new_authority {
                    //                 na.to_string()
                    //             } else {
                    //                 "".to_string()
                    //             },
                    //             parent_key: "".to_string(),
                    //             timestamp: instruction.timestamp.clone(),
                    //         }
                    //     ]
                    // })
                    None
                }
                TokenInstruction::MintTo { amount } => {
                    // msg!("Instruction: MintTo");
                    // Self::process_mint_to(program_id, accounts, amount, None)
                    // msg!("Instruction: Burn");
                    // Self::process_burn(program_id, accounts, amount, None)
                    let key =
                        (NATIVE_TOKEN_MINT_INFLATION_TABLE_NAME.to_string(), *NATIVE_TOKEN_MINT_BURN_SCHEMA);
                    let mint_to = MintInflation {
                        account: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 1)
                            .collect(),
                        mint: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 0)
                            .collect(),
                        amount: amount as i64,
                        decimals: None,
                        owner: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 2)
                            .collect(),
                        timestamp: instruction.timestamp
                    };

                    if response.contains(&key) {
                        response[&key].push(mint_to);
                    } else {
                        response[&key] = vec![mint_to];
                    }

                    Some(response)
                }
                TokenInstruction::Burn { amount } => {
                    // msg!("Instruction: Burn");
                    // Self::process_burn(program_id, accounts, amount, None)
                    let key =
                        (NATIVE_TOKEN_MINT_INFLATION_TABLE_NAME.to_string(), *NATIVE_TOKEN_MINT_BURN_SCHEMA);
                    let mint_burn = MintInflation {
                        account: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 0)
                            .collect(),
                        mint: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 1)
                            .collect(),
                        amount: -1 * (amount as i64),
                        decimals: None,
                        owner: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 2)
                            .collect(),
                        timestamp: instruction.timestamp
                    };

                    if response.contains(&key) {
                        response[&key].push(mint_burn);
                    } else {
                        response[&key] = vec![mint_burn];
                    }

                    Some(response)
                }
                TokenInstruction::CloseAccount => {
                    // msg!("Instruction: CloseAccount");
                    // Self::process_close_account(program_id, accounts)
                    None
                }
                // TODO: Do we need this?
                TokenInstruction::FreezeAccount => {
                    // msg!("Instruction: FreezeAccount");
                    // Self::process_toggle_freeze_account(program_id, accounts, true)
                    // Some(InstructionSet {
                    //     function: InstructionFunction {
                    //         tx_instruction_id: instruction.tx_instruction_id.clone(),
                    //         transaction_hash: instruction.transaction_hash.clone(),
                    //         parent_index: instruction.parent_index.clone(),
                    //         program: instruction.program.clone(),
                    //         function_name: "freeze-account".to_string(),
                    //         timestamp: instruction.timestamp.clone()
                    //     },
                    //     properties: vec![]
                    // })
                    None
                }
                // TODO: Do we need this?
                TokenInstruction::ThawAccount => {
                    // msg!("Instruction: ThawAccount");
                    // Self::process_toggle_freeze_account(program_id, accounts, false)
                    // Some(InstructionSet {
                    //     function: InstructionFunction {
                    //         tx_instruction_id: instruction.tx_instruction_id.clone(),
                    //         transaction_hash: instruction.transaction_hash.clone(),
                    //         parent_index: instruction.parent_index.clone(),
                    //         program: instruction.program.clone(),
                    //         function_name: "thaw-account".to_string(),
                    //         timestamp: instruction.timestamp.clone()
                    //     },
                    //     properties: vec![]
                    // })
                    None
                }
                TokenInstruction::TransferChecked { amount, decimals } => {
                    // msg!("Instruction: TransferChecked");
                    // Self::process_transfer(program_id, accounts, amount, Some(decimals))
                    let key =
                        (NATIVE_TOKEN_MINT_MOVEMENT_TABLE_NAME.to_string(), *NATIVE_TOKEN_MINT_MOVEMENT_SCHEMA);
                    let mint_movement = MintMovement {
                        destination: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 2)
                            .collect(),
                        source: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 0)
                            .collect(),
                        mint: Some(instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 1)
                            .collect()),
                        amount: amount as i64,
                        decimals: Some(decimals as i16),
                        owner: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 3)
                            .collect(),
                        timestamp: instruction.timestamp
                    };

                    if response.contains(&key) {
                        response[&key].push(mint_movement);
                    } else {
                        response[&key] = vec![mint_movement];
                    }

                    Some(response)
                }
                TokenInstruction::ApproveChecked { amount, decimals } => {
                    // msg!("Instruction: ApproveChecked");
                    // Self::process_approve(program_id, accounts, amount, Some(decimals))
                    let key =
                        (NATIVE_TOKEN_MINT_DELEGATION_TABLE_NAME.to_string(), *NATIVE_TOKEN_MINT_DELEGATION_SCHEMA);
                    let mint_delegation = MintDelegation {
                        delegate: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 2)
                            .collect(),
                        source: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 0)
                            .collect(),
                        mint: Some(instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 1)
                            .collect()),
                        amount: amount as i64,
                        decimals: Some(decimals as i16),
                        owner: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 3)
                            .collect(),
                        timestamp: instruction.timestamp
                    };

                    if response.contains(&key) {
                        response[&key].push(mint_delegation);
                    } else {
                        response[&key] = vec![mint_delegation];
                    }

                    Some(response)
                }
                TokenInstruction::MintToChecked { amount, decimals } => {
                    // msg!("Instruction: MintToChecked");
                    // Self::process_mint_to(program_id, accounts, amount, Some(decimals))
                    let key =
                        (NATIVE_TOKEN_MINT_INFLATION_TABLE_NAME.to_string(), *NATIVE_TOKEN_MINT_BURN_SCHEMA);
                    let mint_to = MintInflation {
                        account: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 1)
                            .collect(),
                        mint: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 0)
                            .collect(),
                        amount: amount as i64,
                        decimals: Some(decimals as i16),
                        owner: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 2)
                            .collect(),
                        timestamp: instruction.timestamp
                    };

                    if response.contains(&key) {
                        response[&key].push(mint_to);
                    } else {
                        response[&key] = vec![mint_to];
                    }

                    Some(response)
                }
                TokenInstruction::BurnChecked { amount, decimals } => {
                    // msg!("Instruction: BurnChecked");
                    // Self::process_burn(program_id, accounts, amount, Some(decimals))
                    let key =
                        (NATIVE_TOKEN_MINT_INFLATION_TABLE_NAME.to_string(), *NATIVE_TOKEN_MINT_INFLATION_SCHEMA);
                    let mint_burn = MintInflation {
                        account: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 0)
                            .collect(),
                        mint: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 1)
                            .collect(),
                        amount: -1 * (amount as i64),
                        decimals: Some(decimals as i16),
                        owner: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 2)
                            .collect(),
                        timestamp: instruction.timestamp
                    };

                    if response.contains(&key) {
                        response[&key].push(mint_burn);
                    } else {
                        response[&key] = vec![mint_burn];
                    }

                    Some(response)
                }
                TokenInstruction::SyncNative => {
                    // msg!("Instruction: SyncNative");
                    None
                }
            }
        }
        Err(err) => {
            let err_msg = match err {
                ProgramError::Custom(_) => "Custom".to_string(),
                ProgramError::InvalidArgument => "InvalidArgument".to_string(),
                ProgramError::InvalidInstructionData => "InvalidInstructionData".to_string(),
                ProgramError::InvalidAccountData => "InvalidAccountData".to_string(),
                ProgramError::AccountDataTooSmall => "AccountDataTooSmall".to_string(),
                ProgramError::InsufficientFunds => "InsufficientFunds".to_string(),
                ProgramError::IncorrectProgramId => "IncorrectProgramId".to_string(),
                ProgramError::MissingRequiredSignature => "MissingRequiredSignature".to_string(),
                ProgramError::AccountAlreadyInitialized => "AccountAlreadyInitialized".to_string(),
                ProgramError::UninitializedAccount => "UninitializedAccount".to_string(),
                ProgramError::NotEnoughAccountKeys => "NotEnoughAccountKeys".to_string(),
                ProgramError::AccountBorrowFailed => "AccountBorrowFailed".to_string(),
                ProgramError::MaxSeedLengthExceeded => "MaxSeedLengthExceeded".to_string(),
                ProgramError::InvalidSeeds => "InvalidSeeds".to_string(),
                ProgramError::BorshIoError(_) => "BorshIoError".to_string(),
                ProgramError::AccountNotRentExempt => "AccountNotRentExempt".to_string(),
                ProgramError::UnsupportedSysvar => "UnsupportedSysvar".to_string(),
                ProgramError::IllegalOwner => "IllegalOwner".to_string()
            };

            error!("{} Reason: {}", "Invalid instruction for the token program.".to_string(), err_msg);
            None
        }
    }
}