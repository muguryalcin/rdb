use std::io::{self, Write};

enum MetaCommands {
    Exit,
    Other,
}
enum Commands {
    Insert,
    Select,
    Unrecognized,
}

enum CommandsSuccess {
    Success,
    Unrecognized,
}

struct Statement {
    statement_type: Commands,
}

fn parse_command(input: &str, statement: &mut Statement) -> CommandsSuccess {
    if input.starts_with("insert") == true {
        statement.statement_type = Commands::Insert;
        return CommandsSuccess::Success;
    }
    if input.starts_with("select") == true {
        statement.statement_type = Commands::Select;
        return CommandsSuccess::Success;
    }
    CommandsSuccess::Unrecognized
}

fn parse_meta_command(input: &str) -> MetaCommands {
    match input {
        ".exit" => MetaCommands::Exit,
        _ => MetaCommands::Other,
    }
}

fn execute_command(statement: &Statement) {
    match statement.statement_type {
        Commands::Insert => {
            println!("Insert")
        }
        Commands::Select => {
            println!("Select")
        }
        _ => println!("Unrecognized Command Error."),
    }
}
fn main() {
    loop {
        let mut statement = Statement {
            statement_type: Commands::Unrecognized,
        };
        let mut input = String::new();

        print!("db > ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the command.");
        input = input.trim().to_lowercase().to_string();
        if input.is_empty() {
            println!("Please enter non-empty commands.");
            continue;
        } else if input.chars().nth(0) == Some('.') {
            match parse_meta_command(&input) {
                MetaCommands::Exit => break,
                MetaCommands::Other => println!("{} is not a valid meta command.", input),
            }
        } else {
            match parse_command(&input, &mut statement) {
                CommandsSuccess::Success => execute_command(&statement),
                CommandsSuccess::Unrecognized => {
                    println!("Unrecognized command {}", input);
                    continue;
                }
            }
        }
    }
}
