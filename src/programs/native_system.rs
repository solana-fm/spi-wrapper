use avro_rs::schema::Schema;
use bincode::{deserialize};
use serde::Serialize;
use solana_program::system_instruction::SystemInstruction;
use tracing::error;

use crate::{Instruction, TableData, TypedDatum};

pub const PROGRAM_ADDRESS: &str = "11111111111111111111111111111111";

pub const NATIVE_SYSTEM_ACCOUNT_CREATIONS_TABLE: &str = "native_system_account_creations";
pub const NATIVE_SYSTEM_ACCOUNT_ASSIGNMENTS_TABLE: &str = "native_system_account_assignments";
pub const NATIVE_SYSTEM_ACCOUNT_TRANSFERS_TABLE: &str = "native_system_account_transfers";
pub const NATIVE_SYSTEM_NONCE_ADVANCEMENTS_TABLE: &str = "native_system_nonce_advancements";
pub const NATIVE_SYSTEM_NONCE_WITHDRAWALS_TABLE: &str = "native_system_nonce_withdrawals";
lazy_static! {
    pub static ref NATIVE_SYSTEM_ACCOUNT_CREATION_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_account_creation",
        "fields": [
            {"name": "address", "type": "string"},
            {"name": "lamports", "type": "long"},
            {"name": "owner", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref NATIVE_SYSTEM_ACCOUNT_ASSIGNMENT_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_account_assignment",
        "fields": [
            {"name": "account", "type": "string"},
            {"name": "program", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref NATIVE_SYSTEM_ACCOUNT_TRANSFER_SCHEMA: Schema = Schema::parse_str(
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
        "name": "native_nonce_advancement",
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

#[derive(Serialize)]
pub enum NativeSystemDatum {
    AccountCreation(AccountCreation),
    AccountAssignment(AccountAssignment),
    AccountTransfer(AccountTransfer),
    NonceAdvancement(NonceAdvancement),
    NonceWithdrawal(NonceWithdrawal),
}

/// Records the state changes of the account at the time.
#[derive(Serialize)]
pub struct AccountCreation {
    /// The new account's address
    pub address: String,
    /// Current lamport change in the account (+ve for deposit, -ve for withdraw)
    pub lamports: i64,
    /// The owner of the account.
    pub owner: String,
    /// Account state at the recorded timestamp.
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct AccountAssignment {
    /// The account that is assigned to the program.
    pub account: String,
    /// The owner program of the account.
    pub program: String,
    /// Account state at the recorded timestamp.
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct AccountTransfer {
    /// The source of the transfer
    pub source: String,
    /// The destination of the transfer
    pub destination: String,
    /// The amount of this transfer.
    pub amount: i64,
    /// Account state at the recorded timestamp.
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct NonceAdvancement {
    /// The nonce account involved
    pub nonce_account: String,
    /// The account approving this advancement.
    pub nonce_authority: String,
    /// The time this advancement was done.
    pub timestamp: i64,
}

#[derive(Serialize)]
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
    let sdr = serde_json::de::from_slice::<SystemInstruction>(
        &instruction.data.as_slice());

    return match sdr {
        Ok(ref sir) => {
            let mut response: Vec<TableData> = Vec::new();
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
                    let table_data = TableData {
                        schema: (*NATIVE_SYSTEM_ACCOUNT_CREATION_SCHEMA).clone(),
                        table_name: NATIVE_SYSTEM_ACCOUNT_CREATIONS_TABLE.to_string(),
                        data: vec![TypedDatum::NativeSystem(
                            NativeSystemDatum::AccountCreation(
                                AccountCreation {
                                    address: instruction.accounts[1].account.to_string(),
                                    lamports: lamports as i64,
                                    owner: owner.to_string(),
                                    timestamp: instruction.timestamp,
                                }
                            )
                        )],
                    };

                    response.push(table_data);

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
                    let table_data = TableData {
                        schema: (*NATIVE_SYSTEM_ACCOUNT_ASSIGNMENT_SCHEMA).clone(),
                        table_name: NATIVE_SYSTEM_ACCOUNT_ASSIGNMENTS_TABLE.to_string(),
                        data: vec![TypedDatum::NativeSystem(
                            NativeSystemDatum::AccountAssignment(
                                AccountAssignment {
                                    account: instruction.accounts[0].account.to_string(),
                                    program: owner.to_string(),
                                    timestamp: instruction.timestamp,
                                }
                            )
                        )],
                    };

                    response.push(table_data);

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
                    let table_data = TableData {
                        schema: (*NATIVE_SYSTEM_ACCOUNT_TRANSFER_SCHEMA).clone(),
                        table_name: NATIVE_SYSTEM_ACCOUNT_TRANSFERS_TABLE.to_string(),
                        data: vec![TypedDatum::NativeSystem(
                            NativeSystemDatum::AccountTransfer(
                                AccountTransfer {
                                    source: instruction.accounts[0].account.to_string(),
                                    destination: instruction.accounts[1].account.to_string(),
                                    amount: lamports as i64,
                                    timestamp: instruction.timestamp,
                                }
                            )
                        )],
                    };

                    response.push(table_data);

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
                    let table_data = TableData {
                        schema: (*NATIVE_SYSTEM_ACCOUNT_CREATION_SCHEMA).clone(),
                        table_name: NATIVE_SYSTEM_ACCOUNT_CREATIONS_TABLE.to_string(),
                        data: vec![TypedDatum::NativeSystem(
                            NativeSystemDatum::AccountCreation(
                                AccountCreation {
                                    address: instruction.accounts[1].account.to_string(),
                                    lamports: lamports as i64,
                                    owner: owner.to_string(),
                                    timestamp: instruction.timestamp,
                                }
                            )
                        )],
                    };

                    response.push(table_data);

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
                    let table_data = TableData {
                        schema: (*NATIVE_SYSTEM_NONCE_ADVANCEMENT_SCHEMA).clone(),
                        table_name: NATIVE_SYSTEM_NONCE_ADVANCEMENTS_TABLE.to_string(),
                        data: vec![TypedDatum::NativeSystem(
                            NativeSystemDatum::NonceAdvancement(
                                NonceAdvancement {
                                    nonce_account: instruction.accounts[0].account.to_string(),
                                    nonce_authority: instruction.accounts[2].account.to_string(),
                                    timestamp: instruction.timestamp,
                                }
                            )
                        )],
                    };

                    response.push(table_data);

                    Some(response)
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
                    let table_data = TableData {
                        schema: (*NATIVE_SYSTEM_NONCE_WITHDRAWAL_SCHEMA).clone(),
                        table_name: NATIVE_SYSTEM_NONCE_WITHDRAWALS_TABLE.to_string(),
                        data: vec![TypedDatum::NativeSystem(
                            NativeSystemDatum::NonceWithdrawal(
                                NonceWithdrawal {
                                    nonce_account: instruction.accounts[0].account.to_string(),
                                    recipient: instruction.accounts[1].account.to_string(),
                                    nonce_authority: instruction.accounts[4].account.to_string(),
                                    amount: lamports as i64,
                                    timestamp: instruction.timestamp,
                                }
                            )
                        )],
                    };

                    response.push(table_data);

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
                    let table_data = TableData {
                        schema: (*NATIVE_SYSTEM_ACCOUNT_ASSIGNMENT_SCHEMA).clone(),
                        table_name: NATIVE_SYSTEM_ACCOUNT_ASSIGNMENTS_TABLE.to_string(),
                        data: vec![TypedDatum::NativeSystem(
                            NativeSystemDatum::AccountAssignment(
                                AccountAssignment {
                                    account: instruction.accounts[0].account.to_string(),
                                    program: owner.to_string(),
                                    timestamp: instruction.timestamp,
                                }
                            )
                        )],
                    };

                    response.push(table_data);

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
                    let table_data = TableData {
                        schema: (*NATIVE_SYSTEM_ACCOUNT_TRANSFER_SCHEMA).clone(),
                        table_name: NATIVE_SYSTEM_ACCOUNT_TRANSFERS_TABLE.to_string(),
                        data: vec![TypedDatum::NativeSystem(
                            NativeSystemDatum::AccountTransfer(
                                AccountTransfer {
                                    source: instruction.accounts[0].account.to_string(),
                                    destination: instruction.accounts[2].account.to_string(),
                                    amount: lamports as i64,
                                    timestamp: instruction.timestamp,
                                }
                            )
                        )],
                    };

                    response.push(table_data);

                    Some(response)
                }
            }
        }
        Err(_) => {
            // Error provided has no utility at the moment.
            error!("[spi-wrapper/programs/native_system] Error deserializing this system \
        instruction! tx: {}, tx_instruction_id: {}, parent_idx: {}", instruction.transaction_hash,
        instruction.tx_instruction_id, instruction.parent_index);

            None
        }
    };
}