use bincode::deserialize;
use tracing::error;

use crate::{InstructionProperty, Instruction, InstructionSet, InstructionFunction};

pub const PROGRAM_ADDRESS: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    instruction: Instruction,
) -> Option<InstructionSet> {
    let atadr = deserialize::<solana_program::instruction::Instruction>(
        &instruction.data.as_slice());

    return if let Ok(associated_token_instruction) = atadr {
        // Create an associated token account for the given wallet address and token mint
        //
        // Accounts expected by this instruction:
        //
        //   0. `[writeable,signer]` Funding account (must be a system account)
        //   1. `[writeable]` Associated token account address to be created
        //   2. `[]` Wallet address for the new associated token account
        //   3. `[]` The token mint for the new associated token account
        //   4. `[]` System program
        //   5. `[]` SPL Token program
        //   6. `[]` Rent sysvar
        let account_sets: Vec<Vec<InstructionProperty>> = associated_token_instruction.accounts
            .into_iter().map(|am| {
                vec![
                    InstructionProperty {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        key: "pubkey".to_string(),
                        value: am.pubkey.to_string(),
                        parent_key: "".to_string(),
                        timestamp: instruction.timestamp.clone(),
                    },
                    InstructionProperty {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        key: "is_signer".to_string(),
                        value: if am.is_signer {
                            "1".to_string()
                        } else {
                            "0".to_string()
                        },
                        parent_key: "".to_string(),
                        timestamp: instruction.timestamp.clone(),
                    },
                    InstructionProperty {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        key: "is_writable".to_string(),
                        value: if am.is_writable {
                            "1".to_string()
                        } else {
                            "0".to_string()
                        },
                        parent_key: "".to_string(),
                        timestamp: instruction.timestamp.clone(),
                    }
                ]
            }).collect();

        let mut properties = vec![
                InstructionProperty {
                    tx_instruction_id: instruction.tx_instruction_id.clone(),
                    transaction_hash: instruction.transaction_hash.clone(),
                    parent_index: instruction.parent_index.clone(),
                    key: "data".to_string(),
                    value: bs58::encode(associated_token_instruction.data).into_string(),
                    parent_key: "".to_string(),
                    timestamp: instruction.timestamp.clone(),
                },
                InstructionProperty {
                    tx_instruction_id: instruction.tx_instruction_id.clone(),
                    transaction_hash: instruction.transaction_hash.clone(),
                    parent_index: instruction.parent_index.clone(),
                    key: "program_id".to_string(),
                    value: associated_token_instruction.program_id.to_string(),
                    parent_key: "".to_string(),
                    timestamp: instruction.timestamp.clone(),
                }
            ];

        for ac in account_sets {
            properties.extend(ac);
        }

        Some(InstructionSet {
            function: InstructionFunction {
                tx_instruction_id: instruction.tx_instruction_id.clone(),
                transaction_hash: instruction.transaction_hash.clone(),
                parent_index: instruction.parent_index.clone(),
                program: instruction.program.clone(),
                function_name: "".to_string(),
                timestamp: instruction.timestamp
            },
            properties
        })
    } else {
        // If the instruction parsing is failing, bail out
        error!("[spi-wrapper/bpf_loader] Attempt to parse instruction from program {}.",
            instruction.program);

        None
    }
}