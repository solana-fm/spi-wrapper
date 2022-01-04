use avro_rs::schema::Schema;
use bincode::{deserialize};
use itertools::Itertools;
use serde::Serialize;
use std::collections::HashMap;
use solana_program::system_instruction::SystemInstruction;
use tracing::error;

use crate::{InstructionProperty, Instruction, InstructionSet, InstructionFunction};

pub const PROGRAM_ADDRESS: &str = "11111111111111111111111111111111";

pub const NATIVE_SYSTEM_ACCOUNT_CREATIONS_TABLE: &str = "native_system_account_creations";
pub const NATIVE_SYSTEM_ACCOUNT_ASSIGNMENTS_TABLE: &str = "native_system_account_assignments";
pub const NATIVE_SYSTEM_ACCOUNT_TRANSFERS_TABLE: &str = "native_system_account_transfers";
pub const NATIVE_SYSTEM_NONCE_ADVANCEMENTS_TABLE: &str = "native_system_nonce_consumptions";
pub const NATIVE_SYSTEM_NONCE_WITHDRAWALS_TABLE: &str = "native_system_nonce_withdrawals";
lazy_static! {
    pub static ref NATIVE_ACCOUNT_CREATION_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_account_creation",
        "fields": [
            {"name": "transaction_hash", "type": "string"},
            {"name": "account", "type": "string"},
            {"name": "amount", "type": "long"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref NATIVE_ACCOUNT_ASSIGNMENT_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_account_assignment",
        "fields": [
            {"name": "program", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref NATIVE_ACCOUNT_TRANSFER_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_account_transfer",
        "fields": [
            {"name": "source", "type": "string"},
            {"name": "destination", "type": "string"},
            {"name": "amount", "type": "long"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref NATIVE_SYSTEM_NONCE_ADVANCEMENT_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_nonce_consumption",
        "fields": [
            {"name": "nonce_account", "type": "string"},
            {"name": "nonce_authority", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref NATIVE_SYSTEM_NONCE_WITHDRAWAL_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_nonce_withdrawal",
        "fields": [
            {"name": "nonce_account", "type": "string"},
            {"name": "recipient", "type": "string"},
            {"name": "nonce_authority", "type": "string"},
            {"name": "amount", "type": "long"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
}

/// Records the state changes of the account at the time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCreation {
    /// Current lamport change in the account (+ve for deposit, -ve for withdraw)
    pub lamports: i64,
    /// The owner of the account.
    pub owner: String,
    /// Account state at the recorded timestamp.
    pub timestamp: i64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountAssignment {
    /// The account that is assigned to the program.
    pub account: String,
    /// The owner program of the account.
    pub program: String,
    /// Account state at the recorded timestamp.
    pub timestamp: i64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountTransfer {
    /// The source of the transfer
    pub source: String,
    /// The destination of the transfer
    pub destination: String,
    /// The amount of this transfer.
    pub amount: i64,
    /// Account state at the recorded timestamp.
    pub timestamp: i64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonceAdvancement {
    /// The nonce account involved
    pub nonce_account: String,
    /// The account approving this advancement.
    pub nonce_authority: String,
    /// The time this advancement was done.
    pub timestamp: i64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonceWithdrawal {
    /// The nonce account involved
    pub nonce_account: String,
    /// The account receiving the withdrawal amount.
    pub recipient: String,
    /// The account approving this advancement.
    pub nonce_authority: String,
    /// The amount withdrawn.
    pub amount: i64,
    /// The time this advancement was done.
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
    let sdr = deserialize::<SystemInstruction>(
        &instruction.data.as_slice());

    return match sdr {
        Ok(ref sir) => {
            let mut response: HashMap<(String, Schema), Vec<T>> = HashMap::new();
            let si = sir.clone();
            match si {
                SystemInstruction::CreateAccount {
                    lamports,
                    space,
                    owner,
                } => {
                    // check_num_system_accounts(&instruction.accounts, 2)?;
                    // Ok(ParsedInstructionEnum {
                    //     instruction_type: "createAccount".to_string(),
                    //     info: json!({
                    //         "source": account_keys[instruction.accounts[0] as usize].to_string(),
                    //         "newAccount": account_keys[instruction.accounts[1] as usize].to_string(),
                    //         "lamports": lamports,
                    //         "space": space,
                    //         "owner": owner.to_string(),
                    //     }),
                    // })
                    let key =
                        (NATIVE_SYSTEM_ACCOUNT_CREATIONS_TABLE.to_string(), *NATIVE_ACCOUNT_CREATION_SCHEMA);
                    let account_creation = AccountCreation {
                        lamports: lamports as i64,
                        owner: owner.to_string(),
                        timestamp: instruction.timestamp,
                    };

                    if response.contains(&key) {
                        response[&key].push(account_creation);
                    } else {
                        response[&key] = vec![account_creation];
                    }

                    Some(response)
                }
                SystemInstruction::Assign { owner } => {
                    // check_num_system_accounts(&instruction.accounts, 1)?;
                    // Ok(ParsedInstructionEnum {
                    //     instruction_type: "assign".to_string(),
                    //     info: json!({
                    //         "account": account_keys[instruction.accounts[0] as usize].to_string(),
                    //         "owner": owner.to_string(),
                    //     }),
                    // })
                    let key =
                        (NATIVE_SYSTEM_ACCOUNT_ASSIGNMENTS_TABLE.to_string(), *NATIVE_ACCOUNT_ASSIGNMENT_SCHEMA);
                    let account_assignment = AccountAssignment {
                        account: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 0)
                            .collect(),
                        program: owner.to_string(),
                        timestamp: instruction.timestamp,
                    };

                    if response.contains(&key) {
                        response[&key].push(account_assignment);
                    } else {
                        response[&key] = vec![account_assignment];
                    }

                    Some(response)
                }
                SystemInstruction::Transfer { lamports } => {
                    // check_num_system_accounts(&instruction.accounts, 2)?;
                    // Ok(ParsedInstructionEnum {
                    //     instruction_type: "transfer".to_string(),
                    //     info: json!({
                    //         "source": account_keys[instruction.accounts[0] as usize].to_string(),
                    //         "destination": account_keys[instruction.accounts[1] as usize].to_string(),
                    //         "lamports": lamports,
                    //     }),
                    // })
                    // check_num_system_accounts(&instruction.accounts, 2)?;
                    let key =
                        (NATIVE_SYSTEM_ACCOUNT_TRANSFERS_TABLE.to_string(), *NATIVE_ACCOUNT_TRANSFER_SCHEMA);
                    let account_assignment = AccountTransfer {
                        source: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 0)
                            .collect(),
                        destination: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 1)
                            .collect(),
                        amount: lamports as i64,
                        timestamp: instruction.timestamp,
                    };

                    if response.contains(&key) {
                        response[&key].push(account_assignment);
                    } else {
                        response[&key] = vec![account_assignment];
                    }

                    Some(response)
                }
                SystemInstruction::CreateAccountWithSeed {
                    base,
                    seed,
                    lamports,
                    space,
                    owner,
                } => {
                    // check_num_system_accounts(&instruction.accounts, 2)?;
                    // Ok(ParsedInstructionEnum {
                    //     instruction_type: "createAccountWithSeed".to_string(),
                    //     info: json!({
                    //         "source": account_keys[instruction.accounts[0] as usize].to_string(),
                    //         "newAccount": account_keys[instruction.accounts[1] as usize].to_string(),
                    //         "base": base.to_string(),
                    //         "seed": seed,
                    //         "lamports": lamports,
                    //         "space": space,
                    //         "owner": owner.to_string(),
                    //     }),
                    // })
                    let key =
                        (NATIVE_SYSTEM_ACCOUNT_CREATIONS_TABLE.to_string(), *NATIVE_ACCOUNT_CREATION_SCHEMA);
                    let account_creation = AccountCreation {
                        lamports: lamports as i64,
                        owner: owner.to_string(),
                        timestamp: instruction.timestamp,
                    };

                    if response.contains(&key) {
                        response[&key].push(account_creation);
                    } else {
                        response[&key] = vec![account_creation];
                    }

                    Some(response)
                }
                // TODO: Evaluate if we need this
                SystemInstruction::AdvanceNonceAccount => {
                    // check_num_system_accounts(&instruction.accounts, 3)?;
                    // Ok(ParsedInstructionEnum {
                    //     instruction_type: "advanceNonce".to_string(),
                    //     info: json!({
                    //         "nonceAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    //         "recentBlockhashesSysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    //         "nonceAuthority": account_keys[instruction.accounts[2] as usize].to_string(),
                    //     }),
                    // })
                    let key =
                        (NATIVE_SYSTEM_NONCE_ADVANCEMENTS_TABLE.to_string(), *NATIVE_SYSTEM_NONCE_ADVANCEMENT_SCHEMA);
                    let nonce_advancement = NonceAdvancement {
                        nonce_account: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 0)
                            .collect(),
                        nonce_authority: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 2)
                            .collect(),
                        timestamp: instruction.timestamp,
                    };

                    if response.contains(&key) {
                        response[&key].push(nonce_advancement);
                    } else {
                        response[&key] = vec![nonce_advancement];
                    }

                    // Some(response)
                    None
                }
                // TODO: Evaluate if we need this
                SystemInstruction::WithdrawNonceAccount(lamports) => {
                    // check_num_system_accounts(&instruction.accounts, 5)?;
                    // Ok(ParsedInstructionEnum {
                    //     instruction_type: "withdrawFromNonce".to_string(),
                    //     info: json!({
                    //         "nonceAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    //         "destination": account_keys[instruction.accounts[1] as usize].to_string(),
                    //         "recentBlockhashesSysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                    //         "rentSysvar": account_keys[instruction.accounts[3] as usize].to_string(),
                    //         "nonceAuthority": account_keys[instruction.accounts[4] as usize].to_string(),
                    //         "lamports": lamports,
                    //     }),
                    // })
                    let key =
                        (NATIVE_SYSTEM_NONCE_WITHDRAWALS_TABLE.to_string(), *NATIVE_SYSTEM_NONCE_WITHDRAWAL_SCHEMA);
                    let nonce_withdrawal = NonceWithdrawal {
                        nonce_account: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 0)
                            .collect(),
                        recipient: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 1)
                            .collect(),
                        nonce_authority: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 4)
                            .collect(),
                        amount: lamports as i64,
                        timestamp: instruction.timestamp,
                    };

                    if response.contains(&key) {
                        response[&key].push(nonce_withdrawal);
                    } else {
                        response[&key] = vec![nonce_withdrawal];
                    }

                    // Some(response)
                    None
                }
                // TODO: Evaluate if we need this
                SystemInstruction::InitializeNonceAccount(_) => {
                    // check_num_system_accounts(&instruction.accounts, 3)?;
                    // Ok(ParsedInstructionEnum {
                    //     instruction_type: "initializeNonce".to_string(),
                    //     info: json!({
                    //         "nonceAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    //         "recentBlockhashesSysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    //         "rentSysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                    //         "nonceAuthority": authority.to_string(),
                    //     }),
                    // })
                    None
                }
                // TODO: Evaluate if we need this
                SystemInstruction::AuthorizeNonceAccount(_) => {
                    // check_num_system_accounts(&instruction.accounts, 1)?;
                    // Ok(ParsedInstructionEnum {
                    //     instruction_type: "authorizeNonce".to_string(),
                    //     info: json!({
                    //         "nonceAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                    //         "nonceAuthority": account_keys[instruction.accounts[1] as usize].to_string(),
                    //         "newAuthorized": authority.to_string(),
                    //     }),
                    // })
                    None
                }
                SystemInstruction::Allocate { .. } => {
                    // check_num_system_accounts(&instruction.accounts, 1)?;
                    // Ok(ParsedInstructionEnum {
                    //     instruction_type: "allocate".to_string(),
                    //     info: json!({
                    //         "account": account_keys[instruction.accounts[0] as usize].to_string(),
                    //         "space": space,
                    //     }),
                    // })
                    None
                }
                SystemInstruction::AllocateWithSeed {
                    base,
                    seed,
                    space,
                    owner,
                } => {
                    // check_num_system_accounts(&instruction.accounts, 2)?;
                    // Ok(ParsedInstructionEnum {
                    //     instruction_type: "allocateWithSeed".to_string(),
                    //     info: json!({
                    //         "account": account_keys[instruction.accounts[0] as usize].to_string(),
                    //         "base": base.to_string(),
                    //         "seed": seed,
                    //         "space": space,
                    //         "owner": owner.to_string(),
                    //     }),
                    // })
                    None
                }
                SystemInstruction::AssignWithSeed { base, seed, owner } => {
                    // check_num_system_accounts(&instruction.accounts, 2)?;
                    // Ok(ParsedInstructionEnum {
                    //     instruction_type: "assignWithSeed".to_string(),
                    //     info: json!({
                    //         "account": account_keys[instruction.accounts[0] as usize].to_string(),
                    //         "base": base.to_string(),
                    //         "seed": seed,
                    //         "owner": owner.to_string(),
                    //     }),
                    // })
                    let key =
                        (NATIVE_SYSTEM_ACCOUNT_ASSIGNMENTS_TABLE.to_string(), *NATIVE_ACCOUNT_ASSIGNMENT_SCHEMA);
                    let account_assignment = AccountAssignment {
                        account: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 0)
                            .collect(),
                        program: owner.to_string(),
                        timestamp: instruction.timestamp,
                    };

                    if response.contains(&key) {
                        response[&key].push(account_assignment);
                    } else {
                        response[&key] = vec![account_assignment];
                    }

                    Some(response)
                }
                SystemInstruction::TransferWithSeed {
                    lamports,
                    from_seed,
                    from_owner,
                } => {
                    // check_num_system_accounts(&instruction.accounts, 3)?;
                    // Ok(ParsedInstructionEnum {
                    //     instruction_type: "transferWithSeed".to_string(),
                    //     info: json!({
                    //         "source": account_keys[instruction.accounts[0] as usize].to_string(),
                    //         "sourceBase": account_keys[instruction.accounts[1] as usize].to_string(),
                    //         "destination": account_keys[instruction.accounts[2] as usize].to_string(),
                    //         "lamports": lamports,
                    //         "sourceSeed": from_seed,
                    //         "sourceOwner": from_owner.to_string(),
                    //     }),
                    // })
                    let key =
                        (NATIVE_SYSTEM_ACCOUNT_TRANSFERS_TABLE.to_string(), *NATIVE_ACCOUNT_TRANSFER_SCHEMA);
                    let account_assignment = AccountTransfer {
                        source: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 0)
                            .collect(),
                        destination: instruction.account_instructions.into_iter()
                            .filter(|ai| ai.index == 2)
                            .collect(),
                        amount: lamports as i64,
                        timestamp: instruction.timestamp,
                    };

                    if response.contains(&key) {
                        response[&key].push(account_assignment);
                    } else {
                        response[&key] = vec![account_assignment];
                    }

                    Some(response)
                }
            }
        }
        Err(_) => { // Error provided has no utility at the moment.
            error!("{}", "[spi-wrapper/programs/native_system] Error deserializing this system \
        instruction!".to_string());

            None
        }
    }
}