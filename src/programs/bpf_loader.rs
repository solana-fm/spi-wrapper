use avro_rs::schema::Schema;
use std::collections::HashMap;
use bincode::deserialize;
use itertools::Itertools;
use solana_sdk::loader_instruction::LoaderInstruction;
use tracing::error;

use crate::{InstructionProperty, Instruction, InstructionSet, InstructionFunction};

pub const PROGRAM_ADDRESS: &str = "BPFLoader1111111111111111111111111111111111";
pub const PROGRAM_ADDRESS_2: &str = "BPFLoader2111111111111111111111111111111111";

pub const NATIVE_BPF_LOADER_WRITE_TABLE_NAME: &str = "native_bpf_writes";
lazy_static! {
    pub static ref NATIVE_BPF_LOADER_WRITE_SCHEMA: Schema = Schema::parse_str(
        r#"
    {
        "type": "record",
        "name": "native_bpf_write",
        "fields": [
            {"name": "transaction_hash", "type": "string"},
            {"name": "program", "type": "string"},
            {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
        ]
    }
    "#
    )
    .unwrap();
}

/// Struct tables
#[derive(Serialize)]
pub struct NativeBpfWrite {
    /// Which transaction was this?
    pub transaction_hash: String,
    /// Program Involved
    pub program: String,
    /// Wen exit?
    pub timestamp: i64
}

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    _instruction: Instruction
) -> Option<HashMap<(String, Schema), Vec<T>>> {
    let bpf_loader_dr = deserialize::<LoaderInstruction>(
        &_instruction.data);

    return match bpf_loader_dr {
        Ok(ref bld) => {
            let deserialized_bpf_loader = bld.clone();
            let mut response: HashMap<(String, Schema), Vec<T>> = HashMap::new();
            return match deserialized_bpf_loader {
                /// Write program data into an account
                ///
                /// # Account references
                ///   0. [WRITE, SIGNER] Account to write to
                // Write {
                //     /// Offset at which to write the given bytes
                //     offset: u32,
                //
                //     /// Serialized program data
                //     #[serde(with = "serde_bytes")]
                //     bytes: Vec<u8>,
                // },
                LoaderInstruction::Write { .. } => {
                    let key =
                        (NATIVE_BPF_LOADER_WRITE_TABLE_NAME.to_string(), *NATIVE_BPF_LOADER_WRITE_SCHEMA);
                    let write_data = NativeBpfWrite {
                        transaction_hash: _instruction.transaction_hash,
                        program: _instruction.program,
                        timestamp: _instruction.timestamp
                    };

                    if response.contains(&key) {
                        response[&key].push(write_data);
                    } else {
                        response[&key] = vec![write_data];
                    }

                    Some(response)
                }
                LoaderInstruction::Finalize => {
                    // Option::from(InstructionSet {
                    //     function: InstructionFunction {
                    //         tx_instruction_id: _instruction.tx_instruction_id.clone(),
                    //         transaction_hash: _instruction.transaction_hash.clone(),
                    //         parent_index: _instruction.parent_index.clone(),
                    //         program: _instruction.program.clone(),
                    //         function_name: "finalize".to_string(),
                    //         timestamp: _instruction.timestamp.clone(),
                    //     },
                    //     properties: vec![],
                    // })
                    None
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