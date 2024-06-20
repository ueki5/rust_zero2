use nix::unistd::pipe;
use std::fs::File;
#[cfg(unix)]
use std::os::unix::io::{AsFd, BorrowedFd, OwnedFd};
#[cfg(windows)]
use std::os::windows::io::{AsHandle, BorrowedHandle, OwnedHandle};

fn main() {
    let mut input = None;
    let mut output = None;
    // パイプを作成
    let p = pipe().unwrap();
    input = Some(p.0);
    output = Some(p.1);
    println!("!!!");

    let f = File::open("hoge.txt").unwrap();

    // 生ハンドルとして借用
    {
        let raw: BorrowedFd = f.as_fd();
    }

    // 所有権を持つ生ハンドルに変換
    let raw: OwnedFd = f.into();

    // ファイルはOwnedFd/OwnedHandleによって正常に閉じられる
}
