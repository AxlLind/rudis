use std::collections::{HashMap, HashSet};
use std::net::TcpListener;
use std::io::{BufReader, BufWriter, Write};
use anyhow;

mod command;
use command::{Parser, Command};

#[allow(unused)] // TODO: remove this
#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    String(Vec<u8>),
    List(Vec<Vec<u8>>),
    Hash(HashMap<Vec<u8>, Vec<u8>>),
    Set(HashSet<Vec<u8>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Response {
    String(Vec<u8>),
    List(Vec<Vec<u8>>),
    Number(i64),
    Nil,
}

struct Database {
    state: HashMap<Vec<u8>, Value>
}

impl Database {
    fn new() -> Self {
        Self { state: HashMap::new() }
    }

    fn get(&mut self, key: &[u8]) -> Option<&mut Value> {
        self.state.get_mut(key)
    }

    fn get_str(&mut self, key: &[u8]) -> anyhow::Result<Option<&mut Vec<u8>>> {
        match self.get(key) {
            Some(Value::String(s)) => Ok(Some(s)),
            Some(_) => anyhow::bail!("expected string value"),
            None => Ok(None)
        }
    }

    fn get_list(&mut self, key: &[u8]) -> anyhow::Result<Option<&mut Vec<Vec<u8>>>> {
        match self.get(key) {
            Some(Value::List(v)) => Ok(Some(v)),
            Some(_) => anyhow::bail!("expected list value"),
            None => Ok(None)
        }
    }

    fn set(&mut self, key: Vec<u8>, value: Value) -> Option<Value> {
        self.state.insert(key, value)
    }

    fn del(&mut self, key: &[u8]) -> Option<Value> {
        self.state.remove(key)
    }

    fn is_set(&self, key: &[u8]) -> bool {
        self.state.contains_key(key)
    }
}

pub fn escape_bytes(bytes: &[u8]) -> String {
    bytes.iter().flat_map(|&b| std::ascii::escape_default(b)).map(|b| b as char).collect()
}

fn int_from_bytes(bytes: &[u8]) -> anyhow::Result<i64> {
    std::str::from_utf8(bytes)
        .map_err(|_| anyhow::anyhow!("tried to parse number, got non-utf8 value"))?
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("tried to parse number, got non-numeric value"))
}

fn incr_by(db: &mut Database, key: Vec<u8>, step: i64) -> anyhow::Result<Response> {
    let val = step + match db.get_str(&key)? {
        Some(v) => int_from_bytes(v)?,
        None => 0,
    };
    db.set(key, Value::String(val.to_string().into_bytes()));
    Ok(Response::Number(val))
}

