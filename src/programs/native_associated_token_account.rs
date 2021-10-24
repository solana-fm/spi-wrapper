use bincode::deserialize;
use chrono::NaiveDateTime;
use solana_sdk::pubkey::Pubkey;
use tracing::error;

use crate::{InstructionProperty, Instruction, InstructionSet, InstructionFunction};

pub const PROGRAM_ADDRESS: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    _instruction: Instruction,
) -> Option<InstructionSet> {
    let atadr = deserialize::<solana_program::instruction::Instruction>(&data);

    return if let Ok(instruction) = atadr {
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
        Some(InstructionSet {
            function: InstructionFunction {
                tx_instruction_id: instruction.tx_instruction_id.clone(),
                transaction_hash: instruction.transaction_hash.clone(),
                parent_index: instruction.parent_index.clone(),
                program: instruction.program.clone(),
                function_name: "".to_string(),
                timestamp: instruction.timestamp
            },
            properties: vec![]
        })
    } else {
        // If the instruction parsing is failing, bail out
        error!("[spi-wrapper/bpf_loader] Attempt to parse instruction from program {} failed due to \
        {}.", _instruction.program, bpf_loader_dr.unwrap_err());

        None
    }
}