enum Commands {
    Info { option: String },
    Continue,
    Breakpoint { line_number: u64 },
    Quit,
}

pub fn print_command_info() {
    println!("Your options are: (q)uit, (r)un, (b)reakpoint, (i)nfo, (c)ontinue");
}
