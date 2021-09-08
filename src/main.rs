use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        let cmd = get_command();

        if cmd.trim() == ".exit" {
            process::exit(0);
        } else {
            print!("Unrecognized command {}", cmd);
        }
    }
}

fn get_command() -> String {
    print!("db > ");
    io::stdout().flush().expect("Failed to flush");

    let mut cmd = String::new();
    io::stdin().read_line(&mut cmd).expect("Failed to read line");

    return cmd;
}
