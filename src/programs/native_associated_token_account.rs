use std::collections::HashMap;
use avro_rs::schema::Schema;
use bincode::deserialize;
use itertools::Itertools;
use serde::Serialize;
use tracing::error;

use crate::{InstructionProperty, Instruction, InstructionSet, InstructionFunction};

pub const PROGRAM_ADDRESS: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
pub const NATIVE_ASSOCIATED_TOKEN_ACCOUNT_NEW_TABLE: &str = "native_associated_token_account_new";
lazy_static! {
    pub static ref NATIVE_ASSOCIATED_TOKEN_ACCOUNT_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_new_associated_token_account",
        "fields": [
            {"name": "transaction_hash", "type": "string"},
            {"name": "ata_address", "type": "string"},
            {"name": "wallet_address", "type": "string"},
            {"name": "mint", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
}

/// Struct tables
#[derive(Serialize)]
pub struct NewAssociatedTokenAccount {
    pub transaction_hash: String,
    pub ata_address: String,
    pub wallet_address: String,
    pub mint: String,
    pub timestamp: i64
}

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction<T: Serialize>(
    // The instruction
    instruction: Instruction
) -> Option<HashMap<(String, Schema), Vec<T>>> {
    let atadr = deserialize::<solana_program::instruction::Instruction>(
        &instruction.data.as_slice());

    return match atadr {
        Ok(ref ati) => {
            let mut response: HashMap<(String, Schema), Vec<T>> = HashMap::new();
            let associated_token_instruction = ati.clone();
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
            let key =
                (NATIVE_ASSOCIATED_TOKEN_ACCOUNT_NEW_TABLE.to_string(), *NATIVE_ASSOCIATED_TOKEN_ACCOUNT_SCHEMA);
            let associated_token_account = NewAssociatedTokenAccount {
                transaction_hash: instruction.transaction_hash.to_string(),
                ata_address: associated_token_instruction.account_instructions.into_iter()
                    .filter(|ai| ai.index == 1)
                    .collect(),
                wallet_address: associated_token_instruction.account_instructions.into_iter()
                    .filter(|ai| ai.index == 2)
                    .collect(),
                mint: associated_token_instruction.account_instructions.into_iter()
                    .filter(|ai| ai.index == 3)
                    .collect(),
                timestamp: associated_token_instruction.timestamp
            };
            if response.contains(&key) {
                response[&key].push(associated_token_account);
            } else {
                response[&key] = vec![associated_token_account];
            }

            Some(response)
        }
        Err(err) => {
            // If the instruction parsing is failing, bail out
            error!("[spi-wrapper/bpf_loader] Attempt to parse instruction from program {} failed due to \
        {}.", instruction.program, err);

            None
        }
    }
}