mod engine;
mod helper;

use clap::Parser;
use engine::{do_matching, print};
use helper::DynError;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

/// 正規表現評価
#[derive(Parser)]
struct Args {
    /// 検索パターン
    #[arg(short, long)]
    regex: String,
    /// 入力ファイル
    #[arg(short, long)]
    input: String,
}

fn main() -> Result<(), DynError> {
    let args = Args::parse();
    match_file(&args.regex, &args.input)?;
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
fn match_file(expr: &str, input: &str) -> Result<(), DynError> {
    let f = File::open(input)?;
    let reader = BufReader::new(f);

    engine::print(expr)?;

    // ファイルを読み込み
    for line in reader.lines() {
        let line = line?;
        for (i, _) in line.char_indices() {
            if engine::do_matching(expr, &line[i..], true)? {
                println!("{line}");
                break;
            }
        }
    }
    Ok(())
}
