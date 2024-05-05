//! 正規表現の式をパースし、抽象構文木に変換
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    mem::take,
};

mod elm {
    pub const PLUS: char = '+';
    pub const STAR: char = '*';
    pub const QUES: char = '?';
    pub const LPAR: char = '(';
    pub const RPAR: char = ')';
    pub const PIPE: char = '|';
    pub const BKSL: char = '\\';
}
/// パースエラーを表すための型
#[derive(Debug)]
pub enum ParseError {
    InvalidEscape(usize, char), // 誤ったエスケープシーケンス
    InvalidOr(usize, char),     // |の後に式がない
    InvalidRightParen(usize),   // 左開き括弧無し
    NoPrev(usize),              // +、|、*、?の前に式がない
    NoRightParen,               // 右閉じ括弧無し
    Empty,                      // 空のパターン
}

/// パースエラーを表示するために、Displayトレイトを実装
impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidEscape(pos, c) => {
                write!(f, "ParseError: invalid escape: pos = {pos}, char = '{c}'")
            }
            ParseError::InvalidOr(pos, c) => {
                write!(f, "ParseError: invalid or: pos = {pos}, char = '{c}'")
            }
            ParseError::InvalidRightParen(pos) => {
                write!(f, "ParseError: invalid right parenthesis: pos = {pos}")
            }
            ParseError::NoPrev(pos) => {
                write!(f, "ParseError: no previous expression: pos = {pos}")
            }
            ParseError::NoRightParen => {
                write!(f, "ParseError: no right parenthesis")
            }
            ParseError::Empty => write!(f, "ParseError: empty expression"),
        }
    }
}

impl Error for ParseError {} // エラー用に、Errorトレイトを実装

/// 抽象構文木を表現するための型
#[derive(Debug, PartialEq)]
pub enum AST {
    Char(char),
    Plus(Box<AST>),
    Star(Box<AST>),
    Question(Box<AST>),
    Or(Box<AST>, Box<AST>),
    Seq(Vec<AST>),
}
impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// parse_plus_star_question関数で利用するための列挙型
enum PSQ {
    Plus,
    Star,
    Question,
}

/// 正規表現を抽象構文木に変換
pub fn parse(expr: &str) -> Result<AST, ParseError> {
    // 内部状態を表現するための型
    // Char状態 : 文字列処理中
    // Escape状態 : エスケープシーケンス処理中
    enum ParseState {
        Char,
        Escape,
    }

    let mut seq: Vec<AST> = Vec::new(); // 現在のSeqのコンテキスト
    let mut seq_or: Vec<AST> = Vec::new(); // 現在のOrのコンテキスト
    let mut stack: Vec<(Vec<AST>, Vec<AST>)> = Vec::new(); // コンテキストのスタック
    let mut state: ParseState = ParseState::Char; // 現在の状態

    for (i, c) in expr.chars().enumerate() {
        match state {
            ParseState::Char => {
                match c {
                    elm::PLUS => parse_plus_star_question(&mut seq, PSQ::Plus, i)?,
                    elm::STAR => parse_plus_star_question(&mut seq, PSQ::Star, i)?,
                    elm::QUES => parse_plus_star_question(&mut seq, PSQ::Question, i)?,
                    elm::LPAR => {
                        let prev = take(&mut seq);
                        let prev_or = take(&mut seq_or);
                        stack.push((prev, prev_or));
                    }
                    elm::RPAR => {
                        if let Some((mut prev, mut prev_or)) = stack.pop() {
                            if !seq.is_empty() {
                                seq_or.push(AST::Seq(seq));
                            }

                            // Orを生成
                            if let Some(ast) = foldr(seq_or) {
                                prev.push(ast);
                            }

                            // 以前のコンテキストを、現在のコンテキストにする
                            seq = prev;
                            seq_or = prev_or;
                        } else {
                            let err = ParseError::InvalidRightParen(i);
                            return Err(err);
                        }
                    }
                    elm::PIPE => {
                        if seq.is_empty() {
                            return Err(ParseError::NoPrev(i));
                        } else {
                            let prev = take(&mut seq);
                            seq_or.push(AST::Seq(prev))
                        }
                    }
                    elm::BKSL => {
                        state = ParseState::Escape;
                    }
                    _ => {
                        seq.push(AST::Char(c));
                    }
                };
            }
            ParseState::Escape => {
                seq.push(parse_escape(i, c).unwrap());
                state = ParseState::Char;
            }
        }
    }
    if !seq.is_empty() {
        let prev = take(&mut seq);
        seq_or.push(AST::Seq(prev));
    }
    if let Some(ast) = foldr(seq_or) {
        Ok(ast)
    } else {
        Err(ParseError::Empty)
    }
}

