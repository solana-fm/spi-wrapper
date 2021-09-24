use solana_account_decoder::parse_bpf_loader::{
    parse_bpf_upgradeable_loader, BpfUpgradeableLoaderAccountType,
};
use tracing::error;

use crate::{Instruction, InstructionFunction, InstructionProperty, InstructionSet};
use solana_account_decoder::parse_account_data::{ParseAccountError, ParsableAccount};

pub const PROGRAM_ADDRESS: &str = "BPFLoaderUpgradeab1e11111111111111111111111";

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    instruction: Instruction,
) -> Option<InstructionSet> {
    let bpf_loader_upgradeable_dr =
        parse_bpf_upgradeable_loader(instruction.data.as_slice());

    return if let Ok(bpf_loader_upgradeable_i) = bpf_loader_upgradeable_dr {
        return match bpf_loader_upgradeable_i {
            BpfUpgradeableLoaderAccountType::Uninitialized => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "Uninitialized".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![]
                })
            }
            BpfUpgradeableLoaderAccountType::Buffer(buffer) => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "Buffer".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "authority_address".to_string(),
                            value: if let Some(ba) = buffer.authority {
                                ba
                            } else {
                                "".to_string()
                            },
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "data".to_string(),
                            value: serde_json::to_string(&buffer.data).unwrap().to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                    ]
                })
            }
            BpfUpgradeableLoaderAccountType::Program(program) => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "Program".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "program_data".to_string(),
                            value: serde_json::to_string(&program.program_data).unwrap().to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        }
                    ]
                })
            }
            BpfUpgradeableLoaderAccountType::ProgramData(program_data) => {
                // program_data.data;
                // program_data.authority;
                // program_data.slot;
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "ProgramData".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "authority".to_string(),
                            value: if let Some(auth) = program_data.authority {
                                auth
                            } else {
                                "".to_string()
                            },
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "program_data".to_string(),
                            value: serde_json::to_string(&program_data.data).unwrap().to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction.tx_instruction_id.clone(),
                            transaction_hash: instruction.transaction_hash.clone(),
                            parent_index: instruction.parent_index.clone(),
                            key: "slot".to_string(),
                            value: program_data.slot.to_string(),
                            parent_key: "".to_string(),
                            timestamp: instruction.timestamp,
                        },
                    ]
                })
            }
        }
    } else {
        // If the instruction parsing is failing, bail out
        let instruction_err = bpf_loader_upgradeable_dr.unwrap_err();
        match instruction_err {
            ParseAccountError::AccountNotParsable(parseable_account) => {
                let account_involved = match parseable_account {
                    ParsableAccount::BpfUpgradeableLoader => "BpfUpgradeableLoader",
                    ParsableAccount::Config => "Config",
                    ParsableAccount::Nonce => "Nonce",
                    ParsableAccount::SplToken => "SplToken",
                    ParsableAccount::Stake => "Stake",
                    ParsableAccount::Sysvar => "Sysvar",
                    ParsableAccount::Vote => "Vote",
                };

                error!("[spi-wrapper/bpf_loader_upgradeable] Attempt to parse instruction from \
                program {} failed as the account was not parsable ({} was not parseable).",
                    instruction.program, account_involved);
            }
            ParseAccountError::ProgramNotParsable => {
                error!("[spi-wrapper/bpf_loader_upgradeable] Attempt to parse instruction from \
                program {} failed as it was not parsable.", instruction.program);
            }
            ParseAccountError::AdditionalDataMissing(missing) => {
                error!("[spi-wrapper/bpf_loader_upgradeable] Attempt to parse instruction from \
                program {} failed as it was missing data for {}.", instruction.program, missing);
            }
            ParseAccountError::InstructionError(_err) => {
                // TODO: Tell us what instruction error it exactly is.
                error!("[spi-wrapper/bpf_loader_upgradeable] Attempt to parse instruction from \
                program {} failed as there was an instruction error.", instruction.program);
            }
            ParseAccountError::SerdeJsonError(err) => {
                error!("[spi-wrapper/bpf_loader_upgradeable] Attempt to parse instruction from \
                program {} failed as there was serde json error: {}.", instruction.program, err);
            }
        }

        None
    }
}