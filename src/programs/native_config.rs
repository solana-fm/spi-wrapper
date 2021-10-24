use solana_config_program::ConfigKeys;
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

    return if let Ok(key_list) = key_list_result {
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
    } else {
        error!("{}", "[spi-wrapper/programs/native_config] Unable to deserialize the config keys");

        None
    };
}