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

    return cmd;
}

fn cmd_processor(cmd: &str) {
   if cmd.starts_with('.')  {
        meta_cmd_processor(cmd);
    } else {
        sql_cmd_processor(cmd);
    }
}

fn sql_cmd_processor(cmd: &str) {
    let sql_statement = to_sql_statement(cmd);
    execute_sql(sql_statement);
}

fn to_sql_statement(raw_cmd: &str) -> SqlStatement {
    if raw_cmd.starts_with("insert") {
        return SqlStatement::Insert;
    }

    if raw_cmd.starts_with("select") {
        return SqlStatement::Select;
    }

    return SqlStatement::Unknown;
}

fn execute_sql(statement: SqlStatement) {
    match statement {
        SqlStatement::Select => {
            println!("Select from database");
        },
        SqlStatement::Insert => {
            println!("Inserting into database");
        },
        SqlStatement::Unknown => {
            println!("Unrecognized SQL syntax");
        }
    }
}

fn meta_cmd_processor(cmd: &str) {
    if cmd.trim() == ".exit" || cmd.trim() == "^D" {
        println!("Bye now ✌️");
        process::exit(0);
    } else {
        print!("Unrecognized command {}", cmd);
    }
}

enum SqlStatement {
    Select,
    Insert,
    Unknown,
}
