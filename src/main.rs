use std::hint::black_box;
use std::process::exit;

use fuel_vm::fuel_asm::{op, RawInstruction};
use fuel_vm::interpreter::InterpreterParams;
use fuel_vm::prelude::{
    Finalizable, GasCosts, Instruction, IntoChecked, RegId, Script, TransactionBuilder,
};
use fuel_vm::state::ExecuteState;
use fuel_vm::storage::MemoryStorage;
use fuel_vm::{interpreter::Interpreter, prelude::MemoryInstance};

fn main() {
    let mut interpreter = Interpreter::<_, _, Script>::with_storage(
        MemoryInstance::new(),
        MemoryStorage::default(),
        InterpreterParams {
            gas_costs: GasCosts::free(),
            ..Default::default()
        },
    );

    let number_of_iterations = 1000;
    let result_register = RegId::new(19);

    let script = TransactionBuilder::script(
        vec![
            op::addi(RegId::WRITABLE, RegId::ZERO, number_of_iterations),
            op::addi(result_register, RegId::ZERO, 0),
            op::addi(result_register, result_register, 1),
            op::subi(RegId::WRITABLE, RegId::WRITABLE, 1),
            op::jnzb(RegId::WRITABLE, RegId::ONE, 0),
            op::ret(result_register),
        ]
        .into_iter()
        .collect(),
        vec![],
    )
    .max_fee_limit(0)
    .add_fee_input()
    .finalize();

    let script = script
        .into_checked_basic(Default::default(), &Default::default())
        .unwrap();
    let script = script.test_into_ready();
    black_box(interpreter.init_script(script)).unwrap();

    // Instruction set pointer
    let is = interpreter.registers().get(12).unwrap().clone();
    println!("{:?}", is);

    // Instruction set pointer
    let pc = interpreter.registers().get(3).unwrap().clone();
    println!("{:?}", pc);

    // let's read value there
    let value: [u8; 4] = interpreter.memory().read_bytes(is).unwrap();
    println!("value at memory {:?}", value);

    let instruction = Instruction::try_from(value).unwrap();
    println!("value at memory {:?}", instruction);

    let value: [u8; 4] = interpreter.memory().read_bytes(is + 4).unwrap();
    println!("value at memory {:?}", value);

    let instruction = Instruction::try_from(value).unwrap();
    println!("value at memory {:?}", instruction);

    let result = loop {
        let script = interpreter.execute().unwrap();

        match script {
            ExecuteState::Return(result) => {
                break result;
            }
            ExecuteState::Revert(result) => {
                println!("revert with code {:?}", result);
                exit(1);
            }
            _ => {}
        }
    };

    println!("result: {:?}", result);
}
