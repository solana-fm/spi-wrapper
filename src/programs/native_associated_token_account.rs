use avro_rs::schema::Schema;
use bincode::deserialize;
use serde::Serialize;
use tracing::error;

use crate::{Instruction, TableData, TypedDatum};

pub const PROGRAM_ADDRESS: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
pub const NATIVE_ASSOCIATED_TOKEN_ACCOUNT_NEW_TABLE: &str = "native_associated_token_accounts";
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

#[derive(Serialize)]
pub enum NativeAssociatedTokenAccountDatum {
    NewAccount(NewAssociatedTokenAccount)
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
pub async fn fragment_instruction(
    // The instruction
    instruction: Instruction
) -> Option<Vec<TableData>> {
    let atadr = deserialize::<solana_program::instruction::Instruction>(
        &instruction.data.as_slice());

    return match atadr {
        Ok(ref ati) => {
            let mut response: Vec<TableData> = Vec::new();
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
            let table_data = TableData {
                schema: (*NATIVE_ASSOCIATED_TOKEN_ACCOUNT_SCHEMA).clone(),
                table_name: NATIVE_ASSOCIATED_TOKEN_ACCOUNT_NEW_TABLE.to_string(),
                data: vec![TypedDatum::NativeAssocicatedTokenAccount(
                    NativeAssociatedTokenAccountDatum::NewAccount(
                        NewAssociatedTokenAccount {
                            transaction_hash: instruction.transaction_hash.to_string(),
                            ata_address: associated_token_instruction.accounts[1].pubkey.to_string(),
                            wallet_address: associated_token_instruction.accounts[2].pubkey.to_string(),
                            mint: associated_token_instruction.accounts[3].pubkey.to_string(),
                            timestamp: instruction.timestamp
                        }
                    )
                )]
            };

            response.push(table_data);

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