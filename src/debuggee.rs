use nix::unistd::write;
use std::process::{exit, Command};
use libc;

pub fn debuggee(program_name: &str) {
    write(libc::STDOUT_FILENO, "I'm a new child process\n".as_bytes()).ok();
    let echo_child = Command::new(program_name)
        .arg("Ezra was here")
        .output()
        .expect("Failed to start echo process");

    write(libc::STDOUT_FILENO, &echo_child.stdout).ok();

    exit(0);
}
