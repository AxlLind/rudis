use std::collections::{HashMap, HashSet};
use anyhow;

mod command;
mod commands;
pub use command::{Parser, Command};
pub use commands::COMMANDS;

pub type ByteString = Vec<u8>;

#[allow(unused)] // TODO: remove this
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    String(ByteString),
    List(Vec<ByteString>),
    Hash(HashMap<ByteString, ByteString>),
    Set(HashSet<ByteString>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    String(ByteString),
    List(Vec<ByteString>),
    Number(i64),
    Nil,
}

pub struct Database {
    state: HashMap<ByteString, Value>
}

impl Database {
    pub fn new() -> Self {
        Self { state: HashMap::new() }
    }

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
            Some(Value::List(v)) => Ok(Some(v)),
            Some(_) => anyhow::bail!("expected list value"),
            None => Ok(None)
        }
    }

    pub fn set(&mut self, key: ByteString, value: Value) -> Option<Value> {
        self.state.insert(key, value)
    }

    pub fn del(&mut self, key: &[u8]) -> Option<Value> {
        self.state.remove(key)
    }

    pub fn is_set(&self, key: &[u8]) -> bool {
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
    match COMMANDS.get(cmd.cmd()) {
        Some(command) => command.run(db, cmd),
        None => anyhow::bail!("Unrecognized command: {:?}", cmd.cmd()),
    }
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
        let mut db = Database::new();
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
