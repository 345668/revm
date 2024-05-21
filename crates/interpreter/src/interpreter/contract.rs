use std::sync::Arc;
use super::analysis::{to_analysed, BytecodeLocked};
use crate::primitives::{Bytecode, Bytes, B160, U256};
use crate::CallContext;
use revm_primitives::{Env, TransactTo};

#[derive(Clone, Default)]
pub struct Contract {
    /// Contracts data
    pub input: Bytes,
    /// Bytecode contains contract code, size of original code, analysis with gas block and jump table.
    /// Note that current code is extended with push padding and STOP at end.
    pub bytecode: Arc<BytecodeLocked>,
    /// Contract address
    pub address: B160,
    /// Caller of the EVM.
    pub caller: B160,
    /// Value send to contract.
    pub value: U256,

    pub code_address: B160,
}

impl Contract {
    pub fn new(input: Bytes, bytecode: Bytecode, address: B160, code_address: B160, caller: B160, value: U256) -> Self {
        let bytecode = Arc::new(to_analysed(bytecode).try_into().expect("it is analyzed"));

        Self {
            input,
            code_address,
            bytecode,
            address,
            caller,
            value,
        }
    }


    pub fn is_valid_jump(&self, possition: usize) -> bool {
        self.bytecode.jump_map().is_valid(possition)
    }

    pub fn new_with_context(input: Bytes, bytecode: Bytecode, call_context: &CallContext) -> Self {
        Self::new(
            input,
            bytecode,
            call_context.address,
            call_context.code_address,
            call_context.caller,
            call_context.apparent_value,
        )
    }


    pub fn new_with_context_analyzed(input: Bytes, bytecode: Arc<BytecodeLocked>, call_context: &CallContext) -> Self {
        Self {
            input,
            bytecode,
            code_address: call_context.code_address,
            address: call_context.address,
            caller: call_context.caller,
            value: call_context.apparent_value,
        }
    }
}
