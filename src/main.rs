use nix::{
    sys::wait::waitpid,
    unistd::{fork, write, ForkResult},
};
use std::{env, io, process::Command, str::from_utf8};

enum Commands {
    Info { option: String },
    Continue,
    Breakpoint { line_number: u64 },
    Quit,
}

fn main() {
    let mut option: String = String::new();
    print_start();
    print_command_info();

    let cmd_args: Vec<String> = env::args().collect();

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!(
                "Continuing execution in parent process, new child has pid: {}",
                child
            );
            waitpid(child, None).unwrap();
            println!("Child has finished executing!");
            std::process::exit(0);
        }
        Ok(ForkResult::Child) => {
            // Unsafe to use `println!` (or `unwrap`) here. See Safety.
            write(libc::STDOUT_FILENO, "I'm a new child process\n".as_bytes()).ok();
            unsafe { libc::_exit(0) };
        }
        Err(_) => println!("Fork failed"),
    }

    let echo_child = Command::new(cmd_args[1].as_str())
        .arg("Hello World")
        .output()
        .expect("Failed to start echo process");

    println!(
        "{:?}",
        from_utf8(&echo_child.stdout).expect("Failed to read")
    );

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
}

fn print_start() {
    println!("Welcome to ezdb!");
}
fn print_command_info() {
    println!("Your options are: (q)uit, (r)un, (b)reakpoint, (i)nfo, (c)ontinue");
}
