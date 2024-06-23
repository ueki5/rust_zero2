use nix::unistd::pipe;
// use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::io::{AsFd, BorrowedFd, OwnedFd};

fn main() {
    let mut input = None;
    let mut output = None;
    // パイプを作成
    let p = pipe().unwrap();
    input = Some(p.0);
    output = Some(p.1);
    let infd = input.unwrap();
    let binfd = infd.as_fd();
    println!("!!!");

    // ファイル操作
    let mut f = File::open("poem.txt").expect("file not found");
    // let f = File::open("poem.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("With text:\n{}", contents);


    // 生ハンドルとして借用
    {
        let raw: BorrowedFd = f.as_fd();
    }

    // 所有権を持つ生ハンドルに変換
    // let raw: OwnedFd = f.into();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("With text2:\n{}", contents);
    // ファイルはOwnedFd/OwnedHandleによって正常に閉じられる
}
