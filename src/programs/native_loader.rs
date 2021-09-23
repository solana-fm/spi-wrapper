use crate::{Instruction, InstructionSet};

pub const PROGRAM_ADDRESS: String = "NativeLoader1111111111111111111111111111111".parse().unwrap();

/// Extracts the contents of an instruction into small bits and pieces, or what we would call,
/// instruction_properties.
///
/// The function should return a list of instruction properties extracted from an instruction.
pub async fn fragment_instruction(
    // The instruction
    _instruction: Instruction,
) -> Option<InstructionSet> {
    // We don't have anything to work with
    None
}