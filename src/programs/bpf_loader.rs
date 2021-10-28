use bincode::deserialize;
use solana_sdk::loader_instruction::LoaderInstruction;
use tracing::error;

use crate::{InstructionProperty, Instruction, InstructionSet, InstructionFunction};

pub const PROGRAM_ADDRESS: &str = "BPFLoader1111111111111111111111111111111111";
pub const PROGRAM_ADDRESS_2: &str = "BPFLoader2111111111111111111111111111111111";

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    _instruction: Instruction,
) -> Option<InstructionSet> {
    let bpf_loader_dr = deserialize::<LoaderInstruction>(
        &_instruction.data);

    return match bpf_loader_dr {
        Ok(ref bld) => {
            let deserialized_bpf_loader = bld.clone();
            return match deserialized_bpf_loader {
                LoaderInstruction::Write { offset, bytes } => {
                    Option::from(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: _instruction.tx_instruction_id.clone(),
                            transaction_hash: _instruction.transaction_hash.clone(),
                            parent_index: _instruction.parent_index.clone(),
                            program: _instruction.program.clone(),
                            function_name: "write".to_string(),
                            timestamp: _instruction.timestamp,
                        },
                        properties: vec![
                            InstructionProperty {
                                tx_instruction_id: _instruction.tx_instruction_id.clone(),
                                transaction_hash: _instruction.transaction_hash.clone(),
                                parent_index: _instruction.parent_index.clone(),
                                key: "offset".to_string(),
                                value: offset.to_string(),
                                parent_key: "".to_string(),
                                timestamp: _instruction.timestamp,
                            },
                            InstructionProperty {
                                tx_instruction_id: _instruction.tx_instruction_id.clone(),
                                transaction_hash: _instruction.transaction_hash.clone(),
                                parent_index: _instruction.parent_index.clone(),
                                key: "bytes".to_string(),
                                value: base64::encode(&bytes),
                                parent_key: "info".to_string(),
                                timestamp: _instruction.timestamp,
                            }
                        ],
                    })
                }
                LoaderInstruction::Finalize => {
                    Option::from(InstructionSet {
                        function: InstructionFunction {
                            tx_instruction_id: _instruction.tx_instruction_id.clone(),
                            transaction_hash: _instruction.transaction_hash.clone(),
                            parent_index: _instruction.parent_index.clone(),
                            program: _instruction.program.clone(),
                            function_name: "finalize".to_string(),
                            timestamp: _instruction.timestamp,
                        },
                        properties: vec![],
                    })
                }
            }
        }
        Err(err) => {
            // If the instruction parsing is failing, bail out
            error!("[spi-wrapper/bpf_loader] Attempt to parse instruction from program {} failed due to \
        {}.", _instruction.program, err);

            None
        }
    }
}