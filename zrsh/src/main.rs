// 確認用
use nix::{
    libc,
    sys::{
        signal::{killpg, signal, SigHandler, Signal},
        wait::{waitpid, WaitPidFlag, WaitStatus},
    },
    unistd::{self, dup2, execvp, fork, pipe, setpgid, tcgetpgrp, tcsetpgrp, ForkResult, Pid},
};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ffi::CString,
    fs::File,
    mem::replace,
    path::Path,
    path::PathBuf,
    process::exit,
    sync::mpsc::{channel, sync_channel, Receiver, Sender, SyncSender},
    thread,
    io,
};
use std::os::fd::AsFd;
use std::os::fd::AsRawFd;
// 確認用
mod helper;
mod shell;

use helper::DynError;

const HISTORY_FILE: &str = ".zerosh_history";

fn main() -> Result<(), DynError> {
    // 確認用
    let stdin = io::stdin(); // We get `Stdin` here.
    let raw_fd = stdin.as_fd().as_raw_fd();
    let res = unsafe { libc::tcgetpgrp(raw_fd) };
    let res2 = unsafe { libc::tcgetpgrp(libc::STDOUT_FILENO) };
    let pid = unsafe { libc::getppid()};
    let res3 = unsafe { libc::getpgid(pid)};
    // 確認用

    let mut logfile = HISTORY_FILE;
    let mut home = dirs::home_dir();
    if let Some(h) = &mut home {
        h.push(HISTORY_FILE);
        logfile = h.to_str().unwrap_or(HISTORY_FILE);
    }

    let sh = shell::Shell::new(logfile);
    sh.run()?;

    Ok(())
}
