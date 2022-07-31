use bytes::{BytesMut};
use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub enum CommandError {
    ParseError(String),
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Ping,
    Echo(String),
}

fn parse_command(command: &[u8]) -> Result<Command, CommandError> {
    let command = String::from_utf8_lossy(command);
    let command = command.trim();
    let command = command.to_lowercase();

    let command_line = command.split("\r\n");

    let args_count = match command_line.clone().nth(0) {
        Some(count) => count,
        None => return Err(CommandError::ParseError(format!("Invalid command: {}", command)))
    };

    let command = match command_line.clone().nth(2) {
        Some(command) => command,
        None => return Err(CommandError::ParseError(format!("Invalid command: {}", command)))
    };

    let args_count = args_count.replace("*", "").parse::<usize>().unwrap();
    println!("args_count: {}", args_count);

    match command {
        "ping" => Ok(Command::Ping),
        "echo" => {
            let arg = command_line.skip(3)
            .zip(1..(args_count * 2)).filter(|(_x, i)| i % 2 == 0)
            .map(|(x, _)| x).collect_vec().join(" ");

            Ok(Command::Echo(arg.to_string()))
        },
        _ => Err(CommandError::ParseError(format!("Unknown command: {}", command))),
    }
}

fn execute_command(command: &Command) -> Result<BytesMut, CommandError> {
    let result = match command {
        Command::Ping => "PONG",
        Command::Echo(arg) => arg,
    };

    let result = format!("+{}\r\n", result);
    let result = result.as_bytes();

    Ok(BytesMut::from(result))
}

pub async fn handle_command(command: &BytesMut) -> Result<BytesMut, CommandError> {
    let command = parse_command(&command)?;

    let result = execute_command(&command)?;

    Ok(result)
}

#[cfg(test)]
mod command_tests {
    use super::*;

    #[tokio::test]
    async fn test_ping_command() {
        let command = BytesMut::from(&b"*1\r\n$4\r\nping\r\n"[..]);

        assert_eq!(handle_command(&command).await.unwrap(), BytesMut::from(&b"+PONG\r\n"[..]));
    }

    #[tokio::test]
    async fn test_echo_command() {
        let command = BytesMut::from(&b"*2\r\n$4\r\necho\r\n$5\r\nhello\r\n"[..]);

        assert_eq!(handle_command(&command).await.unwrap(), BytesMut::from(&b"+hello\r\n"[..]));
    }
}
