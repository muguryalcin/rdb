use std::io::{self, Write};
mod table;
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
    Error,
    Unrecognized,
}
struct Statement {
    statement_type: Commands,
    row: Row,
}
#[derive(Debug)]
struct Row {
    id: u32,
    username: String,
    email: String,
}
const USERNAME_MAX_LENGTH: usize = 32;
const EMAIL_MAX_LENGTH: usize = 255;

impl Row {
    fn validate_username(&self) -> bool {
        self.username.len() <= USERNAME_MAX_LENGTH
    }
    fn validate_email(&self) -> bool {
        self.email.len() <= EMAIL_MAX_LENGTH
    }
}

fn serialize_row(row: &Row) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&row.id.to_le_bytes()[..]); // id
    buf.extend_from_slice(&row.username.len().to_le_bytes()[..]); // username length
    buf.extend_from_slice(&row.username.as_bytes()[..]); // username
    buf.extend_from_slice(&row.email.len().to_le_bytes()[..]); // email length
    buf.extend_from_slice(&row.email.as_bytes()[..]); // email
    buf
}

fn deserialize_row(buf: &[u8]) -> Row {
    let id = u32::from_le_bytes(buf[0..4].try_into().unwrap());
    let username_length = u32::from_le_bytes(buf[4..8].try_into().unwrap());
    let username = String::from_utf8(buf[8..8 + username_length as usize].to_vec()).unwrap();
    let email_length = u32::from_le_bytes(
        buf[8 + username_length as usize..12 + username_length as usize]
            .try_into()
            .unwrap(),
    );
    let email = String::from_utf8(
        buf[12 + username_length as usize..12 + username_length as usize + email_length as usize]
            .to_vec(),
    )
    .unwrap();
    Row {
        id,
        username: username,
        email: email,
    }
}
fn parse_command(input: &str, statement: &mut Statement) -> CommandsSuccess {
    let keys: Vec<&str> = input.split_whitespace().collect();
    if input.starts_with("insert") == true {
        let row = Row {
            id: keys[1].parse::<u32>().unwrap(),
            username: String::from(keys[2]),
            email: String::from(keys[3]),
        };

        if !row.validate_username() && !row.validate_email() {
            println!("Please enter a valid data.");
            return CommandsSuccess::Error;
        }
        statement.statement_type = Commands::Insert;
        statement.row = row;
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
            let row = serialize_row(&statement.row);
            println!("Inserted {} rows", row.len());
            println!("Serialized row: {:?}", row);
            let deserialized = deserialize_row(&row);
            println!("Deserialized row: {:?}", deserialized);
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
            row: Row {
                id: 0,
                username: String::new(),
                email: String::new(),
            },
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
                CommandsSuccess::Error => {
                    println!("Error: {}", input);
                    break;
                }
            }
        }
    }
}
