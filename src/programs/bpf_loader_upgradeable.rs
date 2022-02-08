use avro_rs::schema::Schema;
use serde::Serialize;

use crate::{Instruction, TableData, TypedDatum};
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

#[derive(Serialize)]
pub enum BpfUpgradeableLoaderDatum {
    NativeBpfUpgradeableClosure(NativeBpfUpgradeableClosure),
    NativeBpfUpgradeableDeploy(NativeBpfUpgradeableDeploy),
    NativeBpfUpgradeableUpgrade(NativeBpfUpgradeableUpgrade),
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
    pub timestamp: i64,
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
    pub timestamp: i64,
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
    pub timestamp: i64,
}

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    instruction: Instruction
) -> Option<Vec<TableData>> {
    let bpf_loader_upgradeable_dr =
        bincode::deserialize::<UpgradeableLoaderInstruction>(instruction.data.as_slice());

    return match bpf_loader_upgradeable_dr {
        Ok(ref blu) => {
            let mut response: Vec<TableData> = Vec::new();
            let bpf_loader_upgradeable_i = blu.clone();

            match bpf_loader_upgradeable_i {
                UpgradeableLoaderInstruction::InitializeBuffer => None,
                UpgradeableLoaderInstruction::Write { .. } => None,
                UpgradeableLoaderInstruction::DeployWithMaxDataLen { .. } => {
                    let table_data = TableData {
                        schema: (*NATIVE_BPF_LOADER_UPGRADABLE_DEPLOY_SCHEMA).clone(),
                        table_name: NATIVE_BPF_LOADER_UPGRADABLE_DEPLOYS_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::BpfLoaderUpgradeable(
                            BpfUpgradeableLoaderDatum::NativeBpfUpgradeableDeploy(
                                NativeBpfUpgradeableDeploy {
                                    transaction_hash: instruction.transaction_hash.clone(),
                                    program: instruction.accounts[2].account.to_string(),
                                    program_data: instruction.accounts[1].account.to_string(),
                                    program_authority: instruction.accounts[7].account.to_string(),
                                    timestamp: instruction.timestamp.clone(),
                                }
                            )
                        )],
                    };

                    response.push(table_data);

                    Some(response)
                }
                UpgradeableLoaderInstruction::Upgrade => {
                    let table_data = TableData {
                        schema: (*NATIVE_BPF_LOADER_UPGRADABLE_UPGRADE_SCHEMA).clone(),
                        table_name: NATIVE_BPF_LOADER_UPGRADABLE_UPGRADES_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::BpfLoaderUpgradeable(
                            BpfUpgradeableLoaderDatum::NativeBpfUpgradeableUpgrade(
                                NativeBpfUpgradeableUpgrade {
                                    transaction_hash: instruction.transaction_hash.clone(),
                                    program: instruction.accounts[1].account.to_string(),
                                    program_data: instruction.accounts[0].account.to_string(),
                                    program_buffer: instruction.accounts[2].account.to_string(),
                                    program_authority: instruction.accounts[6].account.to_string(),
                                    timestamp: instruction.timestamp.clone(),
                                }
                            )
                        )],
                    };

                    response.push(table_data);

                    Some(response)
                }
                UpgradeableLoaderInstruction::SetAuthority => None,
                UpgradeableLoaderInstruction::Close => {
                    let table_data = TableData {
                        schema: (*NATIVE_BPF_LOADER_CLOSURE_DEPLOY_SCHEMA).clone(),
                        table_name: NATIVE_BPF_LOADER_UPGRADABLE_CLOSURES_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::BpfLoaderUpgradeable(
                            BpfUpgradeableLoaderDatum::NativeBpfUpgradeableClosure(
                                NativeBpfUpgradeableClosure {
                                    transaction_hash: instruction.transaction_hash.clone(),
                                    program: if instruction.accounts.len() > 3 {
                                        Some(instruction.accounts[3].account.to_string())
                                    } else {
                                        None
                                    },
                                    program_data: instruction.accounts[0].account.to_string(),
                                    program_authority: Some(instruction.accounts[2].account.to_string()),
                                    timestamp: instruction.timestamp.clone(),
                                }
                            )
                        )],
                    };

                    response.push(table_data);

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
    };
}