use avro_rs::schema::Schema;
use serde::Serialize;
use solana_program::instruction::InstructionError;
use solana_sdk::program_utils::limited_deserialize;
use solana_program::stake::instruction::StakeInstruction;
use tracing::error;

use crate::{Instruction, TableData, TypedDatum};

pub const PROGRAM_ADDRESS: &str = "Stake11111111111111111111111111111111111111";

pub const NATIVE_STAKE_WITHDRAWAL_TABLE_NAME: &str = "native_stake_withdrawals";
pub const NATIVE_STAKE_SPLIT_TABLE_NAME: &str = "native_stake_splits";
lazy_static! {
    pub static ref NATIVE_STAKE_SPLIT_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_stake_split",
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
    pub static ref NATIVE_STAKE_WITHDRAWAL_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_stake_withdrawal",
        "fields": [
            {"name": "transaction_hash", "type": "string"},
            {"name": "amount", "type": "long"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
}

#[derive(Serialize)]
pub enum NativeStakeDatum {
    Withdrawal(NativeStakeWithdrawal),
    Split(NativeStakeSplit),
}

/// Struct tables
#[derive(Serialize)]
pub struct NativeStakeWithdrawal {
    /// Which transaction was this?
    pub transaction_hash: String,
    /// Split amount
    pub amount: i64,
    /// Wen exit?
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct NativeStakeSplit {
    /// Which transaction was this?
    pub transaction_hash: String,
    /// Account involved for this amount.
    pub account: String,
    /// Amount of benefit/loss from split.
    pub amount: i64,
    /// Wen split?
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
    let dsr = limited_deserialize::<StakeInstruction>(
        instruction.data.as_slice());

    match dsr {
        Ok(ref si) => {
            let mut response: Vec<TableData> = Vec::new();
            let stake_result = si.clone();
            match stake_result {
                StakeInstruction::Initialize(_, _) => {
                    // me.initialize(
                    //     &authorized,
                    //     &lockup,
                    //     &from_keyed_account::<Rent>(next_keyed_account(keyed_accounts)?)?,
                    // )

                    // Nothing to index here, just telling us who initialised a new stake.
                    None
                }
                StakeInstruction::InitializeChecked => {
                    // Nothing to index here, just telling us who checked a stake initialisation.
                    None
                }
                StakeInstruction::Authorize(_, _) => {
                    // let require_custodian_for_locked_stake_authorize = invoke_context
                    //     .is_feature_active(
                    //         &feature_set::require_custodian_for_locked_stake_authorize::id(
                    //         ),
                    //     );
                    //
                    // if require_custodian_for_locked_stake_authorize {
                    //     let clock = from_keyed_account::<Clock>(next_keyed_account(
                    //         keyed_accounts,
                    //     )?)?;
                    //     let _current_authority = next_keyed_account(keyed_accounts)?;
                    //     let custodian = keyed_accounts.next().map(|ka| ka.unsigned_key());
                    //
                    //     me.authorize(
                    //         &signers,
                    //         &authorized_pubkey,
                    //         stake_authorize,
                    //         require_custodian_for_locked_stake_authorize,
                    //         &clock,
                    //         custodian,
                    //     )
                    // } else {
                    //     me.authorize(
                    //         &signers,
                    //         &authorized_pubkey,
                    //         stake_authorize,
                    //         require_custodian_for_locked_stake_authorize,
                    //         &Clock::default(),
                    //         None,
                    //     )
                    // }

                    // Nothing to index here, just telling us who authorised the stake.
                    None
                }
                StakeInstruction::AuthorizeChecked(_) => {
                    // Nothing to index here, just telling us who checked the stake authorisation.
                    None
                }
                StakeInstruction::AuthorizeCheckedWithSeed(_) => {
                    // Nothing to index here, just telling us who checked the stake authorisation.
                    None
                }
                StakeInstruction::AuthorizeWithSeed(_) => {
                    // Nothing to index here, just telling us who authorised the stake with a seed.
                    None
                }
                StakeInstruction::DelegateStake => {
                    // let can_reverse_deactivation = invoke_context
                    //     .is_feature_active(&feature_set::stake_program_v4::id());
                    // let vote = next_keyed_account(keyed_accounts)?;
                    //
                    // me.delegate(
                    //     &vote,
                    //     &from_keyed_account::<Clock>(next_keyed_account(keyed_accounts)?)?,
                    //     &from_keyed_account::<StakeHistory>(next_keyed_account(
                    //         keyed_accounts,
                    //     )?)?,
                    //     &config::from_keyed_account(next_keyed_account(keyed_accounts)?)?,
                    //     &signers,
                    //     can_reverse_deactivation,
                    // )

                    // Nothing to index here, just telling us who delegated the stake.
                    None
                }
                StakeInstruction::Split(lamports) => {
                    let table_data = TableData {
                        schema: (*NATIVE_STAKE_SPLIT_SCHEMA).clone(),
                        table_name: NATIVE_STAKE_SPLIT_TABLE_NAME.to_string(),
                        data: vec![
                            TypedDatum::NativeStake(NativeStakeDatum::Split(NativeStakeSplit {
                                    transaction_hash: instruction.transaction_hash.clone(),
                                    account: instruction.accounts[0].account.to_string(),
                                    amount: (lamports as i64) * -1,
                                    timestamp: instruction.timestamp
                            })),
                            TypedDatum::NativeStake(NativeStakeDatum::Split(NativeStakeSplit {
                                transaction_hash: instruction.transaction_hash.clone(),
                                account: instruction.accounts[1].account.to_string(),
                                amount: lamports as i64,
                                timestamp: instruction.timestamp
                            })),
                        ],
                    };

                    response.push(table_data);

                    Some(response)
                }
                StakeInstruction::Merge => {
                    // let source_stake = &next_keyed_account(keyed_accounts)?;
                    // let can_merge_expired_lockups = invoke_context
                    //     .is_feature_active(&feature_set::stake_program_v4::id());
                    // me.merge(
                    //     invoke_context,
                    //     source_stake,
                    //     &from_keyed_account::<Clock>(next_keyed_account(keyed_accounts)?)?,
                    //     &from_keyed_account::<StakeHistory>(next_keyed_account(
                    //         keyed_accounts,
                    //     )?)?,
                    //     &signers,
                    //     can_merge_expired_lockups,
                    // )

                    // Nothing to index here, just telling us who merged the stakes.
                    None
                }
                StakeInstruction::Withdraw(lamports) => {
                    // let to = &next_keyed_account(keyed_accounts)?;
                    // me.withdraw(
                    //     lamports,
                    //     to,
                    //     &from_keyed_account::<Clock>(next_keyed_account(keyed_accounts)?)?,
                    //     &from_keyed_account::<StakeHistory>(next_keyed_account(
                    //         keyed_accounts,
                    //     )?)?,
                    //     next_keyed_account(keyed_accounts)?,
                    //     keyed_accounts.next(),
                    //     invoke_context
                    //         .is_feature_active(&feature_set::stake_program_v4::id()),
                    // )
                    let table_data = TableData {
                        schema: (*NATIVE_STAKE_WITHDRAWAL_SCHEMA).clone(),
                        table_name: NATIVE_STAKE_WITHDRAWAL_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::NativeStake(
                            NativeStakeDatum::Withdrawal(
                                NativeStakeWithdrawal {
                                    transaction_hash: instruction.transaction_hash,
                                    amount: lamports as i64,
                                    timestamp: instruction.timestamp,
                                }
                            )
                        )]
                    };

                    response.push(table_data);

                    Some(response)
                }
                StakeInstruction::Deactivate => {
                    // me.deactivate(
                    //     &from_keyed_account::<Clock>(next_keyed_account(keyed_accounts)?)?,
                    //     &signers,
                    // )

                    // Nothing to index here, just telling us who deactivated the stake.
                    None
                }
                StakeInstruction::SetLockup(_) => {
                    // let clock = if invoke_context
                    //     .is_feature_active(&feature_set::stake_program_v4::id())
                    // {
                    //     Some(get_sysvar::<Clock>(invoke_context, &sysvar::clock::id())?)
                    // } else {
                    //     None
                    // };
                    // me.set_lockup(&lockup, &signers, clock.as_ref())

                    // Nothing to index here, just telling us who set the stake's lockup.
                    None
                }
                StakeInstruction::SetLockupChecked(_) => {
                    // lockup_checked

                    // Nothing to index here, just telling us who checked the stake's lockup.
                    None
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
                InstructionError::IllegalOwner => "IllegalOwner".to_string()
            };

            error!("{} Reason: {}", "[spi-wrapper/programs/native_stake] \
        This stake instruction not yet supported!".to_string(), err_msg);

            None
        }
    }
}