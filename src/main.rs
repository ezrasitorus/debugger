use std::{char::ToLowercase, io};

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
