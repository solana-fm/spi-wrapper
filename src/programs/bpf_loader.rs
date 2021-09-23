use bincode::deserialize;
use solana_sdk::loader_instruction::LoaderInstruction;
use solana_program::instruction::CompiledInstruction;
use solana_transaction_status::parse_instruction::ParsedInstructionEnum;
use tracing::{event, Level};

use crate::{InstructionProperty, Instruction, InstructionSet, InstructionFunction};

pub const PROGRAM_ADDRESS: String = "BPFLoader1111111111111111111111111111111111".parse().unwrap();
pub const PROGRAM_ADDRESS_2: String = "BPFLoader2111111111111111111111111111111111".parse().unwrap();

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    _instruction: Instruction,
) -> Option<InstructionSet> {
    let bpf_loader_dr = deserialize::<LoaderInstruction>(
        &_instruction.data.as_bytes());

    if let Ok(deserialized_bpf_loader) = bpf_loader_dr {
        return match deserialized_bpf_loader {
            LoaderInstruction::Write { offset, bytes } => {
                /// How things would look like
                /// let write_li = ParsedInstructionEnum {
                ///     instruction_type: "write".to_string(),
                ///     info: json!({
                ///         "offset": offset,
                ///         "bytes": base64::encode(&bytes),
                ///         "account": &program_hash.to_string()}),
                /// };

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
                            key: "instruction_type".to_string(),
                            value: "write".to_string(),
                            parent_key: "".to_string(),
                            timestamp: _instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: _instruction.tx_instruction_id.clone(),
                            transaction_hash: _instruction.transaction_hash.clone(),
                            parent_index: _instruction.parent_index.clone(),
                            key: "info".to_string(),
                            value: "".to_string(),
                            parent_key: "".to_string(),
                            timestamp: _instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: _instruction.tx_instruction_id.clone(),
                            transaction_hash: _instruction.transaction_hash.clone(),
                            parent_index: _instruction.parent_index.clone(),
                            key: "offset".to_string(),
                            value: offset.to_string(),
                            parent_key: "info".to_string(),
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
                        },
                        InstructionProperty {
                            tx_instruction_id: _instruction.tx_instruction_id.clone(),
                            transaction_hash: _instruction.transaction_hash.clone(),
                            parent_index: _instruction.parent_index.clone(),
                            key: "account".to_string(),
                            value: _instruction.program.clone(),
                            parent_key: "info".to_string(),
                            timestamp: _instruction.timestamp,
                        },
                    ],
                })
            }
            LoaderInstruction::Finalize => {
                // let finalize_li = ParsedInstructionEnum {
                //     instruction_type: "finalize".to_string(),
                //     info: json!({"account": &program_hash.to_string()}),
                // };

                Option::from(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: _instruction.tx_instruction_id.clone(),
                        transaction_hash: _instruction.transaction_hash.clone(),
                        parent_index: _instruction.parent_index.clone(),
                        program: _instruction.program.clone(),
                        function_name: "finalize".to_string(),
                        timestamp: _instruction.timestamp,
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: _instruction.tx_instruction_id.clone(),
                            transaction_hash: _instruction.transaction_hash.clone(),
                            parent_index: _instruction.parent_index.clone(),
                            key: "instruction_type".to_string(),
                            value: "finalize".to_string(),
                            parent_key: "".to_string(),
                            timestamp: _instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: _instruction.tx_instruction_id.clone(),
                            transaction_hash: _instruction.transaction_hash.clone(),
                            parent_index: _instruction.parent_index.clone(),
                            key: "info".to_string(),
                            value: "".to_string(),
                            parent_key: "".to_string(),
                            timestamp: _instruction.timestamp,
                        },
                        InstructionProperty {
                            tx_instruction_id: _instruction.tx_instruction_id.clone(),
                            transaction_hash: _instruction.transaction_hash.clone(),
                            parent_index: _instruction.parent_index.clone(),
                            key: "account".to_string(),
                            value: _instruction.program.clone(),
                            parent_key: "info".to_string(),
                            timestamp: _instruction.timestamp,
                        },
                    ],
                })
            }
        }
    } else {
        // If the instruction parsing is failing, bail out
        event!(Level::ERROR, format!("[spi-wrapper/bpf_loader] Attempt to parse instruction \
        from program {} failed due to {}.", instruction.program, bpf_loader_dr.into_err()));

        None
    }
}