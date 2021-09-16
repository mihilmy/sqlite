mod store;

use regex::Regex;
use std::io::{self, Write};
use std::process;
use store::{Row, Table, TABLE_MAX_ROWS};

fn main() {
    let mut table = Table {
        rows: Vec::with_capacity(TABLE_MAX_ROWS),
    };
    loop {
        let cmd = get_command();
        cmd_processor(&cmd, &mut table);
    }
}

fn get_command() -> String {
    print!("db > ");
    io::stdout().flush().expect("Failed to flush");

    let mut cmd = String::new();
    io::stdin()
        .read_line(&mut cmd)
        .expect("Failed to read line");

    return cmd.trim().to_string();
}

fn cmd_processor(cmd: &str, table: &mut Table) {
    if cmd.starts_with('.') {
        meta_cmd_processor(cmd);
    } else {
        sql_cmd_processor(cmd, table);
    }
}

fn sql_cmd_processor(cmd: &str, table: &mut Table) {
    let sql_processors = [exec_insert, exec_select];
    for exec_fn in sql_processors {
        let success = exec_fn(cmd, table);

        if success {
            return ();
        }
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

fn exec_insert(cmd: &str, table: &mut Table) -> bool {
    let insert_regex =
        Regex::new(r"^(?i)insert (?P<id>\d+) (?P<username>\w+) (?P<email>.+)$").unwrap();
    return match insert_regex.captures(cmd) {
        Some(captured_groups) => {
            table.insert_row(Row::from(captured_groups));

            true
        }
        None => false,
    };
}

fn exec_select(_cmd: &str, table: &mut Table) -> bool {
    table.list_rows();
    true
}
