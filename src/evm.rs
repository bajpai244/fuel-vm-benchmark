use std::{cell::RefCell, rc::Rc};

use revm::{
    context::{BlockEnv, CfgEnv, TxEnv},
    interpreter::{
        interpreter::EthInterpreter, table::make_instruction_table, DummyHost, InputsImpl,
        Interpreter, SharedMemory,
    },
    primitives::{Address, Bytes, U256},
    specification::hardfork::SpecId,
    state::Bytecode,
};

pub enum EVMExecutorError {}

pub struct EVMExecutor {
    bytecode: Vec<u8>,
}

impl EVMExecutor {
    pub fn new(bytecode: Vec<u8>) -> Self {
        Self { bytecode }
    }

    // NOTE: We are for now assuming that result is at the top of the stack
    // TODO: This needs to be fixed, and use of memory should be preferred
    pub fn run(&self) -> Result<U256, EVMExecutorError> {
        let bytecode = Bytecode::new_raw(Bytes::from(self.bytecode.clone()));

        let mut interpreter = Interpreter::<EthInterpreter>::new(
            Rc::new(RefCell::new(SharedMemory::new())),
            bytecode,
            InputsImpl {
                target_address: Address::ZERO,
                caller_address: Address::ZERO,
                input: Bytes::default(),
                call_value: U256::ZERO,
            },
            false,
            false,
            SpecId::LATEST,
            u64::MAX,
        );

        let mut host = DummyHost::<BlockEnv, TxEnv, CfgEnv>::default();

        let table = make_instruction_table::<EthInterpreter, DummyHost<BlockEnv, TxEnv, CfgEnv>>();

        interpreter.run(&table, &mut host);

        Ok(interpreter.stack.pop().unwrap())
    }
}
