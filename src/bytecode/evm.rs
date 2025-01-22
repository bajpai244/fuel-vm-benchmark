/// EVM bytecode for counter, counts from 0 to n
pub fn counter_builder_evm(number_of_iterations: u64) -> Vec<u8> {
    let number_of_iterations_bytes = number_of_iterations.to_be_bytes();

    // Starts with result = 0, add 1 to it `number_of_iterations` times
    let bytecode = [
        0x67, // PUSH8 number_of_iterations
        number_of_iterations_bytes[0],
        number_of_iterations_bytes[1],
        number_of_iterations_bytes[2],
        number_of_iterations_bytes[3],
        number_of_iterations_bytes[4],
        number_of_iterations_bytes[5],
        number_of_iterations_bytes[6],
        number_of_iterations_bytes[7],
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

    bytecode
}
