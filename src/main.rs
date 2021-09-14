use regex::Regex;
use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        let cmd = get_command();
        cmd_processor(&cmd);
    }
}

fn get_command() -> String {
    print!("db > ");
    io::stdout().flush().expect("Failed to flush");

    let mut cmd = String::new();
    io::stdin().read_line(&mut cmd).expect("Failed to read line");

    return cmd.trim().to_string();
}

fn cmd_processor(cmd: &str) {
   if cmd.starts_with('.')  {
        meta_cmd_processor(cmd);
    } else {
        sql_cmd_processor(cmd);
    }
}

fn sql_cmd_processor(cmd: &str) {
    let sql_processors = [exec_insert, exec_select];
    for exec_fn in sql_processors {
        let success = exec_fn(cmd);

        if success { return (); }
    }

    println!("Unrecognized SQL syntax");
}

fn meta_cmd_processor(cmd: &str) {
    if cmd == ".exit" || cmd == "^D" {
        println!("Bye now ✌️");
        process::exit(0);
    } else {
        print!("Unrecognized command {}", cmd);
    }
}

fn exec_insert(cmd: &str) -> bool {
    let insert_regex = Regex::new(r"^(?i)insert (?P<id>\d+) (?P<username>\w+) (?P<email>.+)$").unwrap();
    
    return match insert_regex.captures(cmd) {
        Some(captured_groups) => {
            let id = captured_groups.name("id").unwrap().as_str();
            let username = captured_groups.name("username").unwrap().as_str();
            let email = captured_groups.name("email").unwrap().as_str();

            println!("Inserting user with id={}, username={}, email={}", id, username, email);
            true
        },
        None => false,
    };
}
fn exec_select(_cmd: &str) -> bool {
    println!("selecting");

    false
}
