use std::time::Instant;

use fuel_vm_benchmark::bytecode::evm::counter_builder_evm;
use fuel_vm_benchmark::bytecode::fuel::counter_builder_fuel_vm;
use fuel_vm_benchmark::evm::EVMExecutor;
use fuel_vm_benchmark::fuel_vm::FuelVMExecutor;

// This binary runs the fuelVM and rEVM over loops of different sizes to perform addition
// both the VMs are running handwritten bytecode to avoid language compiler optimization issues
// It dumps the data into JSON file for plotting
fn main() {
    let a = 100;
    let b = 100;

    let fuel_vm_counter_bytecode = counter_builder_fuel_vm(a, b);
    let mut fuel_vm_executor = FuelVMExecutor::new(fuel_vm_counter_bytecode);

    let start = Instant::now();

    let result = fuel_vm_executor.run_script().unwrap();

    let duration = start.elapsed();
    println!("addition result on FuelVM: {:?}", result);
    println!("Time elapsed on FuelVM: {:?}", duration);

    // reth side of the code

    let number_of_iterations: u64 = (a * b).into();

    let evm_bytecode = counter_builder_evm(number_of_iterations);
    let evm_executor = EVMExecutor::new(evm_bytecode);

    let start = Instant::now();
    let result = evm_executor.run().unwrap();
    let duration = start.elapsed();

    println!("result on rEVM: {:?}", result);
    println!("Time elapsed on rEVM: {:?}", duration);
}
