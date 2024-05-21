use crate::{
    gas, interpreter::Interpreter, primitives::Spec, primitives::SpecId::*, Host, InstructionResult,
};

pub fn chainid<T, SPEC: Spec>(interpreter: &mut Interpreter, host: &mut dyn Host<T>) {
    // EIP-1344: ChainID opcode
    check!(interpreter, SPEC::enabled(ISTANBUL));
    gas!(interpreter, gas::BASE);
    push!(interpreter, host.env().cfg.chain_id);
}

pub fn coinbase<T>(interpreter: &mut Interpreter, host: &mut dyn Host<T>) {
    gas!(interpreter, gas::BASE);
    push_b256!(interpreter, host.env().block.coinbase.into());
}

pub fn timestamp<T>(interpreter: &mut Interpreter, host: &mut dyn Host<T>) {
    gas!(interpreter, gas::BASE);
    push!(interpreter, host.env().block.timestamp);
}

pub fn number<T>(interpreter: &mut Interpreter, host: &mut dyn Host<T>) {
    gas!(interpreter, gas::BASE);
    push!(interpreter, host.env().block.number);
}

pub fn difficulty<T, H: Host<T>, SPEC: Spec>(interpreter: &mut Interpreter, host: &mut H) {
    gas!(interpreter, gas::BASE);
    if SPEC::enabled(MERGE) {
        push_b256!(interpreter, host.env().block.prevrandao.unwrap());
    } else {
        push!(interpreter, host.env().block.difficulty);
    }
}

pub fn gaslimit<T>(interpreter: &mut Interpreter, host: &mut dyn Host<T>) {
    gas!(interpreter, gas::BASE);
    push!(interpreter, host.env().block.gas_limit);
}

pub fn gasprice<T>(interpreter: &mut Interpreter, host: &mut dyn Host<T>) {
    gas!(interpreter, gas::BASE);
    push!(interpreter, host.env().effective_gas_price());
}

pub fn basefee<T, SPEC: Spec>(interpreter: &mut Interpreter, host: &mut dyn Host<T>) {
    gas!(interpreter, gas::BASE);
    // EIP-3198: BASEFEE opcode
    check!(interpreter, SPEC::enabled(LONDON));
    push!(interpreter, host.env().block.basefee);
}

pub fn origin<T>(interpreter: &mut Interpreter, host: &mut dyn Host<T>) {
    gas!(interpreter, gas::BASE);
    push_b256!(interpreter, host.env().tx.caller.into());
}
