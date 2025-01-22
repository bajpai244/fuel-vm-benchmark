/// This benchmark is for benchmarking the performance of addition in loops in FuelVM and rEVM
/// The bytecode for both is to implement a counter, that counts from 0 to n
use criterion::{criterion_group, criterion_main, Criterion};

use fuel_vm_benchmark::bytecode::evm::counter_builder_evm;
use fuel_vm_benchmark::bytecode::fuel::counter_builder_fuel_vm;
use fuel_vm_benchmark::evm::EVMExecutor;
use fuel_vm_benchmark::fuel_vm::FuelVMExecutor;

fn bench_counter(c: &mut Criterion) {
    let input_a = 100;
    let input_b = 100;
    let number_of_iterations: u64 = (input_a * input_b).into();

    let mut group = c.benchmark_group("counter");

    group.bench_function("EVM", |b| {
        b.iter(|| {
            let evm_bytecode = counter_builder_evm(number_of_iterations);
            let evm_executor = EVMExecutor::new(evm_bytecode);

            let _ = evm_executor.run().unwrap();
        });
    });

    group.bench_function("FuelVM", |b| {
        b.iter(|| {
            let fuel_vm_counter_bytecode = counter_builder_fuel_vm(input_a, input_b);
            let mut fuel_vm_executor = FuelVMExecutor::new(fuel_vm_counter_bytecode);

            let _ = fuel_vm_executor.run_script().unwrap();
        });
    });

    group.finish();
}

criterion_group!(benches, bench_counter);
criterion_main!(benches);
