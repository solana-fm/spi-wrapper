use serde::Serialize;
use crate::{Instruction, TableData};

pub const PROGRAM_ADDRESS: &str = "NativeLoader1111111111111111111111111111111";

#[derive(Serialize)]
pub enum NativeLoaderDatum {
    None
}

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    _instruction: Instruction
) -> Option<Vec<TableData>> {
    // We don't have anything to work with
    None
}