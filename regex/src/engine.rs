//! 正規表現エンジン
pub mod parser;

use crate::helper::DynError;
use std::fmt::{self, Display};

pub fn do_matching(regex: &parser::AST, input: &str, switch: bool) -> Result<bool, DynError> {
    Ok(true)
}
