use nix::{
    sys::ptrace,
    unistd::{fork, ForkResult},
};
use std::env;

mod common;
mod debuggee;
mod debugger;

fn main() {
    print_start();
    common::print_command_info();

    let cmd_args: Vec<String> = env::args().collect();
    assert!(cmd_args.len() == 2);

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            debugger::debugger(child);
        }
        Ok(ForkResult::Child) => {
            ptrace::traceme().unwrap();
            debuggee::debuggee(cmd_args[1].as_str());
        }
        Err(_) => println!("Fork failed"),
    }
}

fn print_start() {
    println!("Welcome to ezdb!");
}
