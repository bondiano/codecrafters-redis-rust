use bytes::BytesMut;
use itertools::Itertools;

use crate::storage::Storage;

#[derive(Debug)]
enum CommandResult {
    Empty,
    Ok(String),
    Error(String),
}

#[derive(Debug)]
enum Command {
    Ping,
    Set(String, String, Option<u128>),
    Get(String),
    Error(String),
    Echo(String),
}

fn parse_command(command: &[u8]) -> Command {
    let command = String::from_utf8_lossy(command).trim().to_lowercase();
    let command_line = command.split("\r\n");

    let args_count = command_line.clone().next().unwrap_or("0");
    let args_count = args_count.replace('*', "").parse::<usize>().unwrap();

    let command = command_line.clone().nth(2).unwrap_or("");
    let mut args = String::new();

    if args_count > 0 {
        args = command_line
            .skip(3)
            .zip(1..(args_count * 2))
            .filter(|(_x, i)| i % 2 == 0)
            .map(|(x, _)| x)
            .collect_vec()
            .join(" ");
    }

    match command {
        "ping" => Command::Ping,
        "echo" => Command::Echo(args),
        "set" => {
            let mut args = args.split(" ");
            let key = args.next().unwrap_or("").to_string();
            let value = args.next().unwrap_or("").to_string();
            let ttl_type = args.next().unwrap_or("").to_string();
            let ttl = if ttl_type == "ex" || ttl_type == "px" {
                let ttl = args.next().unwrap_or("").to_string();

                if ttl_type == "ex" {
                    Some(ttl.parse::<u128>().unwrap() * 1000)
                } else {
                    Some(ttl.parse::<u128>().unwrap())
                }
            } else {
                None
            };

            Command::Set(key, value, ttl)
        }
        "get" => {
            let mut args = args.split(" ");
            let key = args.next().unwrap_or("").to_string();
            Command::Get(key)
        }
        _ => Command::Error(format!("unknown command {} {}", command, args)),
    }
}

fn execute_command(command: Command, storage: &mut Storage) -> CommandResult {
    match command {
        Command::Get(key) => {
            if let Some(value) = storage.get(&key) {
                return CommandResult::Ok(value);
            }

            CommandResult::Empty
        }
        Command::Set(key, value, ttl) => {
            storage.set(&key, &value, ttl);

            CommandResult::Ok("OK".to_string())
        }
        Command::Ping => CommandResult::Ok("PONG".to_string()),
        Command::Echo(value) => CommandResult::Ok(value),
        Command::Error(error) => CommandResult::Error(error),
    }
}

fn format_result(result: &CommandResult) -> BytesMut {
    let result = match result {
        CommandResult::Ok(value) => format!("+{}\r\n", value),
        CommandResult::Empty => "$-1\r\n".to_string(),
        CommandResult::Error(error) => format!("-ERR: {}\r\n", error),
    };

    BytesMut::from(result.as_bytes())
}

pub fn handle_command(command: &BytesMut, storage: &mut Storage) -> BytesMut {
    let command = parse_command(command);
    let result = execute_command(command, storage);

    format_result(&result)
}

#[cfg(test)]
mod command_tests {
    use crate::storage;

    use super::*;

    #[test]
    fn test_ping_command() {
        let command = BytesMut::from(&b"*1\r\n$4\r\nping\r\n"[..]);
        let mut storage = storage::Storage::new();

        assert_eq!(
            handle_command(&command, &mut storage),
            BytesMut::from(&b"+PONG\r\n"[..])
        );
    }

    #[test]
    fn test_echo_command() {
        let command = BytesMut::from(&b"*2\r\n$4\r\necho\r\n$5\r\nhello hello\r\n"[..]);
        let mut storage = storage::Storage::new();

        assert_eq!(
            handle_command(&command, &mut storage),
            BytesMut::from(&b"+hello hello\r\n"[..])
        );
    }

    #[test]
    fn test_unknown_command() {
        let command = BytesMut::from(&b"*2\r\n$4\r\ntest\r\n$5\r\nhello world\r\n"[..]);
        let mut storage = storage::Storage::new();

        assert_eq!(
            handle_command(&command, &mut storage),
            BytesMut::from(&b"-ERR: unknown command test hello world\r\n"[..])
        );
    }

    #[test]
    fn test_set_get_commands() {
        let set_command = BytesMut::from(&b"*3\r\n$3\r\nset\r\n$3\r\nkey\r\n$5\r\nvalue\r\n"[..]);
        let get_command = BytesMut::from(&b"*2\r\n$3\r\nget\r\n$3\r\nkey\r\n"[..]);
        let mut storage = storage::Storage::new();

        handle_command(&set_command, &mut storage);

        assert_eq!(
            handle_command(&get_command, &mut storage),
            BytesMut::from(&b"+value\r\n"[..])
        );
    }

    #[test]
    fn test_expire_set() {
        let set_command = BytesMut::from(
            &b"*5\r\n$3\r\nset\r\n$3\r\nkey\r\n$5\r\nvalue\r\n$2\r\npx\r\n$100\r\n1\r\n"[..],
        );
        let get_command = BytesMut::from(&b"*2\r\n$3\r\nget\r\n$3\r\nkey\r\n"[..]);
        let mut storage = storage::Storage::new();

        handle_command(&set_command, &mut storage);

        assert_eq!(
            handle_command(&get_command, &mut storage),
            BytesMut::from(&b"+value\r\n"[..])
        );

        std::thread::sleep(std::time::Duration::from_millis(100));

        assert_eq!(
            handle_command(&get_command, &mut storage),
            BytesMut::from(&b"$-1\r\n"[..])
        );
    }
}
