use std::collections::{HashMap, HashSet};
use std::io::Write;

mod cmd_parser;
mod commands;
mod sorted_set;
use sorted_set::SortedSet;
pub use cmd_parser::Command;
pub use commands::COMMANDS;

pub type ByteString = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    String(ByteString),
    Array(Vec<ByteString>),
    Hash(HashMap<ByteString, ByteString>),
    Set(HashSet<ByteString>),
    ZSet(SortedSet<ByteString, i64>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    SimpleString(ByteString),
    BulkString(ByteString),
    Number(i64),
    Array(Vec<Response>),
    Nil,
}

impl Response {
    pub fn string_array(strings: impl IntoIterator<Item=ByteString>) -> Self {
        Self::Array(strings.into_iter().map(Response::BulkString).collect())
    }
}

impl Default for Response {
    fn default() -> Self { Self::Nil }
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

    pub fn get_array(&mut self, key: &[u8]) -> anyhow::Result<Option<&mut Vec<ByteString>>> {
        match self.get(key) {
            Some(Value::Array(v)) => Ok(Some(v)),
            Some(_) => anyhow::bail!("expected array value"),
            None => Ok(None)
        }
    }

    pub fn get_hash(&mut self, key: &[u8]) -> anyhow::Result<Option<&mut HashMap<ByteString, ByteString>>> {
        match self.get(key) {
            Some(Value::Hash(v)) => Ok(Some(v)),
            Some(_) => anyhow::bail!("expected hash value"),
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

    pub fn get_zset(&mut self, key: &[u8]) -> anyhow::Result<Option<&mut SortedSet<ByteString, i64>>> {
        match self.get(key) {
            Some(Value::ZSet(v)) => Ok(Some(v)),
            Some(_) => anyhow::bail!("expected zset value"),
            None => Ok(None)
        }
    }

    pub fn get_or_insert_str(&mut self, key: Vec<u8>) -> anyhow::Result<&mut ByteString> {
        let v = self.state.entry(key).or_insert_with(|| Value::String(Vec::new()));
        match v {
            Value::String(v) => Ok(v),
            _ => anyhow::bail!("expected string value"),
        }
    }

    pub fn get_or_insert_array(&mut self, key: Vec<u8>) -> anyhow::Result<&mut Vec<ByteString>> {
        let v = self.state.entry(key).or_insert_with(|| Value::Array(Vec::new()));
        match v {
            Value::Array(v) => Ok(v),
            _ => anyhow::bail!("expected array value"),
        }
    }

    pub fn get_or_insert_hash(&mut self, key: Vec<u8>) -> anyhow::Result<&mut HashMap<ByteString, ByteString>> {
        let v = self.state.entry(key).or_insert_with(|| Value::Hash(HashMap::new()));
        match v {
            Value::Hash(v) => Ok(v),
            _ => anyhow::bail!("expected hash value"),
        }
    }

    pub fn get_or_insert_set(&mut self, key: Vec<u8>) -> anyhow::Result<&mut HashSet<ByteString>> {
        let v = self.state.entry(key).or_insert_with(|| Value::Set(HashSet::new()));
        match v {
            Value::Set(v) => Ok(v),
            _ => anyhow::bail!("expected set value"),
        }
    }

    pub fn get_or_insert_zset(&mut self, key: Vec<u8>) -> anyhow::Result<&mut SortedSet<ByteString, i64>> {
        let v = self.state.entry(key).or_insert_with(|| Value::ZSet(SortedSet::new()));
        match v {
            Value::ZSet(v) => Ok(v),
            _ => anyhow::bail!("expected zset value"),
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
        Some((command, _)) => command(db, cmd),
        None => anyhow::bail!("Unrecognized command: {:?}", cmd.cmd()),
    }
}

pub fn write_response(writer: &mut impl Write, res: Response) -> anyhow::Result<()> {
    match res {
        Response::SimpleString(value) => {
            writer.write_all(b"+")?;
            writer.write_all(&value)?;
            writer.write_all(b"\r\n")?;
        }
        Response::BulkString(value) => {
            write!(writer, "${}\r\n", value.len())?;
            writer.write_all(&value)?;
            writer.write_all(b"\r\n")?;
        }
        Response::Number(value) => {
            write!(writer, ":{value}\r\n")?;
        },
        Response::Array(value) => {
            write!(writer, "*{}\r\n", value.len())?;
            for v in value {
                write_response(writer, v)?;
            }
        }
        Response::Nil => write!(writer, "$-1\r\n")?,
    }
    Ok(())
}

#[cfg(test)]
mod test_utils;
