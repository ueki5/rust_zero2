use nix::unistd::pipe;
fn main() {
    let mut input = None;
    let mut output = None;
    // パイプを作成
    let p = pipe().unwrap();
    input = Some(p.0);
    output = Some(p.1);
}
