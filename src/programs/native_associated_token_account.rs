use avro_rs::schema::Schema;
use bincode::deserialize;
use serde::Serialize;
use tracing::{error, warn};

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
            {"name": "tx_hash", "type": "string"},
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
    pub tx_hash: String,
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
    if instruction.accounts.len() < 3 {
        warn!("Currently parsed ATA has insufficient accounts.");
        return None
    }

    let mut response = Vec::new();

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
                    tx_hash: instruction.transaction_hash.to_string(),
                    ata_address: instruction.accounts[1].account.to_string(),
                    wallet_address: instruction.accounts[2].account.to_string(),
                    mint: instruction.accounts[3].account.to_string(),
                    timestamp: instruction.timestamp
                }
            )
        )]
    };

    response.push(table_data);

    Some(response)
}