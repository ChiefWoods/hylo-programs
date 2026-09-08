#![cfg(feature = "test-sbf")]

use {
    anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
    mollusk_svm::{result::Check, Mollusk},
};

#[test]
fn test_initialize() {
    let program_id = hylo_programs::id();

    let mollusk = Mollusk::new(&program_id, "hylo_programs");

    let instruction = Instruction::new_with_bytes(
        program_id,
        &hylo_programs::instruction::Initialize {}.data(),
        hylo_programs::accounts::Initialize {}.to_account_metas(None),
    );

    mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
}
