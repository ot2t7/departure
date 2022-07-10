mod instr;
mod parse;

pub use instr::*;
pub use parse::{deserialize, ParserError, to_bytecode};