use solana_program::instruction::InstructionError;
use solana_sdk::program_utils::limited_deserialize;
use solana_program::stake::instruction::StakeInstruction;
use solana_program::stake::state::StakeAuthorize;
use tracing::error;

use crate::{InstructionProperty, Instruction, InstructionSet, InstructionFunction};

pub const PROGRAM_ADDRESS: &str = "Stake11111111111111111111111111111111111111";

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    instruction: Instruction,
) -> Option<InstructionSet> {
    let dsr = limited_deserialize::<StakeInstruction>(
        instruction.data.as_slice());

    match dsr {
        Ok(ref si) => {
            let stake_result = si.clone();
            match stake_result {
                StakeInstruction::Initialize(authorized, lockup) => {
                    // me.initialize(
                    //     &authorized,
                    //     &lockup,
                    //     &from_keyed_account::<Rent>(next_keyed_account(keyed_accounts)?)?,
                    // )
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "initialize".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "staker".to_string(),
                                value: authorized.staker.to_string(),
                                parent_key: "authorized".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "withdrawer".to_string(),
                                value: authorized.withdrawer.to_string(),
                                parent_key: "authorized".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "epoch".to_string(),
                                value: lockup.epoch.to_string(),
                                parent_key: "lockup".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "custodian".to_string(),
                                value: lockup.custodian.to_string(),
                                parent_key: "lockup".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "unix_timestamp".to_string(),
                                value: lockup.unix_timestamp.to_string(),
                                parent_key: "lockup".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                        ],
                    })
                }
                StakeInstruction::InitializeChecked => {
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "initialize-checked".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        properties: vec![],
                    })
                }
                StakeInstruction::Authorize(authorized_pubkey, stake_authorize) => {
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
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "authorize".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "authorized_pubkey".to_string(),
                                value: authorized_pubkey.to_string(),
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "stake_authorize".to_string(),
                                value: match stake_authorize {
                                    StakeAuthorize::Staker => "staker".to_string(),
                                    StakeAuthorize::Withdrawer => "withdrawer".to_string()
                                },
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                        ],
                    })
                }
                StakeInstruction::AuthorizeChecked(stake_authorize) => {
                    // stake_authorize
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "authorize-checked".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "stake_authorize".to_string(),
                                value: match stake_authorize {
                                    StakeAuthorize::Staker => "staker".to_string(),
                                    StakeAuthorize::Withdrawer => "withdrawer".to_string()
                                },
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            }
                        ],
                    })
                }
                StakeInstruction::AuthorizeCheckedWithSeed(authorize_checked_with_seed_args) => {
                    // stake_authorize
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "authorize-checked-with-seed".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "authority_seed".to_string(),
                                value: authorize_checked_with_seed_args.authority_seed.to_string(),
                                parent_key: "authorize_checked_with_seed_args".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "authority_owner".to_string(),
                                value: authorize_checked_with_seed_args.authority_owner.to_string(),
                                parent_key: "authorize_checked_with_seed_args".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "stake_authorize".to_string(),
                                value: match authorize_checked_with_seed_args.stake_authorize {
                                    StakeAuthorize::Staker => "staker".to_string(),
                                    StakeAuthorize::Withdrawer => "withdrawer".to_string()
                                },
                                parent_key: "authorize_checked_with_seed_args".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                        ],
                    })
                }
                StakeInstruction::AuthorizeWithSeed(authorize_with_seed_args) => {
                    // let authority_base = next_keyed_account(keyed_accounts)?;
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
                    //     let custodian = keyed_accounts.next().map(|ka| ka.unsigned_key());
                    //
                    //     me.authorize_with_seed(
                    //         &authority_base,
                    //         &args.authority_seed,
                    //         &args.authority_owner,
                    //         &args.new_authorized_pubkey,
                    //         args.stake_authorize,
                    //         require_custodian_for_locked_stake_authorize,
                    //         &clock,
                    //         custodian,
                    //     )
                    // } else {
                    //     me.authorize_with_seed(
                    //         &authority_base,
                    //         &args.authority_seed,
                    //         &args.authority_owner,
                    //         &args.new_authorized_pubkey,
                    //         args.stake_authorize,
                    //         require_custodian_for_locked_stake_authorize,
                    //         &Clock::default(),
                    //         None,
                    //     )
                    // }
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "authorize-with-seed".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "authority_seed".to_string(),
                                value: authorize_with_seed_args.authority_seed.to_string(),
                                parent_key: "authorize_with_seed_args".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "authority_owner".to_string(),
                                value: authorize_with_seed_args.authority_owner.to_string(),
                                parent_key: "authorize_with_seed_args".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "stake_authorize".to_string(),
                                value: match authorize_with_seed_args.stake_authorize {
                                    StakeAuthorize::Staker => "staker".to_string(),
                                    StakeAuthorize::Withdrawer => "withdrawer".to_string()
                                },
                                parent_key: "authorize_checked_with_seed_args".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "new_authorized_pubkey".to_string(),
                                value: authorize_with_seed_args.new_authorized_pubkey.to_string(),
                                parent_key: "authorize_checked_with_seed_args".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                        ],
                    })
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
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "delegate-stake".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        properties: vec![],
                    })
                }
                StakeInstruction::Split(lamports) => {
                    // let split_stake = &next_keyed_account(keyed_accounts)?;
                    // me.split(lamports, split_stake, &signers)
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "split".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "lamports".to_string(),
                                value: lamports.to_string(),
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            }
                        ],
                    })
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
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "merge".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        properties: vec![],
                    })
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
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "withdraw".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "lamports".to_string(),
                                value: lamports.to_string(),
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            }
                        ],
                    })
                }
                StakeInstruction::Deactivate => {
                    // me.deactivate(
                    //     &from_keyed_account::<Clock>(next_keyed_account(keyed_accounts)?)?,
                    //     &signers,
                    // )
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "deactivate".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        properties: vec![],
                    })
                }
                StakeInstruction::SetLockup(lockup_args) => {
                    // let clock = if invoke_context
                    //     .is_feature_active(&feature_set::stake_program_v4::id())
                    // {
                    //     Some(get_sysvar::<Clock>(invoke_context, &sysvar::clock::id())?)
                    // } else {
                    //     None
                    // };
                    // me.set_lockup(&lockup, &signers, clock.as_ref())
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "set-lockup".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "epoch".to_string(),
                                value: if let Some(epoch) = lockup_args.epoch {
                                    epoch.to_string()
                                } else {
                                    "".to_string()
                                },
                                parent_key: "lockup_args".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "unix_timestamp".to_string(),
                                value: if let Some(unix_timestamp) = lockup_args.unix_timestamp {
                                    unix_timestamp.to_string()
                                } else {
                                    "".to_string()
                                },
                                parent_key: "lockup_args".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "custodian".to_string(),
                                value: if let Some(custodian) = lockup_args.custodian {
                                    custodian.to_string()
                                } else {
                                    "".to_string()
                                },
                                parent_key: "lockup_args".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                        ],
                    })
                }
                StakeInstruction::SetLockupChecked(lockup_checked_args) => {
                    // lockup_checked
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "set-lockup-checked".to_string(),
                            timestamp: instruction.timestamp.clone(),
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "epoch".to_string(),
                                value: if let Some(epoch) = lockup_checked_args.epoch {
                                    epoch.to_string()
                                } else {
                                    "".to_string()
                                },
                                parent_key: "lockup_checked_args".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "unix_timestamp".to_string(),
                                value: if let Some(unix_timestamp) =
                                lockup_checked_args.unix_timestamp {
                                    unix_timestamp.to_string()
                                } else {
                                    "".to_string()
                                },
                                parent_key: "lockup_checked_args".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                        ],
                    })
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