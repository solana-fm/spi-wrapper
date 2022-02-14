use avro_rs::Schema;
use serde::Serialize;
use solana_program::instruction::InstructionError;
use solana_sdk::program_utils::limited_deserialize;
use solana_sdk::transaction::Transaction;
use solana_vote_program::vote_instruction::VoteInstruction;
use tracing::error;

use crate::{Instruction, TableData, TypedDatum};

pub const PROGRAM_ADDRESS: &str = "Vote111111111111111111111111111111111111111";

pub const NATIVE_VOTE_NEW_VOTE_ACCOUNT_TABLE_NAME: &str = "native_vote_new_vote_accounts";
pub const NATIVE_VOTE_NODE_COMMISSION_TABLE_NAME: &str = "native_vote_node_commissions";
pub const NATIVE_VOTE_UPDATED_VALIDATOR_IDENTITY_TABLE_NAME: &str = "native_vote_updated_validator_identities";
pub const NATIVE_VOTE_VOTE_TABLE_NAME: &str = "native_vote_votes";
pub const NATIVE_VOTE_ACCOUNT_WITHDRAWAL_TABLE_NAME: &str = "native_vote_account_withdrawals";

lazy_static! {
    pub static ref NATIVE_VOTE_NEW_VOTE_ACCOUNT_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_vote_new_vote_account",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "account", "type": "string"},
            {"name": "node", "type": "string"},
            {"name": "authorised_voter", "type": "string"},
            {"name": "authorised_withdrawer", "type": "string"},
            {"name": "commission", "type": "int"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref NATIVE_VOTE_UPDATED_VALIDATOR_IDENTITY_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_vote_updated_validator_identity",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "account", "type": "string"},
            {"name": "new_identity", "type": "string"},
            {"name": "withdraw_authority", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref NATIVE_VOTE_NODE_COMMISSION_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_vote_node_commission",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "vote_account", "type": "string"},
            {"name": "withdraw_authority", "type": "string"},
            {"name": "commission", "type": "int"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref NATIVE_VOTE_VOTE_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_vote_vote",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "account", "type": "string"},
            {"name": "authority", "type": "string"},
            {"name": "slots", "type": "array", "items": "long" },
            {"name": "last_slot_sig", "type": "string"},
            {"name": "last_slot_timestamp", "type": ["null", "long"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref NATIVE_VOTE_ACCOUNT_WITHDRAWAL_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_vote_account_withdrawal",
        "fields": [
            {"name": "tx_hash", "type": "string"},
            {"name": "account", "type": "string"},
            {"name": "amount", "type": "long"},
            {"name": "recipient", "type": "string"},
            {"name": "withdraw_authority", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
}

#[derive(Serialize)]
pub enum VoteDatum {
    InitialiseAccount(NewVoteAccount),
    NodeCommission(NodeCommission),
    UpdatedValidatorIdentity(UpdatedValidatorIdentity),
    Vote(Vote),
    VoteAccountWithdrawal(VoteAccountWithdrawal)
}

#[derive(Serialize)]
pub struct NewVoteAccount {
    pub tx_hash : String,
    pub account: String,
    pub node: String,
    pub authorised_voter: String,
    pub authorised_withdrawer: String,
    pub commission: i16,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct NodeCommission {
    pub tx_hash : String,
    pub vote_account: String,
    pub withdraw_authority: String,
    pub commission: i16,
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct UpdatedValidatorIdentity {
    pub tx_hash : String,
    pub account: String,
    pub new_identity: String,
    pub withdraw_authority: String,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct Vote {
    pub tx_hash : String,
    pub account: String,
    pub authority: String,
    pub slots: Vec<i64>,
    pub last_slot_sig: String,
    pub last_slot_timestamp: Option<i64>,
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct VoteAccountWithdrawal {
    pub tx_hash : String,
    pub account: String,
    pub amount: i64,
    pub recipient: String,
    pub withdraw_authority: String,
    pub timestamp: i64,
}

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    instruction: Instruction,
    transaction: Transaction
) -> Option<Vec<TableData>> {
    // Deserialize the instruction
    let vdr: Result<VoteInstruction, InstructionError> = limited_deserialize(
        instruction.data.as_slice());

    return match vdr {
        Ok(ref di) => {
            let mut response: Vec<TableData> = Vec::new();
            let deserialized_instruction = di.clone();
            match deserialized_instruction {
                VoteInstruction::InitializeAccount(vote_init) => {
                    let table_data = TableData {
                        schema: (*NATIVE_VOTE_NEW_VOTE_ACCOUNT_SCHEMA).clone(),
                        table_name: NATIVE_VOTE_NEW_VOTE_ACCOUNT_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::NativeVote(
                            VoteDatum::InitialiseAccount(
                                NewVoteAccount {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    account: instruction.accounts[0].account.to_string(),
                                    node: vote_init.node_pubkey.to_string(),
                                    authorised_voter: vote_init.authorized_voter.to_string(),
                                    authorised_withdrawer: vote_init.authorized_withdrawer.to_string(),
                                    commission: vote_init.commission as i16,
                                    timestamp: instruction.timestamp
                                }
                            )
                        )]
                    };

                    response.push(table_data);

                    Some(response)
                }
                // TODO: To consider Vote spam attacks if it happens, should do so if we need to track the number of voters specific to each validator.
                VoteInstruction::Authorize(_, _) => {
                    // vote_state::authorize(
                    //     me,
                    //     &voter_pubkey,
                    //     vote_authorize,
                    //     &signers,
                    //     &from_keyed_account::<Clock>(next_keyed_account(keyed_accounts)?)?,
                    // )
                    None
                }
                // TODO: To consider Vote spam attacks if it happens, should do so if we need to track the number of voters specific to each validator.
                VoteInstruction::AuthorizeChecked(_) => {
                    None
                }
                VoteInstruction::UpdateValidatorIdentity => {
                    // vote_state::update_validator_identity(
                    //     me,
                    //     next_keyed_account(keyed_accounts)?.unsigned_key(),
                    //     &signers,
                    // )
                    let table_data = TableData {
                        schema: (*NATIVE_VOTE_UPDATED_VALIDATOR_IDENTITY_SCHEMA).clone(),
                        table_name: NATIVE_VOTE_UPDATED_VALIDATOR_IDENTITY_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::NativeVote(
                            VoteDatum::UpdatedValidatorIdentity(
                                UpdatedValidatorIdentity {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    account: instruction.accounts[0].account.clone(),
                                    new_identity: instruction.accounts[1].account.clone(),
                                    withdraw_authority: instruction.accounts[2].account.clone(),
                                    timestamp: instruction.timestamp,
                                }
                            )
                        )]
                    };

                    response.push(table_data);

                    Some(response)
                }
                VoteInstruction::UpdateCommission(commission) => {
                    // vote_state::update_commission(me, commission, &signers)
                    let table_data = TableData {
                        schema: (*NATIVE_VOTE_NODE_COMMISSION_SCHEMA).clone(),
                        table_name: NATIVE_VOTE_NODE_COMMISSION_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::NativeVote(
                            VoteDatum::NodeCommission(
                                NodeCommission {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    vote_account: instruction.accounts[0].account.clone(),
                                    withdraw_authority: instruction.accounts[1].account.clone(),
                                    commission: commission as i16,
                                    timestamp: instruction.timestamp,
                                }
                            )
                        )]
                    };

                    response.push(table_data);

                    Some(response)
                }
                // TODO: Consider indexing specific votes
                VoteInstruction::VoteSwitch(_, _) => {
                    None
                }
                VoteInstruction::Vote(vote) => {
                    // Source code execution
                    // vote_state::process_vote(
                    //     me,
                    //     &from_keyed_account::<SlotHashes>(next_keyed_account(keyed_accounts)?)?,
                    //     &from_keyed_account::<Clock>(next_keyed_account(keyed_accounts)?)?,
                    //     &vote,
                    //     &signers,
                    // )
                    let table_data = TableData {
                        schema: (*NATIVE_VOTE_VOTE_SCHEMA).clone(),
                        table_name: NATIVE_VOTE_VOTE_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::NativeVote(
                            VoteDatum::Vote(
                                Vote {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    account: instruction.accounts[0].account.to_string(),
                                    authority: instruction.accounts[3].account.to_string(),
                                    slots: vote.slots.as_slice().into_iter().map(|s| *s as i64).collect(),
                                    last_slot_sig: vote.hash.to_string(),
                                    last_slot_timestamp: if let Some(last_timestamp) = vote.timestamp {
                                        Some(last_timestamp as i64)
                                    } else {
                                        None
                                    },
                                    timestamp: instruction.timestamp,
                                }
                            )
                        )]
                    };

                    response.push(table_data);

                    Some(response)
                }
                VoteInstruction::Withdraw(lamports) => {
                    // let to = next_keyed_account(keyed_accounts)?;
                    // vote_state::withdraw(me, lamports, to, &signers)
                    // vote_state::update_commission(me, commission, &signers)
                    let table_data = TableData {
                        schema: (*NATIVE_VOTE_ACCOUNT_WITHDRAWAL_SCHEMA).clone(),
                        table_name: NATIVE_VOTE_ACCOUNT_WITHDRAWAL_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::NativeVote(
                            VoteDatum::VoteAccountWithdrawal(
                                VoteAccountWithdrawal {
                                    tx_hash: instruction.transaction_hash.to_string(),
                                    account: instruction.accounts[0].account.clone(),
                                    amount: lamports as i64,
                                    recipient: instruction.accounts[1].account.clone(),
                                    withdraw_authority: instruction.accounts[2].account.clone(),
                                    timestamp: instruction.timestamp,
                                }
                            )
                        )]
                    };

                    response.push(table_data);

                    Some(response)
                }
            }
        }
        Err(err) => {
            let err_msg = match err {
                InstructionError::GenericError => "GenericError".to_string(),
                InstructionError::InvalidArgument => "InvalidArgument".to_string(),
                InstructionError::InvalidInstructionData => "InvalidInstructionData".to_string(),
                InstructionError::InvalidAccountData => "InvalidAccountData".to_string(),
                InstructionError::AccountDataTooSmall => "AccountDataTooSmall".to_string(),
                InstructionError::InsufficientFunds => "InsufficientFunds".to_string(),
                InstructionError::IncorrectProgramId => "IncorrectProgramId".to_string(),
                InstructionError::MissingRequiredSignature => "MissingRequiredSignature".to_string(),
                InstructionError::AccountAlreadyInitialized => "AccountAlreadyInitialized".to_string(),
                InstructionError::UninitializedAccount => "UninitializedAccount".to_string(),
                InstructionError::UnbalancedInstruction => "UnbalancedInstruction".to_string(),
                InstructionError::ModifiedProgramId => "ModifiedProgramId".to_string(),
                InstructionError::ExternalAccountLamportSpend => "ExternalAccountLamportSpend".to_string(),
                InstructionError::ExternalAccountDataModified => "ExternalAccountDataModified".to_string(),
                InstructionError::ReadonlyLamportChange => "ReadonlyLamportChange".to_string(),
                InstructionError::ReadonlyDataModified => "ReadonlyDataModified".to_string(),
                InstructionError::DuplicateAccountIndex => "DuplicateAccountIndex".to_string(),
                InstructionError::ExecutableModified => "ExecutableModified".to_string(),
                InstructionError::RentEpochModified => "RentEpochModified".to_string(),
                InstructionError::NotEnoughAccountKeys => "NotEnoughAccountKeys".to_string(),
                InstructionError::AccountDataSizeChanged => "AccountDataSizeChanged".to_string(),
                InstructionError::AccountNotExecutable => "AccountNotExecutable".to_string(),
                InstructionError::AccountBorrowFailed => "AccountBorrowFailed".to_string(),
                InstructionError::AccountBorrowOutstanding => "AccountBorrowOutstanding".to_string(),
                InstructionError::DuplicateAccountOutOfSync => "DuplicateAccountOutOfSync".to_string(),
                InstructionError::Custom(_) => "Custom".to_string(),
                InstructionError::InvalidError => "InvalidError".to_string(),
                InstructionError::ExecutableDataModified => "ExecutableDataModified".to_string(),
                InstructionError::ExecutableLamportChange => "ExecutableLamportChange".to_string(),
                InstructionError::ExecutableAccountNotRentExempt => "ExecutableAccountNotRentExempt".to_string(),
                InstructionError::UnsupportedProgramId => "UnsupportedProgramId".to_string(),
                InstructionError::CallDepth => "CallDepth".to_string(),
                InstructionError::MissingAccount => "MissingAccount".to_string(),
                InstructionError::ReentrancyNotAllowed => "ReentrancyNotAllowed".to_string(),
                InstructionError::MaxSeedLengthExceeded => "MaxSeedLengthExceeded".to_string(),
                InstructionError::InvalidSeeds => "InvalidSeeds".to_string(),
                InstructionError::InvalidRealloc => "InvalidRealloc".to_string(),
                InstructionError::ComputationalBudgetExceeded => "ComputationalBudgetExceeded".to_string(),
                InstructionError::PrivilegeEscalation => "PrivilegeEscalation".to_string(),
                InstructionError::ProgramEnvironmentSetupFailure => "ProgramEnvironmentSetupFailure".to_string(),
                InstructionError::ProgramFailedToComplete => "ProgramFailedToComplete".to_string(),
                InstructionError::ProgramFailedToCompile => "ProgramFailedToCompile".to_string(),
                InstructionError::Immutable => "Immutable".to_string(),
                InstructionError::IncorrectAuthority => "IncorrectAuthority".to_string(),
                InstructionError::BorshIoError(_) => "BorshIoError".to_string(),
                InstructionError::AccountNotRentExempt => "AccountNotRentExempt".to_string(),
                InstructionError::InvalidAccountOwner => "InvalidAccountOwner".to_string(),
                InstructionError::ArithmeticOverflow => "ArithmeticOverflow".to_string(),
                InstructionError::UnsupportedSysvar => "UnsupportedSysvar".to_string(),
                InstructionError::IllegalOwner => "IllegalOwner".to_string(),
                InstructionError::AccountsDataBudgetExceeded => "AccountsDataBudgetExceeded".to_string(),
                InstructionError::ActiveVoteAccountClose => "ActiveVoteAccountClose".to_string()
            };

            error!("{} Reason: {}", "Invalid instruction for the vote program.".to_string(), err_msg);
            None
        }
    }
}