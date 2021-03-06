mod intcode;
mod intcode_error;
mod intcode_result;
mod operation;
mod operation_result;

pub use intcode::IntCode;
pub use intcode_error::{IntCodeError, IntCodeErrorKind};pub use intcode_result::{IntCodeResult, IntCodeResultKind};