fn execute_command(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    println!("Got command: {}", cmd);
    let value = match cmd.cmd() {
        b"APPEND" => {
            let (key, value) = cmd.parse_args::<(Vec<u8>, Vec<u8>)>()?;
            let len = match db.get_str(&key)? {
                Some(v) => {
                    v.extend(value);
                    v.len()
                }
                None => {
                    let len = value.len();
                    db.set(key, Value::String(value));
                    len
                }
            };
            Response::Number(len as _)
        }
        b"COPY" => {
            let (src, dst) = cmd.parse_args::<(Vec<u8>, Vec<u8>)>()?;
            match db.get(&src) {
                Some(v) => {
                    let copy = v.clone();
                    db.set(dst, copy);
                    Response::Number(1)
                }
                None => Response::Number(0),
            }
        }
        b"DECR" => {
            let key = cmd.parse_args::<Vec<u8>>()?;
            incr_by(db, key, -1)?
        }
        b"DECRBY" => {
            let (key, step) = cmd.parse_args::<(Vec<u8>, i64)>()?;
            incr_by(db, key, -step)?
        }
        b"DEL" => {
            let keys = cmd.parse_args::<Vec<Vec<u8>>>()?;
            anyhow::ensure!(!keys.is_empty(), "expected DEL key [key ...]");
            Response::Number(keys.iter().filter(|&key| db.del(key).is_some()).count() as _)
        }
        b"EXISTS" => {
            let keys = cmd.parse_args::<Vec<Vec<u8>>>()?;
            anyhow::ensure!(!keys.is_empty(), "expected EXISTS key [key ...]");
            Response::Number(keys.iter().filter(|&key| db.is_set(key)).count() as _)
        }
        b"GET" => {
            let key = cmd.parse_args::<Vec<u8>>()?;
            match db.get_str(&key)? {
                Some(s) => Response::String(s.clone()),
                None => Response::Nil,
            }
        }
        b"GETDEL" => {
            let key = cmd.parse_args::<Vec<u8>>()?;
            match db.get_str(&key)? {
                Some(s) => {
                    let val = Response::String(std::mem::take(s));
                    db.del(&key);
                    val
                }
                None => Response::Nil,
            }
        }
        b"GETSET" => {
            let (key, value) = cmd.parse_args::<(Vec<u8>, Vec<u8>)>()?;
            match db.get_str(&key)? {
                Some(s) => Response::String(std::mem::replace(s, value)),
                None => Response::Nil,
            }
        }
        b"INCR" => {
            let key = cmd.parse_args::<Vec<u8>>()?;
            incr_by(db, key, 1)?
        }
        b"INCRBY" => {
            let (key, step) = cmd.parse_args::<(Vec<u8>, i64)>()?;
            incr_by(db, key, step)?
        }
        b"LPUSH" => {
            let (key, elements) = cmd.parse_args::<(Vec<u8>, Vec<Vec<u8>>)>()?;
            anyhow::ensure!(!elements.is_empty(), "expected LPUSH key element [element ...]");
            match db.get_list(&key)? {
                Some(list) => {
                    for (i, e) in elements.into_iter().enumerate() {
                        list.insert(i, e);
                    }
                    Response::Number(list.len() as _)
                }
                None => {
                    let len = elements.len();
                    db.set(key, Value::List(elements));
                    Response::Number(len as _)
                }
            }
        }
        b"LRANGE" => {
            let (key, start, stop) = cmd.parse_args::<(Vec<u8>, i64, i64)>()?;
            match db.get_list(&key)? {
                Some(list) => {
                    let start = if start < 0 {list.len() - 2 - start as usize} else {start as usize};
                    let stop = if stop < 0 {list.len() - 2 - stop as usize} else {stop as usize};
                    // TODO: Implement more correct index handling here
                    Response::List(list[start..=stop].iter().cloned().collect())
                }
                None => Response::List(Vec::new()),
            }
        }
        b"RENAME" => {
            let (key, newkey) = cmd.parse_args::<(Vec<u8>, Vec<u8>)>()?;
            let val = db.del(&key).ok_or(anyhow::anyhow!("key does not exist"))?;
            db.set(newkey, val);
            Response::String(b"OK".to_vec())
        }
        b"SET" => {
            let (key, value) = cmd.parse_args::<(Vec<u8>, Vec<u8>)>()?;
            db.set(key, Value::String(value));
            Response::String(b"OK".to_vec())
        }
        b"STRLEN" => {
            let key = cmd.parse_args::<Vec<u8>>()?;
            match db.get_str(&key)? {
                Some(s) => Response::Number(s.len() as _),
                None => Response::Number(0),
            }
        }
        b"TYPE" => {
            let key = cmd.parse_args::<Vec<u8>>()?;
            let t: &[u8] = match db.get(&key) {
                Some(Value::String(_)) => b"string",
                Some(Value::List(_)) => b"list",
                Some(Value::Hash(_)) => b"hash",
                Some(Value::Set(_)) => b"set",
                None => b"none",
            };
            Response::String(t.to_vec())
        }
        b"COMMAND" => {
            // TODO: Implement this somehow
            Response::List(vec![])
        }
        _ => anyhow::bail!("Unrecognized command: {:?}", escape_bytes(cmd.cmd())),
    };
    Ok(value)
}

fn write_response(writer: &mut impl Write, res: anyhow::Result<Response>) -> anyhow::Result<()> {
    match res {
        Ok(Response::String(value)) => {
            writer.write_all(b"+")?;
            writer.write_all(&value)?;
            writer.write_all(b"\r\n")?;
        }
        Ok(Response::List(value)) => {
            write!(writer, "*{}\r\n", value.len())?;
            for v in &value {
                write!(writer, "${}\r\n", v.len())?;
                writer.write_all(v)?;
                write!(writer, "\r\n")?;
            }
        }
        Ok(Response::Number(value)) => write!(writer, ":{value}\r\n")?,
        Ok(Response::Nil) => write!(writer, "$-1\r\n")?,
        Err(e) => write!(writer, "-ERR {e}\r\n")?,
    }
    writer.flush()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", 8888))?;
    let mut db = Database::new();
    for stream in listener.incoming() {
        let stream = stream?;
        let mut parser = Parser::new(BufReader::new(stream.try_clone()?));
        let mut writer = BufWriter::new(stream);
        loop {
            let r = match parser.read_command() {
                Ok(cmd) => execute_command(&mut db, cmd),
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
        ($db:expr, $($cmd:expr),+) => {
            execute_command(&mut $db, Command::new(vec![$($cmd.as_bytes().to_vec(),)+])).unwrap()
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
