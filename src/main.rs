use std::cell::RefCell;
use std::hint::black_box;
use std::process::exit;
use std::rc::Rc;
use std::time::Instant;

use fuel_vm::fuel_asm::op;
use fuel_vm::interpreter::InterpreterParams;
use fuel_vm::prelude::{Finalizable, GasCosts, IntoChecked, RegId, Script, TransactionBuilder};
use fuel_vm::state::ExecuteState;
use fuel_vm::storage::MemoryStorage;
use fuel_vm::{interpreter::Interpreter, prelude::MemoryInstance};
use revm::context::{BlockEnv, CfgEnv, TxEnv};
use revm::interpreter::interpreter::EthInterpreter;

use revm::interpreter::table::make_instruction_table;
use revm::interpreter::{DummyHost, InputsImpl, Interpreter as RevmInterpreter, SharedMemory};
use revm::primitives::{Address as RevmAddress, Bytes, U256 as RevmU256};
use revm::specification::hardfork::SpecId;
use revm::state::Bytecode;

fn main() {
    let mut interpreter = Interpreter::<_, _, Script>::with_storage(
        MemoryInstance::new(),
        MemoryStorage::default(),
        InterpreterParams {
            gas_costs: GasCosts::free(),
            ..Default::default()
        },
    );

    let a: u32 = 100000;
    let b: u32 = 10000;
    let number_of_iterations: u64 = (a * b).into();

    println!("number_of_iterations: {:?}", number_of_iterations);

    let reg_a = RegId::new(19);
    let reg_b = RegId::new(20);
    let reg_result = RegId::new(21);

    let script = TransactionBuilder::script(
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

    let start = Instant::now();

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

    let duration = start.elapsed();
    println!("addition result on FuelVM: {:?}", result);
    println!("Time elapsed on FuelVM: {:?}", duration);

    // convert number of iterations to bytes, little endian, padded to 8 bytes
    let number_of_iterations_bytes = number_of_iterations.to_be_bytes();
    // println!("number_of_iterations_bytes: {:?}", number_of_iterations_bytes);

    let bytecode = [
        0x67,
        number_of_iterations_bytes[0],
        number_of_iterations_bytes[1],
        number_of_iterations_bytes[2],
        number_of_iterations_bytes[3],
        number_of_iterations_bytes[4],
        number_of_iterations_bytes[5],
        number_of_iterations_bytes[6],
        number_of_iterations_bytes[7], // PUSH8 number_of_iterations
        0x60,
        0x00, // PUSH1 0x00
        0x5b, // JUMPDEST
        0x81, // DUP2
        0x15, // ISZERO
        0x60,
        0x1d, // PUSH1 0x1d
        0x57, // JUMPI
        0x60,
        0x01, // PUSH1 0x01
        0x01, // ADD
        0x90, // SWAP1
        0x60,
        0x01, // PUSH1 0x01
        0x90, // SWAP1
        0x03, // SUB
        0x90, // SWAP1
        0x60,
        0x0b, // PUSH1 0x0b
        0x56, // JUMP
        0x5b, // JUMPDEST
        0x90, // SWAP1
        0x50, // POP
    ]
    .to_vec();

    // reth side of the code
    // ---------
    // the bytecode needs to the following:
    // iterate a certain number of times, and keep adding 1 to the result which is initially zero
    let bytecode = Bytecode::new_raw(Bytes::from(bytecode));

    let mut interpreter = RevmInterpreter::<EthInterpreter>::new(
        Rc::new(RefCell::new(SharedMemory::new())),
        bytecode,
        InputsImpl {
            target_address: RevmAddress::ZERO,
            caller_address: RevmAddress::ZERO,
            input: Bytes::default(),
            call_value: RevmU256::ZERO,
        },
        false,
        false,
        SpecId::LATEST,
        u64::MAX,
    );

    let mut host = DummyHost::<BlockEnv, TxEnv, CfgEnv>::default();

    let table = make_instruction_table::<EthInterpreter, DummyHost<BlockEnv, TxEnv, CfgEnv>>();

    let start = Instant::now();
    interpreter.run(&table, &mut host);
    let duration = start.elapsed();
    println!("Time elapsed on rEVM: {:?}", duration);

    println!("result on rEVM: {:?}", interpreter.stack.pop());
}
