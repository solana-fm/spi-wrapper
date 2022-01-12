use std::collections::HashMap;
use avro_rs::schema::Schema;
use itertools::Itertools;
use serde::Serialize;
use solana_account_decoder::parse_bpf_loader::{
    parse_bpf_upgradeable_loader, BpfUpgradeableLoaderAccountType,
};
use tracing::error;

use crate::{Instruction, InstructionFunction, InstructionProperty, InstructionSet};
use solana_account_decoder::parse_account_data::{ParseAccountError, ParsableAccount};
use solana_program::loader_upgradeable_instruction::UpgradeableLoaderInstruction;

pub const PROGRAM_ADDRESS: &str = "BPFLoaderUpgradeab1e11111111111111111111111";

pub const NATIVE_BPF_LOADER_UPGRADABLE_DEPLOYS_TABLE_NAME: &str = "native_bpf_upgradable_deployments";
pub const NATIVE_BPF_LOADER_UPGRADABLE_UPGRADES_TABLE_NAME: &str = "native_bpf_upgradable_upgrades";
pub const NATIVE_BPF_LOADER_UPGRADABLE_CLOSURES_TABLE_NAME: &str = "native_bpf_upgradable_closures";
lazy_static! {
    pub static ref NATIVE_BPF_LOADER_CLOSURE_DEPLOY_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_bpf_upgradable_closure",
        "fields": [
            {"name": "transaction_hash", "type": "string"},
            {"name": "program", "type": ["null", "string"]},
            {"name": "program_data", "type": "string"},
            {"name": "program_authority", "type": ["null", "string"]},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref NATIVE_BPF_LOADER_UPGRADABLE_DEPLOY_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_bpf_upgradable_deploy",
        "fields": [
            {"name": "transaction_hash", "type": "string"},
            {"name": "program", "type": "string"},
            {"name": "program_data", "type": "string"},
            {"name": "program_authority", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
    pub static ref NATIVE_BPF_LOADER_UPGRADABLE_UPGRADE_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_bpf_upgradable_upgrade",
        "fields": [
            {"name": "transaction_hash", "type": "string"},
            {"name": "program", "type": "string"},
            {"name": "program_data", "type": "string"},
            {"name": "program_buffer", "type": "string"},
            {"name": "program_authority", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
}

/// Struct tables
#[derive(Serialize)]
pub struct NativeBpfUpgradeableDeploy {
    /// Which transaction was this deployment made?
    pub transaction_hash: String,
    /// Where's this program?
    pub program: String,
    /// Which account stores this program's bytecode?
    pub program_data: String,
    /// Who's the program's owner?
    pub program_authority: String,
    /// Wen deployed?
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct NativeBpfUpgradeableUpgrade {
    /// Which transaction was this deployment made?
    pub transaction_hash: String,
    /// Where's this program?
    pub program: String,
    /// Which account stores this program's bytecode?
    pub program_data: String,
    /// Which account storess this program's buffer?
    pub program_buffer: String,
    /// Who's the program's owner?
    pub program_authority: String,
    /// Wen deployed?
    pub timestamp: i64
}

#[derive(Serialize)]
pub struct NativeBpfUpgradeableClosure {
    /// Which transaction was this deployment made?
    pub transaction_hash: String,
    /// Where's this program?
    pub program: Option<String>,
    /// Which account stores this program's bytecode?
    pub program_data: String,
    /// Who's the program's owner?
    pub program_authority: Option<String>,
    /// Wen deployed?
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
    let bpf_loader_upgradeable_dr =
        bincode::deserialize::<UpgradeableLoaderInstruction>(instruction.data.as_slice());

    return match bpf_loader_upgradeable_dr {
        Ok(ref blu) => {
            let mut response: HashMap<(String, Schema), Vec<T>> = HashMap::new();
            let bpf_loader_upgradeable_i = blu.clone();

            match bpf_loader_upgradeable_i {
                /// Initialize a Buffer account.
                ///
                /// A Buffer account is an intermediary that once fully populated is used
                /// with the `DeployWithMaxDataLen` instruction to populate the program's
                /// ProgramData account.
                ///
                /// The `InitializeBuffer` instruction requires no signers and MUST be
                /// included within the same Transaction as the system program's
                /// `CreateAccount` instruction that creates the account being initialized.
                /// Otherwise another party may initialize the account.
                ///
                /// # Account references
                ///   0. `[writable]` source account to initialize.
                ///   1. `[]` Buffer authority, optional, if omitted then the buffer will be
                ///      immutable.
                UpgradeableLoaderInstruction::InitializeBuffer => None,
                /// Write program data into a Buffer account.
                ///
                /// # Account references
                ///   0. `[writable]` Buffer account to write program data to.
                ///   1. `[signer]` Buffer authority
                UpgradeableLoaderInstruction::Write { .. } => None,
                /// Deploy an executable program.
                ///
                /// A program consists of a Program and ProgramData account pair.
                ///   - The Program account's address will serve as the program id for any
                ///     instructions that execute this program.
                ///   - The ProgramData account will remain mutable by the loader only and
                ///     holds the program data and authority information.  The ProgramData
                ///     account's address is derived from the Program account's address and
                ///     created by the DeployWithMaxDataLen instruction.
                ///
                /// The ProgramData address is derived from the Program account's address as
                /// follows:
                ///
                /// ```
                /// # use solana_program::pubkey::Pubkey;
                /// # use solana_program::bpf_loader_upgradeable;
                /// # let program_address = &[];
                /// let (program_data_address, _) = Pubkey::find_program_address(
                ///      &[program_address],
                ///      &bpf_loader_upgradeable::id()
                ///  );
                /// ```
                ///
                /// The `DeployWithMaxDataLen` instruction does not require the ProgramData
                /// account be a signer and therefore MUST be included within the same
                /// Transaction as the system program's `CreateAccount` instruction that
                /// creates the Program account. Otherwise another party may initialize the
                /// account.
                ///
                /// # Account references
                ///   0. `[signer]` The payer account that will pay to create the ProgramData
                ///      account.
                ///   1. `[writable]` The uninitialized ProgramData account.
                ///   2. `[writable]` The uninitialized Program account.
                ///   3. `[writable]` The Buffer account where the program data has been
                ///      written.  The buffer account's authority must match the program's
                ///      authority
                ///   4. `[]` Rent sysvar.
                ///   5. `[]` Clock sysvar.
                ///   6. `[]` System program (`solana_sdk::system_program::id()`).
                ///   7. `[signer]` The program's authority
                UpgradeableLoaderInstruction::DeployWithMaxDataLen { .. } => {
                    let key =
                        (NATIVE_BPF_LOADER_UPGRADABLE_DEPLOYS_TABLE_NAME.to_string(), *NATIVE_BPF_LOADER_UPGRADABLE_DEPLOY_SCHEMA);
                    let deployment_data = NativeBpfUpgradeableDeploy {
                        transaction_hash: instruction.transaction_hash.clone(),
                        program: instruction.accounts[2].account.to_string(),
                        program_data: instruction.accounts[1].account.to_string(),
                        program_authority: instruction.accounts[7].account.to_string(),
                        timestamp: instruction.timestamp.clone()
                    };
                    if response.contains(&key) {
                        response[&key].push(deployment_data);
                    } else {
                        response[&key] = vec![deployment_data];
                    }

                    Some(response)
                }
                /// Upgrade a program.
                ///
                /// A program can be updated as long as the program's authority has not been
                /// set to `None`.
                ///
                /// The Buffer account must contain sufficient lamports to fund the
                /// ProgramData account to be rent-exempt, any additional lamports left over
                /// will be transferred to the spill account, leaving the Buffer account
                /// balance at zero.
                ///
                /// # Account references
                ///   0. `[writable]` The ProgramData account.
                ///   1. `[writable]` The Program account.
                ///   2. `[writable]` The Buffer account where the program data has been
                ///      written.  The buffer account's authority must match the program's
                ///      authority
                ///   3. `[writable]` The spill account.
                ///   4. `[]` Rent sysvar.
                ///   5. `[]` Clock sysvar.
                ///   6. `[signer]` The program's authority.
                UpgradeableLoaderInstruction::Upgrade => {
                    let key =
                        (NATIVE_BPF_LOADER_UPGRADABLE_UPGRADES_TABLE_NAME.to_string(), *NATIVE_BPF_LOADER_UPGRADABLE_UPGRADE_SCHEMA);
                    let deployment_data = NativeBpfUpgradeableUpgrade {
                        transaction_hash: instruction.transaction_hash.clone(),
                        program: instruction.accounts[1].account.to_string(),
                        program_data: instruction.accounts[0].account.to_string(),
                        program_buffer: instruction.accounts[2].account.to_string(),
                        program_authority: instruction.accounts[6].account.to_string(),
                        timestamp: instruction.timestamp.clone()
                    };
                    if response.contains(&key) {
                        response[&key].push(deployment_data);
                    } else {
                        response[&key] = vec![deployment_data];
                    }

                    Some(response)
                }
                /// Set a new authority that is allowed to write the buffer or upgrade the
                /// program.  To permanently make the buffer immutable or disable program
                /// updates omit the new authority.
                ///
                /// # Account references
                ///   0. `[writable]` The Buffer or ProgramData account to change the
                ///      authority of.
                ///   1. `[signer]` The current authority.
                ///   2. `[]` The new authority, optional, if omitted then the program will
                ///      not be upgradeable.
                UpgradeableLoaderInstruction::SetAuthority => None,
                /// Closes an account owned by the upgradeable loader of all lamports and
                /// withdraws all the lamports
                ///
                /// # Account references
                ///   0. `[writable]` The account to close, if closing a program must be the
                ///      ProgramData account.
                ///   1. `[writable]` The account to deposit the closed account's lamports.
                ///   2. `[signer]` The account's authority, Optional, required for
                ///      initialized accounts.
                ///   3. `[writable]` The associated Program account if the account to close
                ///      is a ProgramData account.
                UpgradeableLoaderInstruction::Close => {
                    let key =
                        (NATIVE_BPF_LOADER_UPGRADABLE_CLOSURES_TABLE_NAME.to_string(), *NATIVE_BPF_LOADER_CLOSURE_DEPLOY_SCHEMA);
                    // TODO: Does not handle all edge cases.
                    let deployment_data = NativeBpfUpgradeableClosure {
                        transaction_hash: instruction.transaction_hash.clone(),
                        program: Some(instruction.accounts[3].account.to_string()),
                        program_data: instruction.accounts[0].account.to_string(),
                        program_authority: Some(instruction.accounts[2].account.to_string()),
                        timestamp: instruction.timestamp.clone()
                    };
                    if response.contains(&key) {
                        response[&key].push(deployment_data);
                    } else {
                        response[&key] = vec![deployment_data];
                    }

                    Some(response)
                }
            }
        }
        Err(_) => {
            // If the instruction parsing is failing, bail out
            // match instruction_err {
            // }

            None
        }
    }
}