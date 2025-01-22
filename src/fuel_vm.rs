use fuel_vm::{
    interpreter::InterpreterParams,
    prelude::{
        Finalizable, GasCosts, Interpreter, IntoChecked, MemoryInstance, Script, TransactionBuilder,
    },
    state::ExecuteState,
    storage::MemoryStorage,
};

pub enum FuelVMExecutorError {
    Revert,
}

pub struct FuelVMExecutor {
    script: Vec<u8>,
}

impl FuelVMExecutor {
    pub fn new(script: Vec<u8>) -> Self {
        Self { script }
    }

    pub fn run_script(&mut self) -> Result<u64, FuelVMExecutorError> {
        let mut interpreter = Interpreter::<_, _, Script>::with_storage(
            MemoryInstance::new(),
            MemoryStorage::default(),
            InterpreterParams {
                gas_costs: GasCosts::free(),
                ..Default::default()
            },
        );

        let script = TransactionBuilder::script(self.script.clone(), vec![])
            .max_fee_limit(0)
            .add_fee_input()
            .finalize();

        let script = script
            .into_checked_basic(Default::default(), &Default::default())
            .unwrap();
        let script = script.test_into_ready();

        interpreter.init_script(script).unwrap();

        let result = loop {
            let script = interpreter.execute().unwrap();

            match script {
                ExecuteState::Return(result) => {
                    break result;
                }
                ExecuteState::Revert(result) => {
                    println!("revert with code {:?}", result);
                    return Err(FuelVMExecutorError::Revert);
                }
                _ => {}
            }
        };

        Ok(result)
    }
}
