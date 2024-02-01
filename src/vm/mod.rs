mod debugger;
mod instructions;
mod intreg;

pub mod decompiler;
pub mod run;
// Access to register and storage is needed for patching the binary
pub mod register;
pub mod storage;
pub mod terminal;
