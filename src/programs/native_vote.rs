use solana_program::instruction::InstructionError;
use solana_sdk::program_utils::limited_deserialize;
use solana_vote_program::vote_instruction::VoteInstruction;
use solana_vote_program::vote_state::VoteAuthorize;
use tracing::error;

use crate::{Instruction, InstructionFunction, InstructionProperty, InstructionSet};

pub const PROGRAM_ADDRESS: &str = "Vote111111111111111111111111111111111111111";

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    instruction: Instruction,
) -> Option<InstructionSet> {
    // Deserialize the instruction
    let vdr: Result<VoteInstruction, InstructionError> = limited_deserialize(
        instruction.data.as_slice());

    return match vdr {
        Ok(ref di) => {
            let deserialized_instruction = di.clone();
            match deserialized_instruction {
                VoteInstruction::InitializeAccount(vote_init) => {
                    // Source code
                    // verify_rent_exemption(me, next_keyed_account(keyed_accounts)?)?;
                    // let vs = vote_state::initialize_account(
                    //     me,
                    //     &vote_init,
                    //     &signers,
                    //     &from_keyed_account::<Clock>(next_keyed_account(keyed_accounts)?)?,
                    //     invoke_context
                    //         .is_feature_active(&feature_set::check_init_vote_data::id()),
                    // );
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "initialize-account".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "node_pubkey".to_string(),
                                value: vote_init.node_pubkey.to_string(),
                                parent_key: "vote_init".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "commission".to_string(),
                                value: vote_init.commission.to_string(),
                                parent_key: "vote_init".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "authorized_withdrawer".to_string(),
                                value: vote_init.authorized_withdrawer.to_string(),
                                parent_key: "vote_init".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "authorized_voter".to_string(),
                                value: vote_init.authorized_voter.to_string(),
                                parent_key: "vote_init".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                        ],
                    })
                }
                VoteInstruction::Authorize(voter_pubkey, vote_authorize) => {
                    // vote_state::authorize(
                    //     me,
                    //     &voter_pubkey,
                    //     vote_authorize,
                    //     &signers,
                    //     &from_keyed_account::<Clock>(next_keyed_account(keyed_accounts)?)?,
                    // )
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "authorize".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "voter_pubkey".to_string(),
                                value: voter_pubkey.to_string(),
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "vote_authorize".to_string(),
                                value: match vote_authorize {
                                    VoteAuthorize::Voter => "voter".to_string(),
                                    VoteAuthorize::Withdrawer => "withdrawer".to_string()
                                },
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            }
                        ],
                    })
                }
                VoteInstruction::AuthorizeChecked(vote_authorize) => {
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "vote-authorize".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "vote_authorize".to_string(),
                                value: match vote_authorize {
                                    VoteAuthorize::Voter => "voter".to_string(),
                                    VoteAuthorize::Withdrawer => "withdrawer".to_string()
                                },
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            }
                        ],
                    })
                }
                VoteInstruction::UpdateValidatorIdentity => {
                    // vote_state::update_validator_identity(
                    //     me,
                    //     next_keyed_account(keyed_accounts)?.unsigned_key(),
                    //     &signers,
                    // )
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "update-validator-identity".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        properties: vec![],
                    })
                }
                VoteInstruction::UpdateCommission(commission) => {
                    // vote_state::update_commission(me, commission, &signers)
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "update-commission".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "commission".to_string(),
                                value: commission.to_string(),
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            }
                        ],
                    })
                }
                VoteInstruction::VoteSwitch(vote, hash) => {
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "vote-switch".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "hash".to_string(),
                                value: bs58::encode(vote.hash.0).into_string(),
                                parent_key: "vote".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "slots".to_string(),
                                value: serde_json::to_string(vote.slots.as_slice()).unwrap(),
                                parent_key: "vote".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "timestamp".to_string(),
                                value: if let Some(ts) = vote.timestamp {
                                    ts.to_string()
                                } else {
                                    "".to_string()
                                },
                                parent_key: "vote".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "hash".to_string(),
                                value: bs58::encode(hash.0).into_string(),
                                parent_key: "".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            }
                        ],
                    })
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
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "vote".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "hash".to_string(),
                                value: bs58::encode(vote.hash.0).into_string(),
                                parent_key: "vote".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "slots".to_string(),
                                value: serde_json::to_string(vote.slots.as_slice()).unwrap(),
                                parent_key: "vote".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            },
                            InstructionProperty {
                                tx_instruction_id: instruction.tx_instruction_id.clone(),
                                transaction_hash: instruction.transaction_hash.clone(),
                                parent_index: instruction.parent_index.clone(),
                                key: "timestamp".to_string(),
                                value: if let Some(ts) = vote.timestamp {
                                    ts.to_string()
                                } else {
                                    "".to_string()
                                },
                                parent_key: "vote".to_string(),
                                timestamp: instruction.timestamp.clone(),
                            }
                        ],
                    })
                }
                VoteInstruction::Withdraw(lamports) => {
                    // let to = next_keyed_account(keyed_accounts)?;
                    // vote_state::withdraw(me, lamports, to, &signers)
                    // vote_state::update_commission(me, commission, &signers)
                    Some(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            program: instruction.program.clone(),
                            function_name: "withdraw".to_string(),
                            timestamp: instruction.timestamp,
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

            error!("{} Reason: {}", "Invalid instruction for the vote program.".to_string(), err_msg);
            None
        }
    }
}