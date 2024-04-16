//! 命令列と入力文字列を受け取り、マッチングを行う
use super::Instruction;
use crate::helper::safe_add;
use std::{
    collections::VecDeque,
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum EvalError {
    PCOverFlow,
    SPOverFlow,
    InvalidPC,
    InvalidContext,
}

impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodeGenError: {:?}", self)
    }
}

impl Error for EvalError {}

/// 命令列の評価を行う関数。
///
/// instが命令列となり、その命令列を用いて入力文字列lineにマッチさせる。
/// is_depthがtrueの場合に深さ優先探索を、falseの場合に幅優先探索を行う。
///
/// 実行時エラーが起きた場合はErrを返す。
/// マッチ成功時はOk(true)を、失敗時はOk(false)を返す。
pub fn eval(insts: &[Instruction], line: &[char], is_depth: bool) -> Result<bool, EvalError> {
    _eval(insts, line, 0, 0)
}
fn _eval(insts: &[Instruction], line: &[char], pc: usize, sp: usize) -> Result<bool, EvalError> {
    match insts[pc] {
        Instruction::Char(c) if c == line[sp] => _eval(insts, line, pc + 1, sp + 1),
        Instruction::Char(_) => return Ok(true), // 後でやる
        Instruction::Match => {
            return Ok(true);
        }
        Instruction::Jump(pc1) => _eval(insts, line, pc1, sp),
        Instruction::Split(pc1, pc2) => {
            let r1 = _eval(insts, line, pc1, sp)?;
            let r2 = _eval(insts, line, pc2, sp)?;
            return Ok(r2);
        }
    };
    Ok(true)
}
