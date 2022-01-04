use std::collections::HashMap;
use avro_rs::Schema;
use serde::Serialize;
use crate::{Instruction, InstructionSet};

pub const PROGRAM_ADDRESS: &str = "NativeLoader1111111111111111111111111111111";

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction<T: Serialize>(
    // The instruction
    _instruction: Instruction,
) -> Option<HashMap<(String, Schema), Vec<T>>> {
    // We don't have anything to work with
    None
}