/// +、*、?をASTに変換
///
/// 後置記法で、+、*、?の前にパターンがない場合はエラー
///
/// 例 : *ab、abc|+などはエラー
fn parse_plus_star_question(
    seq: &mut Vec<AST>,
    ast_type: PSQ,
    pos: usize,
) -> Result<(), ParseError> {
    if let Some(prev) = seq.pop() {
        let ast = match ast_type {
            PSQ::Plus => AST::Plus(Box::new(prev)),
            PSQ::Star => AST::Star(Box::new(prev)),
            PSQ::Question => AST::Question(Box::new(prev)),
        };
        seq.push(ast);
        Ok(())
    } else {
        Err(ParseError::NoPrev(pos))
    }
}

/// 特殊文字のエスケープ
fn parse_escape(pos: usize, c: char) -> Result<AST, ParseError> {
    match c {
        elm::PLUS | elm::STAR | elm::QUES | elm::PIPE | elm::LPAR | elm::RPAR | elm::BKSL => {
            Ok(AST::Char(c))
        }
        _ => {
            let err = ParseError::InvalidEscape(pos, c);
            Err(err)
        }
    }
}

/// orで結合された複数の式をASTに変換
///
/// たとえば、abc|def|ghi は、AST::Or("abc", AST::Or("def", "ghi"))というASTとなる
fn foldr(mut seq_or: Vec<AST>) -> Option<AST> {
    if seq_or.len() > 1 {
        // seq_orの要素が複数ある場合は、Orで式を結合
        let mut ast = seq_or.pop().unwrap();
        seq_or.reverse();
        for s in seq_or {
            ast = AST::Or(Box::new(s), Box::new(ast));
        }
        Some(ast)
    } else {
        // seq_orの要素が一つのみの場合は、Orではなく、最初の値を返す
        seq_or.pop()
    }
}
#[test]
fn test() {
    // Char
    assert_eq!(parse("a").unwrap(), AST::Seq(vec![AST::Char('a')]));
    // Plus
    assert_eq!(
        parse("a+").unwrap(),
        AST::Seq(vec![AST::Plus(Box::new(AST::Char('a')))])
    );
    assert_eq!(
        parse("aa+").unwrap(),
        AST::Seq(vec![AST::Char('a'), AST::Plus(Box::new(AST::Char('a')))])
    );
    assert_eq!(
        parse("a+a").unwrap(),
        AST::Seq(vec![AST::Plus(Box::new(AST::Char('a'))), AST::Char('a')])
    );
    // Star
    assert_eq!(
        parse("a*").unwrap(),
        AST::Seq(vec![AST::Star(Box::new(AST::Char('a')))])
    );
    assert_eq!(
        parse("aa*").unwrap(),
        AST::Seq(vec![AST::Char('a'), AST::Star(Box::new(AST::Char('a')))])
    );
    assert_eq!(
        parse("a*a").unwrap(),
        AST::Seq(vec![AST::Star(Box::new(AST::Char('a'))), AST::Char('a')])
    );
    // Question
    assert_eq!(
        parse("a?").unwrap(),
        AST::Seq(vec![AST::Question(Box::new(AST::Char('a')))])
    );
    assert_eq!(
        parse("aa?").unwrap(),
        AST::Seq(vec![AST::Char('a'), AST::Question(Box::new(AST::Char('a')))])
    );
    assert_eq!(
        parse("a?a").unwrap(),
        AST::Seq(vec![AST::Question(Box::new(AST::Char('a'))), AST::Char('a')])
    );
    // Or
    assert_eq!(
        parse("a|b").unwrap(),
        AST::Or(Box::new(AST::Seq(vec![AST::Char('a')])), 
                Box::new(AST::Seq(vec![AST::Char('b')])))
    );
    assert_eq!(
        parse("aa|b").unwrap(),
        AST::Or(Box::new(AST::Seq(vec![AST::Char('a'), AST::Char('a')])), 
                Box::new(AST::Seq(vec![AST::Char('b')])))
    );
    assert_eq!(
        parse("a|bb").unwrap(),
        AST::Or(Box::new(AST::Seq(vec![AST::Char('a')])), 
                Box::new(AST::Seq(vec![AST::Char('b'), AST::Char('b')])))
    );
    // Seq
    assert_eq!(
        parse("ab").unwrap(),
        AST::Seq(vec![AST::Char('a'), AST::Char('b')])
    );
    // parentheses
    assert_eq!(parse("(a)").unwrap(), AST::Seq(vec![AST::Seq(vec![AST::Char('a')])]));
    assert_eq!(parse("(a)b").unwrap(), AST::Seq(vec![AST::Seq(vec![AST::Char('a')]), AST::Char('b')]));
    assert_eq!(parse("a(b)").unwrap(), AST::Seq(vec![AST::Char('a'), AST::Seq(vec![AST::Char('b')])]));
    assert_eq!(parse("(ab)").unwrap(), AST::Seq(vec![AST::Seq(vec![AST::Char('a'), AST::Char('b')])]));
}
