use nix::{sys::wait::waitpid, unistd::Pid};
use std::{io, process::exit};

use crate::common;

pub fn debugger(child: Pid) {
    println!(
        "Continuing execution in parent process, new child has pid: {}",
        child
    );

    let mut option: String = String::new();

    loop {
        option.clear();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let argv: Vec<&str> = option.trim().split(' ').collect();
        let argc = argv.len();

        if argc != 0 {
            match argv[0].to_lowercase().as_str() {
                "q" | "quit" => {
                    println!("You've picked quit.");
                    break;
                }
                "r" | "run" => println!("You've picked run."),
                "i" | "info" => println!("You've picked info."),
                "b" | "breakpoint" => println!("You've picked breakpoint."),
                "c" | "continue" => println!("You've picked continue."),
                "" => println!(""),
                _ => {
                    println!("Invalid option");
                    common::print_command_info();
                }
            }
        }
    }

    waitpid(child, None).unwrap();
    println!("Child has finished executing!");
    exit(0);
}
