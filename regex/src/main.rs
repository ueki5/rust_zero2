mod engine;
mod helper;

use clap::{Parser, ValueEnum};
use engine::{do_matching, print};
use helper::DynError;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    fmt::{Debug, Display, Formatter},
};

/// 正規表現を評価する
#[derive(Parser)]
struct Args {
    /// 検索パターン
    #[arg(short, long)]
    regex: String,
    /// 入力ファイル
    #[arg(short, long)]
    input: String,
    /// 深さ優先探索
    #[arg(short, long, value_enum, default_value_t = SearchMethod::Dfs, help = "Search Method")]
    method: SearchMethod,
}
#[derive(Debug, Clone, ValueEnum, PartialEq)]
enum SearchMethod {
    Dfs,
    Bfs,
}

impl Display for SearchMethod {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let raw = format!("{:?}", self);
        write!(f, "{}", raw)
    }
}
fn main() -> Result<(), DynError> {
    let args = Args::parse();
    let is_depth = if args.method == SearchMethod::Dfs {
        true
    } else {
        false
    };
    match_file(&args.regex, &args.input, is_depth)?;
    Ok(())
}

/// ファイルをオープンし、行ごとにマッチングを行う。
///
/// マッチングはそれぞれの行頭から1文字ずつずらして行い、
/// いずれかにマッチした場合に、その行がマッチしたものとみなす。
///
/// たとえば、abcdという文字列があった場合、以下の順にマッチが行われ、
/// このいずれかにマッチした場合、与えられた正規表現にマッチする行と判定する。
///
/// - abcd
/// - bcd
/// - cd
/// - d
fn match_file(expr: &str, input: &str, breadth: bool) -> Result<(), DynError> {
    let f = File::open(input)?;
    let reader = BufReader::new(f);

    engine::print(expr)?;
    // ファイルを読み込み
    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        for (i, _) in line.char_indices() {
            if engine::do_matching(expr, &line[i..], breadth)?.len() > 0 {
                println!("line={idx}:{line}");
                break;
            }
        }
    }
    Ok(())
}
#[test]
fn test() {
    _test(true);
    _test(false);
}
fn _test(is_depth: bool) -> () {
    // char
    assert_eq!(engine::do_matching("a", "a", is_depth).unwrap(), String::from("a"));
    // plus
    assert_eq!(engine::do_matching("a+", "a", is_depth).unwrap(), String::from("a"));
    assert_eq!(engine::do_matching("a+", "aa", is_depth).unwrap(), String::from("aa"));
    // star
    assert_eq!(engine::do_matching("a*", "", is_depth).unwrap(), String::from(""));
    assert_eq!(engine::do_matching("a*", "a", is_depth).unwrap(), String::from("a"));
    assert_eq!(engine::do_matching("a*", "aa", is_depth).unwrap(), String::from("aa"));
    // or
    assert_eq!(engine::do_matching("a|b", "a", is_depth).unwrap(), String::from("a"));
    assert_eq!(engine::do_matching("a|b", "b", is_depth).unwrap(), String::from("b"));
    assert_eq!(engine::do_matching("a|b|c", "c", is_depth).unwrap(), String::from("c"));
}