mod contract;
mod contract_store;
mod interface;
mod runtime;

pub use contract::{Contract, ContractKey, ContractValue};
pub use contract_store::ContractStore;
pub use interface::{ExecError, RuntimeInterface};
pub use runtime::Runtime;

type RuntimeResult<T> = std::result::Result<T, ContractRuntimeError>;

#[derive(thiserror::Error, Debug)]
pub enum ContractRuntimeError {
    #[error("contract {0} not found in store")]
    ContractNotFound(ContractKey),

    #[error(transparent)]
    ExecError(#[from] ExecError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("failed while unwrapping contract to raw bytes")]
    UnwrapContract,

    #[error(transparent)]
    WasmCompileError(#[from] wasmer::CompileError),

    #[error(transparent)]
    WasmExportError(#[from] wasmer::ExportError),

    #[error(transparent)]
    WasmInstantiationError(#[from] wasmer::InstantiationError),

    #[error(transparent)]
    WasmMemError(#[from] wasmer::MemoryError),

    #[error(transparent)]
    WasmRtError(#[from] wasmer::RuntimeError),
}