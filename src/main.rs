use nix::{
    sys::{ptrace, wait::waitpid},
    unistd::{fork, write, ForkResult, Pid},
};
use std::{
    env, io,
    process::{exit, Command},
};

enum Commands {
    Info { option: String },
    Continue,
    Breakpoint { line_number: u64 },
    Quit,
}

fn main() {
    print_start();
    print_command_info();

    let cmd_args: Vec<String> = env::args().collect();
    assert!(cmd_args.len() == 2);

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            debugger(child);
        }
        Ok(ForkResult::Child) => {
            ptrace::traceme().unwrap();
            debuggee(cmd_args[1].as_str());
        }
        Err(_) => println!("Fork failed"),
    }
}

fn print_start() {
    println!("Welcome to ezdb!");
}
fn print_command_info() {
    println!("Your options are: (q)uit, (r)un, (b)reakpoint, (i)nfo, (c)ontinue");
}

fn debugger(child: Pid) {
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
                    print_command_info();
                }
            }
        }
    }

    waitpid(child, None).unwrap();
    println!("Child has finished executing!");
    exit(0);
}

fn debuggee(program_name: &str) {
    write(libc::STDOUT_FILENO, "I'm a new child process\n".as_bytes()).ok();
    let echo_child = Command::new(program_name)
        .arg("Ezra was here")
        .output()
        .expect("Failed to start echo process");

    write(libc::STDOUT_FILENO, &echo_child.stdout).ok();

    exit(0);
}
