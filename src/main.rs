use std::{collections::HashMap, net::TcpListener, io::{BufReader, BufWriter, Write}};
use anyhow;

mod command;
use command::Parser;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    String(Vec<u8>),
    List(Vec<Vec<u8>>),
    Number(i64),
    Nil,
}

fn execute_command(state: &mut HashMap<Vec<u8>, Value>, cmd: Vec<Vec<u8>>) -> anyhow::Result<Value> {
    println!("Command: {:?}", cmd);
    let value = match cmd[0].as_slice() {
        b"DEL" => {
            let removed = cmd[1..].iter().filter(|&key| state.remove(key).is_some()).count();
            Value::Number(removed as _)
        }
        b"GET" => {
            let [_, key] = cmd.try_into().map_err(|_| anyhow::anyhow!("expected GET key"))?;
            match state.get(&key) {
                Some(Value::String(v)) => Value::String(v.clone()),
                Some(_) => anyhow::bail!("GET on non-string value"),
                None => Value::Nil,
            }
        }
        b"SET" => {
            let [_, key, value] = cmd.try_into().map_err(|_| anyhow::anyhow!("expected SET key value"))?;
            state.insert(key, Value::String(value));
            Value::String(b"OK".to_vec())
        }
        b"COMMAND" => {
            // TODO: Implement this somehow
            Value::List(vec![])
        }
        _ => anyhow::bail!("Unrecognized command: {:?}", cmd[0]),
    };
    Ok(value)
}

fn write_response(writer: &mut impl Write, res: anyhow::Result<Value>) -> anyhow::Result<()> {
    match res {
        Ok(Value::String(value)) => {
            writer.write_all(b"+")?;
            writer.write_all(&value)?;
            writer.write_all(b"\r\n")?;
        }
        Ok(Value::List(value)) => {
            write!(writer, "*{}\r\n", value.len())?;
            for v in &value {
                write!(writer, "${}\r\n", v.len())?;
                writer.write_all(v)?;
                write!(writer, "\r\n")?;
            }
        }
        Ok(Value::Number(value)) => write!(writer, ":{value}\r\n")?,
        Ok(Value::Nil) => write!(writer, "$-1\r\n")?,
        Err(e) => write!(writer, "-ERR {e}\r\n")?,
    }
    writer.flush()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", 8888))?;
    let mut state = HashMap::new();
    for stream in listener.incoming() {
        let stream = stream?;
        let mut parser = Parser::new(BufReader::new(stream.try_clone()?));
        let mut writer = BufWriter::new(stream);
        loop {
            let r = match parser.read_string_array() {
                Ok(cmd) => execute_command(&mut state, cmd),
                Err(e) => Err(e),
            };
            if let Err(e) = write_response(&mut writer, r) {
                println!("Client error: {e}");
                break;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! exec_cmd {
        ($state:expr, $($cmd:expr),+) => {
            execute_command(&mut $state, vec![$($cmd.as_bytes().to_vec(),)+]).unwrap()
        }
    }

    #[test]
    fn test_get_set() {
        let mut state = HashMap::new();
        assert_eq!(exec_cmd!(state, "GET", "a"), Value::Nil);

        assert_eq!(exec_cmd!(state, "SET", "a", "b"), Value::String(b"OK".to_vec()));
        assert_eq!(exec_cmd!(state, "GET", "a"), Value::String(b"b".to_vec()));

        assert_eq!(exec_cmd!(state, "SET", "a", "c"), Value::String(b"OK".to_vec()));
        assert_eq!(exec_cmd!(state, "GET", "a"), Value::String(b"c".to_vec()));

        assert_eq!(exec_cmd!(state, "DEL", "a"), Value::Number(1));
        assert_eq!(exec_cmd!(state, "DEL", "b"), Value::Number(0));
        assert_eq!(exec_cmd!(state, "DEL", "a", "b", "c"), Value::Number(0));
    }
}
