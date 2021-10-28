use solana_config_program::ConfigKeys;
use solana_program::instruction::InstructionError;
use solana_sdk::program_utils::limited_deserialize;
use tracing::error;

use crate::{InstructionProperty, Instruction, InstructionSet, InstructionFunction};

pub const PROGRAM_ADDRESS: &str = "Config1111111111111111111111111111111111111";

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    instruction: Instruction,
) -> Option<InstructionSet> {
    let key_list_result = limited_deserialize::<ConfigKeys>(
        instruction.data.as_slice());

    return match key_list_result {
        Ok(ref kl) => {
            let key_list = kl.clone();

            let mut instruction_set = InstructionSet {
                function: InstructionFunction {
                    tx_instruction_id: instruction.tx_instruction_id.clone(),
                    transaction_hash: instruction.transaction_hash.clone(),
                    parent_index: instruction.parent_index.clone(),
                    program: instruction.program.clone(),
                    function_name: "".to_string(),
                    timestamp: instruction.timestamp,
                },
                properties: vec![],
            };

            let config_keys: Vec<Vec<InstructionProperty>> = (0..key_list.keys.len()).into_iter()
                .map(|idx| {
                    let mut properties = Vec::new();
                    let cloned_key_list = key_list.keys.clone();

                    let (pk, is_signer) = cloned_key_list[idx];
                    let key_name = "config_keys/".to_owned() + &*idx.to_string();
                    let pubkey_name = key_name.clone() + &"/pubkey".to_owned();
                    properties.push(InstructionProperty {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        key: pubkey_name,
                        value: pk.to_string(),
                        parent_key: key_name.clone(),
                        timestamp: instruction.timestamp.clone(),
                    });

                    let signer_name = key_name.clone() + &"/signer".to_owned();
                    properties.push(InstructionProperty {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        key: signer_name.clone(),
                        value: (is_signer as i32).to_string(),
                        parent_key: key_name,
                        timestamp: instruction.timestamp.clone(),
                    });

                    properties
                }).collect();

            let mut properties = Vec::new();
            for ck in config_keys {
                properties.extend(ck);
            }
            instruction_set.properties = properties;

            Some(instruction_set)
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

            error!("{}{}", "[spi-wrapper/programs/native_config] Unable to deserialize the config \
            keys due to: ", err_msg);

            None
        }
    }
}