use fuel_vm::{fuel_asm::op, prelude::RegId};

/// FuelVM script for counter, counts from 0 to n
/// number of iterations is a * b
pub fn counter_builder_fuel_vm(a: u32, b: u32) -> Vec<u8> {
    let reg_a = RegId::new(19);
    let reg_b = RegId::new(20);
    let reg_result = RegId::new(21);

    vec![
        op::movi(reg_a, a),
        op::movi(reg_b, b),
        op::mul(RegId::WRITABLE, reg_a, reg_b),
        op::addi(reg_result, RegId::ZERO, 0),
        op::addi(reg_result, reg_result, 1),
        op::subi(RegId::WRITABLE, RegId::WRITABLE, 1),
        op::jnzb(RegId::WRITABLE, RegId::ONE, 0),
        op::ret(reg_result),
    ]
    .into_iter()
    .collect()
}
