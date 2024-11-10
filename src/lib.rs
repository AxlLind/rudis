use std::collections::{HashMap, HashSet};
use std::io::Write;

mod command;
mod commands;
pub use command::{Parser, Command};
use commands::CommandInfo;
pub use commands::COMMANDS;

pub type ByteString = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    String(ByteString),
    Array(Vec<ByteString>),
    Hash(HashMap<ByteString, ByteString>),
    Set(HashSet<ByteString>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    String(ByteString),
    Number(i64),
    Array(Vec<ByteString>),
    CommandList(Vec<&'static CommandInfo>),
    Nil,
}

#[derive(Default)]
pub struct Database {
    state: HashMap<ByteString, Value>
}

impl Database {
    pub fn get(&mut self, key: &[u8]) -> Option<&mut Value> {
        self.state.get_mut(key)
    }

    pub fn get_str(&mut self, key: &[u8]) -> anyhow::Result<Option<&mut ByteString>> {
        match self.get(key) {
            Some(Value::String(s)) => Ok(Some(s)),
            Some(_) => anyhow::bail!("expected string value"),
            None => Ok(None)
        }
    }

    pub fn get_list(&mut self, key: &[u8]) -> anyhow::Result<Option<&mut Vec<ByteString>>> {
        match self.get(key) {
            Some(Value::Array(v)) => Ok(Some(v)),
            Some(_) => anyhow::bail!("expected list value"),
            None => Ok(None)
        }
    }

    pub fn get_set(&mut self, key: &[u8]) -> anyhow::Result<Option<&mut HashSet<ByteString>>> {
        match self.get(key) {
            Some(Value::Set(v)) => Ok(Some(v)),
            Some(_) => anyhow::bail!("expected set value"),
            None => Ok(None)
        }
    }

    pub fn set(&mut self, key: ByteString, value: Value) -> Option<Value> {
        self.state.insert(key, value)
    }

    pub fn del(&mut self, key: &[u8]) -> Option<Value> {
        self.state.remove(key)
    }

    pub fn contains(&self, key: &[u8]) -> bool {
        self.state.contains_key(key)
    }

    pub fn clear(&mut self) {
        self.state.clear();
    }
}

pub fn escape_bytes(bytes: &[u8]) -> String {
    bytes.iter().flat_map(|&b| std::ascii::escape_default(b)).map(|b| b as char).collect()
}

pub fn execute_command(db: &mut Database, cmd: Command) -> anyhow::Result<Response> {
    match COMMANDS.get(cmd.cmd().as_bytes()) {
        Some(command) => command.run(db, cmd),
        None => anyhow::bail!("Unrecognized command: {:?}", cmd.cmd()),
    }
}

pub fn write_response(writer: &mut impl Write, res: Response) -> anyhow::Result<()> {
    match res {
        Response::String(value) => {
            writer.write_all(b"+")?;
            writer.write_all(&value)?;
            writer.write_all(b"\r\n")?;
        }
        Response::Number(value) => {
            write!(writer, ":{value}\r\n")?;
        },
        Response::Array(value) => {
            write!(writer, "*{}\r\n", value.len())?;
            for v in &value {
                write!(writer, "${}\r\n", v.len())?;
                writer.write_all(v)?;
                write!(writer, "\r\n")?;
            }
        }
        Response::CommandList(mut value) => {
            write!(writer, "*{}\r\n", value.len())?;
            for v in &mut value {
                write!(writer, "${}\r\n", v.name.len())?;
                writer.write_all(v.name)?;
                write_response(writer, Response::Number(v.arity))?;
                write_response(writer, Response::Array(v.flags.iter().map(|s| s.to_vec()).collect()))?;
                write_response(writer, Response::Number(v.first_key))?;
                write_response(writer, Response::Number(v.last_key))?;
                write_response(writer, Response::Number(v.step))?;
            }
        },
        Response::Nil => write!(writer, "$-1\r\n")?,
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! exec_cmd {
        ($db:expr, $($cmd:expr),+) => {
            execute_command(&mut $db, Command::new(vec![$($cmd.as_bytes().to_vec(),)+]).unwrap()).unwrap()
        }
    }

    #[test]
    fn test_get_set() {
        let mut db = Database::default();
        assert_eq!(exec_cmd!(db, "GET", "a"), Response::Nil);

        assert_eq!(exec_cmd!(db, "SET", "a", "b"), Response::String(b"OK".to_vec()));
        assert_eq!(exec_cmd!(db, "GET", "a"), Response::String(b"b".to_vec()));

        assert_eq!(exec_cmd!(db, "SET", "a", "c"), Response::String(b"OK".to_vec()));
        assert_eq!(exec_cmd!(db, "GET", "a"), Response::String(b"c".to_vec()));

        assert_eq!(exec_cmd!(db, "DEL", "a"), Response::Number(1));
        assert_eq!(exec_cmd!(db, "DEL", "b"), Response::Number(0));
        assert_eq!(exec_cmd!(db, "DEL", "a", "b", "c"), Response::Number(0));
    }
}
