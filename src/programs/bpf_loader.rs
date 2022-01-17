use avro_rs::schema::Schema;
use bincode::deserialize;
use serde::Serialize;
use solana_sdk::loader_instruction::LoaderInstruction;
use tracing::error;

use crate::{Instruction, TableData, TypedDatum};

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

#[derive(Serialize)]
pub enum BpfLoaderDatum {
    NativeBpfWrite(NativeBpfWrite),
}

/// Struct tables
#[derive(Serialize)]
pub struct NativeBpfWrite {
    /// Which transaction was this?
    pub transaction_hash: String,
    /// Program Involved
    pub program: String,
    /// Wen exit?
    pub timestamp: i64,
}

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    _instruction: Instruction
) -> Option<Vec<TableData>> {
    let bpf_loader_dr = deserialize::<LoaderInstruction>(
        &_instruction.data);

    return match bpf_loader_dr {
        Ok(ref bld) => {
            let deserialized_bpf_loader = bld.clone();
            let mut response: Vec<TableData> = Vec::new();
            return match deserialized_bpf_loader {
                LoaderInstruction::Write { .. } => {
                    let native_bpf_write_table_data = TableData {
                        schema: (*NATIVE_BPF_LOADER_WRITE_SCHEMA).clone(),
                        table_name: NATIVE_BPF_LOADER_WRITE_TABLE_NAME.to_string(),
                        data: vec![TypedDatum::BpfLoader(BpfLoaderDatum::NativeBpfWrite(
                            NativeBpfWrite {
                                transaction_hash: _instruction.transaction_hash,
                                program: _instruction.program,
                                timestamp: _instruction.timestamp,
                            }))]
                    };

                    response.push(native_bpf_write_table_data);

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
            };
        }
        Err(err) => {
            // If the instruction parsing is failing, bail out
            error!("[spi-wrapper/bpf_loader] Attempt to parse instruction from program {} failed due to \
        {}.", _instruction.program, err);

            None
        }
    };
}